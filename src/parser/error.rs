/// What kind of invalid instruction was attempted to be parsed
#[derive(Copy, Clone, Debug)]
pub enum InstType {
    Stack,
    Heap,
    Io,
    ControlFlow,
    Arithmetic,
    Unknown,
}

/// What kind of parse error was found
#[derive(Debug)]
pub enum ErrorKind {
    /// The literal is too large to fit in an `i64`
    LiteralOverflow,
    /// The literal could not be parsed
    InvalidLiteral,
    /// An invalid sequence of spaces, tabs, and LFs was encountered
    InvalidInstruction(InstType),
    /// The program attempted to jump to a label that does not exist
    InvalidLabel,
    /// The program contains too many labels (max is `usize::MAX`)
    TooManyLabels,
    /// The source file ended before the program could be fully parsed
    UnexpectedEof,
}

#[derive(Debug)]
pub struct ParseError {
    line_no: usize,
    kind: ErrorKind,
}

impl ParseError {
    pub fn new(kind: ErrorKind, line_no: usize) -> Self {
        Self { kind, line_no }
    }

    /// Dumps the error to stdout
    pub fn print_error(&self) {
        print!("[Line {}] ", self.line_no);
        match self.kind {
            ErrorKind::LiteralOverflow => {
                println!("Literal too large to fit in an i64.");
            }
            ErrorKind::InvalidLiteral => {
                println!("Invalid literal.");
            }
            ErrorKind::InvalidInstruction(inst) => match inst {
                InstType::Stack => {
                    println!("Invalid stack manipulation instruction.");
                }
                InstType::Heap => {
                    println!("Invalid heap manipulation instruction.");
                }
                InstType::Io => {
                    println!("Invalid IO instruction.");
                }
                InstType::ControlFlow => {
                    println!("Invalid control flow instruction.");
                }
                InstType::Arithmetic => {
                    println!("Invalid arithmetic instruction.");
                }
                InstType::Unknown => println!("Invalid instruction prefix."),
            },
            ErrorKind::InvalidLabel => {
                println!("Invalid Label.");
            }
            ErrorKind::TooManyLabels => {
                println!("Program contains too many labels.");
            }
            ErrorKind::UnexpectedEof => {
                println!("Unexpected end of file.");
            }
        }
    }
}
