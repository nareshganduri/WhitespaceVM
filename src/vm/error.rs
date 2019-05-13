#[derive(Copy, Clone, Debug)]
pub enum RuntimeError {
    /// The program tried to divide by zero
    ZeroDivision,
    /// The program tried to access an invalid heap location
    InvalidHeapEntry,
    /// The program had some unspecified IO error
    IoError,
    /// The program could not parse the user's input as a valid number
    NumParseError,
    /// The program tried to pop the stack while it was empty
    StackUnderflow,
}

pub struct TraceEntry {
    line_no: usize,
    label: Option<usize>,
}

impl TraceEntry {
    pub fn new(line_no: usize, label: Option<usize>) -> Self {
        Self { line_no, label }
    }
}

pub struct Traceback {
    pub stack: Vec<TraceEntry>,
    pub reason: RuntimeError,
}

impl Traceback {
    /// Prints the traceback to stdout
    pub fn dump(&self) {
        println!("Stack traceback:");
        for entry in &self.stack {
            if let Some(label) = entry.label {
                println!("[Line {}] in subroutine #{}", entry.line_no, label);
            } else {
                println!("[Line {}] in main()", entry.line_no);
            }
        }

        match self.reason {
            RuntimeError::ZeroDivision => {
                println!("Error: Attempted to divide by zero");
            }
            RuntimeError::InvalidHeapEntry => {
                println!("Error: Attempted to access invalid heap entry");
            }
            RuntimeError::IoError => {
                println!("Error: An unexpected IO error occurred.");
            }
            RuntimeError::NumParseError => {
                println!("Error: Could not parse input as valid integer.");
            }
            RuntimeError::StackUnderflow => {
                println!("Error: The program stack underflowed.");
            }
        }
    }
}
