use self::error::{ErrorKind, InstType, ParseError};
use self::label_map::LabelMap;
use crate::program::{Instruction, Program};
use crate::token::{Token, Tokens};

mod error;
mod label_map;

/// A constant representing a dummy jump target that will be resolved later
const UNINITIALIZED_JUMP_TARGET: usize = 0;

type PResult<T> = Result<T, ParseError>;

pub struct Parser<'a> {
    tokens: Tokens<'a>,
    /// The line number of the previous token
    prev_line_no: usize,
    curr: Option<Token>,
    labels: LabelMap,
    program: Program,
}

impl<'a> Parser<'a> {
    pub fn new(source: &'a str) -> Self {
        let tokens = Tokens::new(source);

        Self {
            tokens,
            prev_line_no: 1,
            curr: None,
            labels: LabelMap::new(),
            program: Program::new(),
        }
    }

    fn error(&self, kind: ErrorKind) -> ParseError {
        let line_no = self.tokens.line_no();
        ParseError::new(kind, line_no)
    }

    /// Returns an error indicating the parser has encountered an invalid
    /// instruction
    fn invalid_inst(&self, inst_type: InstType) -> ParseError {
        self.error(ErrorKind::InvalidInstruction(inst_type))
    }

    fn get_next(&mut self) {
        self.prev_line_no = self.tokens.line_no();
        self.curr = self.tokens.next();
    }

    fn matches(&mut self, token: Token) -> bool {
        if self.curr == Some(token) {
            self.get_next();
            true
        } else {
            false
        }
    }

    /// Fetches the next two tokens from the token stream. Errors
    /// if there are not at least two more tokens available
    fn get_next_two(&mut self) -> PResult<(Token, Token)> {
        let first = self.curr.take();
        self.get_next();
        let second = self.curr.take();
        self.get_next();

        match (first, second) {
            (Some(first), Some(second)) => Ok((first, second)),
            _ => Err(self.error(ErrorKind::UnexpectedEof)),
        }
    }

    /// Adds an instruction to the current program
    fn emit(&mut self, inst: Instruction) {
        self.program.emit(inst, self.prev_line_no);
    }

    /// Reads a number literal from the source
    fn get_number(&mut self) -> PResult<i64> {
        let is_negative = if self.matches(Token::Space) {
            false
        } else if self.matches(Token::Tab) {
            true
        } else {
            return Err(self.error(ErrorKind::InvalidLiteral));
        };
        let mut num = 0i64;

        loop {
            if self.matches(Token::Space) {
                num = if let Some(x) = num.checked_shl(1) {
                    x
                } else {
                    return Err(self.error(ErrorKind::LiteralOverflow));
                };
            } else if self.matches(Token::Tab) {
                num = if let Some(x) = num.checked_shl(1) {
                    x
                } else {
                    return Err(self.error(ErrorKind::LiteralOverflow));
                };
                num |= 1;
            } else if self.matches(Token::Newline) {
                break;
            } else {
                return Err(self.error(ErrorKind::UnexpectedEof));
            }
        }

        if is_negative {
            num = -num;
        }

        Ok(num)
    }

    /// Reads a label from the source
    fn get_label(&mut self) -> PResult<usize> {
        let mut label = 0usize;

        loop {
            if self.matches(Token::Space) {
                label = if let Some(x) = label.checked_shl(1) {
                    x
                } else {
                    return Err(self.error(ErrorKind::TooManyLabels));
                };
            } else if self.matches(Token::Tab) {
                label = if let Some(x) = label.checked_shl(1) {
                    x
                } else {
                    return Err(self.error(ErrorKind::TooManyLabels));
                };
                label |= 1;
            } else if self.matches(Token::Newline) {
                break;
            } else {
                return Err(self.error(ErrorKind::UnexpectedEof));
            }
        }

        Ok(label)
    }

    fn get_stack_inst(&mut self) -> PResult<()> {
        if self.matches(Token::Space) {
            let num = self.get_number()?;
            let idx = self.program.add_const(num);

            let inst = Instruction::Push(idx);
            self.emit(inst);

            return Ok(());
        }

        let next_two = self.get_next_two()?;
        match next_two {
            (Token::Tab, Token::Space) => {
                let num = self.get_number()?;
                let inst = Instruction::Copy(num);
                self.emit(inst);
            }
            (Token::Tab, Token::Newline) => {
                let num = self.get_number()?;
                let inst = Instruction::Slide(num);
                self.emit(inst);
            }
            (Token::Newline, Token::Space) => {
                let inst = Instruction::Dup;
                self.emit(inst);
            }
            (Token::Newline, Token::Tab) => {
                let inst = Instruction::Swap;
                self.emit(inst);
            }
            (Token::Newline, Token::Newline) => {
                let inst = Instruction::Pop;
                self.emit(inst);
            }
            _ => return Err(self.invalid_inst(InstType::Stack)),
        }

        Ok(())
    }

    fn get_arith_inst(&mut self) -> PResult<()> {
        let next_two = self.get_next_two()?;
        match next_two {
            (Token::Space, Token::Space) => {
                let inst = Instruction::Add;
                self.emit(inst);
            }
            (Token::Space, Token::Tab) => {
                let inst = Instruction::Subtract;
                self.emit(inst);
            }
            (Token::Space, Token::Newline) => {
                let inst = Instruction::Multiply;
                self.emit(inst);
            }
            (Token::Tab, Token::Space) => {
                let inst = Instruction::Divide;
                self.emit(inst);
            }
            (Token::Tab, Token::Tab) => {
                let inst = Instruction::Modulo;
                self.emit(inst);
            }
            _ => return Err(self.invalid_inst(InstType::Arithmetic)),
        }

        Ok(())
    }

    fn get_heap_inst(&mut self) -> PResult<()> {
        if self.matches(Token::Space) {
            let inst = Instruction::Store;
            self.emit(inst);
        } else if self.matches(Token::Tab) {
            let inst = Instruction::Retrieve;
            self.emit(inst);
        } else {
            return Err(self.invalid_inst(InstType::Heap));
        }

        Ok(())
    }

    fn get_io_inst(&mut self) -> PResult<()> {
        let next_two = self.get_next_two()?;
        match next_two {
            (Token::Space, Token::Space) => {
                let inst = Instruction::OutputChar;
                self.emit(inst);
            }
            (Token::Space, Token::Tab) => {
                let inst = Instruction::OutputNum;
                self.emit(inst);
            }
            (Token::Tab, Token::Space) => {
                let inst = Instruction::ReadChar;
                self.emit(inst);
            }
            (Token::Tab, Token::Tab) => {
                let inst = Instruction::ReadNum;
                self.emit(inst);
            }
            _ => return Err(self.invalid_inst(InstType::Io)),
        }

        Ok(())
    }

    fn get_flow_inst(&mut self) -> PResult<()> {
        let next_two = self.get_next_two()?;
        match next_two {
            (Token::Space, Token::Space) => {
                let label = self.get_label()?;
                let pc = self.program.inst_count();

                self.labels.add_label(label, pc);
            }
            (Token::Space, Token::Tab) => {
                let label = self.get_label()?;
                let idx = self.program.inst_count();
                self.labels.add_inst(idx, label);

                let inst = Instruction::Call(UNINITIALIZED_JUMP_TARGET);
                self.emit(inst);
            }
            (Token::Space, Token::Newline) => {
                let label = self.get_label()?;
                let idx = self.program.inst_count();
                self.labels.add_inst(idx, label);

                let inst = Instruction::Jump(UNINITIALIZED_JUMP_TARGET);
                self.emit(inst);
            }
            (Token::Tab, Token::Space) => {
                let label = self.get_label()?;
                let idx = self.program.inst_count();
                self.labels.add_inst(idx, label);

                let inst = Instruction::JumpIfZero(UNINITIALIZED_JUMP_TARGET);
                self.emit(inst);
            }
            (Token::Tab, Token::Tab) => {
                let label = self.get_label()?;
                let idx = self.program.inst_count();
                self.labels.add_inst(idx, label);

                let inst = Instruction::JumpIfNeg(UNINITIALIZED_JUMP_TARGET);
                self.emit(inst);
            }
            (Token::Tab, Token::Newline) => {
                let inst = Instruction::Return;
                self.emit(inst);
            }
            (Token::Newline, Token::Newline) => {
                let inst = Instruction::End;
                self.emit(inst);
            }
            _ => return Err(self.invalid_inst(InstType::ControlFlow)),
        }

        Ok(())
    }

    /// Walk through the current program and resolve all the jump targets
    /// using the `LabelMap`
    fn patch_jumps(&mut self) -> PResult<()> {
        for (idx, label) in self.labels.iter_insts() {
            let inst = self.program.inst_at_mut(*idx);
            let pc = match self.labels.get_pc(*label) {
                Some(x) => x,
                None => {
                    let line_no = self.program.line_at(*idx);
                    return Err(ParseError::new(ErrorKind::InvalidLabel, line_no));
                }
            };

            let mut add_label = false;
            let new_inst = match inst {
                Instruction::Call(_) => {
                    add_label = true;
                    Instruction::Call(pc)
                }
                Instruction::Jump(_) => Instruction::Jump(pc),
                Instruction::JumpIfZero(_) => Instruction::JumpIfZero(pc),
                Instruction::JumpIfNeg(_) => Instruction::JumpIfNeg(pc),
                _ => unreachable!(),
            };

            *inst = new_inst;
            if add_label {
                self.program.add_sub_label(pc, *label);
            }
        }

        Ok(())
    }

    /// Parses the input
    pub fn parse(mut self) -> Result<Program, ParseError> {
        self.get_next();

        loop {
            if self.matches(Token::Space) {
                self.get_stack_inst()?;
            } else if self.matches(Token::Tab) {
                if self.matches(Token::Space) {
                    self.get_arith_inst()?;
                } else if self.matches(Token::Tab) {
                    self.get_heap_inst()?;
                } else if self.matches(Token::Newline) {
                    self.get_io_inst()?;
                } else {
                    return Err(self.invalid_inst(InstType::Unknown));
                }
            } else if self.matches(Token::Newline) {
                self.get_flow_inst()?;
            } else {
                break;
            }
        }
        self.patch_jumps()?;

        Ok(self.program)
    }
}
