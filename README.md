# brainfrick
Rust implementation of Brainfuck
```
USAGE:
    brainfrick [OPTIONS] [path]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -i, --input <input>    Input given to program [default: ]
    -s, --size <size>      Number of cells [default: 30000]

ARGS:
    <path>    Path to program [optional: will use stdin instead]
```
## Examples
From stdin: `cat ./path/to/code.bf | brainfrick`\
From path:  `brainfrick ./path/to/code.bf`

## Compiling
```
git clone https://github.com/Gingeh/brainfrick.git
cd ./brainfrick
cargo build --release 
```
