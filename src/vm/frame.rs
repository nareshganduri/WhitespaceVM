pub struct CallFrame {
    /// The program counter for the given subroutine
    pub pc: usize,
    /// The current subroutine's label. If the current subroutine
    /// is at the top level (i.e. main), the label is `None`
    pub label: Option<usize>,
}

impl CallFrame {
    /// Constructs a new `CallFrame`
    pub fn new(pc: usize, label: usize) -> Self {
        Self {
            pc,
            label: Some(label),
        }
    }

    /// Constructs new `CallFrame` for holding the main function
    pub fn new_main() -> Self {
        Self { pc: 0, label: None }
    }
}
