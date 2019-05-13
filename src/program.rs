use std::collections::HashMap;

#[derive(Debug)]
pub enum Instruction {
    Push(usize),
    Dup,
    Copy(i64),
    Swap,
    Pop,
    Slide(i64),
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Store,
    Retrieve,
    Call(usize),
    Jump(usize),
    JumpIfZero(usize),
    JumpIfNeg(usize),
    Return,
    End,
    OutputChar,
    OutputNum,
    ReadChar,
    ReadNum,
}

#[derive(Debug)]
pub struct Program {
    instructions: Vec<Instruction>,
    line_nos: Vec<usize>,
    constants: Vec<i64>,
    /// A mapping between subroutine labels and their corresponding
    /// program counters
    sub_labels: HashMap<usize, usize>,
}

impl Program {
    pub fn new() -> Self {
        Self {
            instructions: vec![],
            line_nos: vec![],
            constants: vec![],
            sub_labels: HashMap::new(),
        }
    }

    /// Adds a new constant to the constant pool if it is not in the pool
    /// already and returns the index of the constant in the pool
    pub fn add_const(&mut self, constant: i64) -> usize {
        let idx = self.constants.iter().position(|x| *x == constant);
        match idx {
            Some(idx) => idx,
            None => {
                self.constants.push(constant);
                self.constants.len() - 1
            }
        }
    }

    /// Fetches the constant at the given index
    pub fn get_const(&self, idx: usize) -> i64 {
        self.constants[idx]
    }

    /// Fetches the subroutine label for the given program counter if
    /// it exists
    pub fn get_label(&self, pc: usize) -> Option<usize> {
        self.sub_labels.get(&pc).cloned()
    }

    /// Adds a subroutine label
    pub fn add_sub_label(&mut self, pc: usize, label: usize) {
        self.sub_labels.insert(pc, label);
    }

    /// Returns a reference to the instruction at `idx`
    pub fn inst_at(&self, idx: usize) -> &Instruction {
        &self.instructions[idx]
    }

    /// Returns a mutable reference to the instruction at `idx`
    pub fn inst_at_mut(&mut self, idx: usize) -> &mut Instruction {
        &mut self.instructions[idx]
    }

    /// Returns the source line number of the instruction at `idx`
    pub fn line_at(&self, idx: usize) -> usize {
        self.line_nos[idx]
    }

    /// Gets the number of instructions currently added to the program
    pub fn inst_count(&self) -> usize {
        self.instructions.len()
    }

    /// Adds a new instruction to the program with its corresponding line
    /// number in the source
    pub fn emit(&mut self, inst: Instruction, line_no: usize) {
        self.line_nos.push(line_no);
        self.instructions.push(inst);
    }
}
