use std::env;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() == 1 {
        println!("Usage: wspace.exe [file]");
    } else {
        let filename = &args[1];
        whitespace_vm::run_file(filename);
    }
}
