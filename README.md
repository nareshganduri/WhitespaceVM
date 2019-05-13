# Whitespace VM

A [Whitespace][1] interpreter written in Rust.
 
The source code when run is compiled into bytecode and run on a simple
VM like many interpreted languages.

The translation is mostly verbatim with the exception of labels, which
are translated into their equivalent jump targets at compile time.

Furthermore, the VM is designed to give a full stack traceback on error,
unlike the reference implementation.

## Limitations
There are several limitations that prevent this implementation from
being fully compliant with the [reference implementation][ref] in Haskell.
The most notable is all integers are restricted in size to standard 64-bit
signed integers instead of arbitrary precision integers. This was done
mostly for simplicity's sake.

The stack traceback probably has some runtime cost associated with managing
the virtual call stack.
 
## Usage
Running the following should do the trick:
```
cargo run -- [file]
```

## Examples
The [examples][2] directory contains a few simple examples copied directly
from the reference implementation. They should all work; otherwise, something
has gone wrong...

[1]: https://en.wikipedia.org/wiki/Whitespace_(programming_language)
[2]: ./examples
[ref]: http://web.archive.org/web/20150717140342/http://compsoc.dur.ac.uk/whitespace/download.php