use self::error::{RuntimeError, TraceEntry, Traceback};
use self::frame::CallFrame;
use crate::program::{Instruction, Program};
use std::collections::HashMap;
use std::io::{self, Read, Write};

mod error;
mod frame;

/// The result of running the VM on a given program
type VmResult<T> = Result<T, Traceback>;

/// The virtual machine running the program
pub struct Vm<'a> {
    stack: Vec<i64>,
    call_stack: Vec<CallFrame>,
    heap: HashMap<i64, i64>,
    program: &'a Program,
}

impl<'a> Vm<'a> {
    /// Constructs a new VM to run the given program
    pub fn new(program: &'a Program) -> Self {
        Self {
            stack: vec![],
            call_stack: vec![],
            heap: HashMap::new(),
            program,
        }
    }

    /// Raises a runtime error
    fn runtime_error(&self, reason: RuntimeError) -> Traceback {
        let mut stack = vec![];
        for frame in &self.call_stack {
            let CallFrame { pc, label } = frame;
            let line_no = self.program.line_at(*pc);
            let entry = TraceEntry::new(line_no, *label);
            stack.push(entry);
        }

        Traceback { stack, reason }
    }

    fn push(&mut self, value: i64) {
        self.stack.push(value);
    }

    fn pop(&mut self) -> VmResult<i64> {
        match self.stack.pop() {
            Some(x) => Ok(x),
            None => Err(self.runtime_error(RuntimeError::StackUnderflow)),
        }
    }

    fn peek(&self) -> VmResult<i64> {
        match self.stack.last() {
            Some(x) => Ok(*x),
            None => Err(self.runtime_error(RuntimeError::StackUnderflow)),
        }
    }

    fn current_frame(&mut self) -> &mut CallFrame {
        self.call_stack.last_mut().unwrap()
    }

    /// Runs the given `Program`
    pub fn run(mut self) -> VmResult<()> {
        let main_frame = CallFrame::new_main();
        self.call_stack.push(main_frame);

        loop {
            let pc = self.current_frame().pc;
            let inst = self.program.inst_at(pc);
            self.current_frame().pc += 1;

            match inst {
                Instruction::Push(idx) => {
                    let constant = self.program.get_const(*idx);
                    self.push(constant);
                }
                Instruction::Dup => {
                    let last = self.peek()?;
                    self.push(last);
                }
                Instruction::Copy(idx) => {
                    let idx = *idx as usize;
                    if self.stack.len() < idx {
                        return Err(self.runtime_error(RuntimeError::StackUnderflow));
                    }

                    let idx = self.stack.len() - 1 - idx;
                    let value = self.stack[idx];
                    self.push(value);
                }
                Instruction::Swap => {
                    if self.stack.len() < 2 {
                        return Err(self.runtime_error(RuntimeError::StackUnderflow));
                    }

                    let first = self.stack.len() - 1;
                    let second = first - 1;
                    self.stack.swap(first, second);
                }
                Instruction::Pop => {
                    self.pop()?;
                }
                Instruction::Slide(idx) => {
                    let idx = *idx as usize;
                    if self.stack.len() < idx + 1 {
                        return Err(self.runtime_error(RuntimeError::StackUnderflow));
                    }

                    let last = self.pop()?;
                    let new_len = self.stack.len() - idx;
                    self.stack.truncate(new_len);
                    self.push(last);
                }
                Instruction::Add => {
                    let right = self.pop()?;
                    let left = self.pop()?;
                    let result = left + right;
                    self.push(result);
                }
                Instruction::Subtract => {
                    let right = self.pop()?;
                    let left = self.pop()?;
                    let result = left - right;
                    self.push(result);
                }
                Instruction::Multiply => {
                    let right = self.pop()?;
                    let left = self.pop()?;
                    let result = left * right;
                    self.push(result);
                }
                Instruction::Divide => {
                    let right = self.pop()?;
                    if right == 0 {
                        return Err(self.runtime_error(RuntimeError::ZeroDivision));
                    }

                    let left = self.pop()?;
                    let result = left / right;
                    self.push(result);
                }
                Instruction::Modulo => {
                    let right = self.pop()?;
                    if right == 0 {
                        return Err(self.runtime_error(RuntimeError::ZeroDivision));
                    }

                    let left = self.pop()?;
                    let result = left % right;
                    self.push(result);
                }
                Instruction::Store => {
                    let value = self.pop()?;
                    let addr = self.pop()?;
                    self.heap.insert(addr, value);
                }
                Instruction::Retrieve => {
                    let addr = self.pop()?;
                    let value = match self.heap.get(&addr) {
                        Some(x) => *x,
                        None => return Err(self.runtime_error(RuntimeError::InvalidHeapEntry)),
                    };
                    self.push(value);
                }
                Instruction::Call(pc) => {
                    let label = self.program.get_label(*pc).unwrap();
                    let frame = CallFrame::new(*pc, label);
                    self.call_stack.push(frame);
                }
                Instruction::Jump(pc) => {
                    self.current_frame().pc = *pc;
                }
                Instruction::JumpIfZero(pc) => {
                    let cond = self.pop()?;
                    if cond == 0 {
                        self.current_frame().pc = *pc;
                    }
                }
                Instruction::JumpIfNeg(pc) => {
                    let cond = self.pop()?;
                    if cond.is_negative() {
                        self.current_frame().pc = *pc;
                    }
                }
                Instruction::Return => {
                    self.call_stack.pop();
                    if self.call_stack.is_empty() {
                        return Ok(());
                    }
                }
                Instruction::End => {
                    return Ok(());
                }
                Instruction::OutputChar => {
                    let c = self.pop()? as u8 as char;
                    print!("{}", c);
                    if io::stdout().flush().is_err() {
                        return Err(self.runtime_error(RuntimeError::IoError));
                    }
                }
                Instruction::OutputNum => {
                    let num = self.pop()?;
                    print!("{}", num);
                    if io::stdout().flush().is_err() {
                        return Err(self.runtime_error(RuntimeError::IoError));
                    }
                }
                Instruction::ReadChar => {
                    let addr = self.pop()?;

                    let mut c = [0u8];
                    if io::stdin().read_exact(&mut c).is_err() {
                        return Err(self.runtime_error(RuntimeError::IoError));
                    }
                    self.heap.insert(addr, i64::from(c[0]));
                }
                Instruction::ReadNum => {
                    let addr = self.pop()?;

                    let mut num = String::new();
                    if io::stdin().read_line(&mut num).is_err() {
                        return Err(self.runtime_error(RuntimeError::IoError));
                    }
                    let len = num.trim_end().len();
                    num.truncate(len);
                    let num: i64 = match num.parse() {
                        Ok(x) => x,
                        Err(_) => return Err(self.runtime_error(RuntimeError::NumParseError)),
                    };

                    self.heap.insert(addr, num);
                }
            }
        }
    }
}
