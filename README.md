# bf_interpreter

Super basic, barely tested [Brainfuck](https://en.wikipedia.org/wiki/Brainfuck) interpreter written in Rust in an evening to avoid boredom.

Brainfuck is an esoteric programming language with only 8 commands operating on a tape of 30,000 byte cells:

| Command | Description |
|---------|-------------|
| `>`     | Move pointer right |
| `<`     | Move pointer left |
| `+`     | Increment current cell |
| `-`     | Decrement current cell |
| `.`     | Output current cell as ASCII |
| `,`     | Read one byte of input into current cell |
| `[`     | Jump past matching `]` if current cell is 0 |
| `]`     | Jump back to matching `[` if current cell is non-zero |

Everything else is treated as a comment.

## Usage

Put your BF source in `main.rs` and run:

```sh
cargo run
```

## Implementation

- 30,000 cell tape of `u8` values
- Jump table precomputed at parse time. no scanning at runtime
- Non-BF characters stripped before execution

## Example

```bf
++++++++[>++++++++>++++++++++++<<-]>+>+>++++++++++>++++++++++++++++++++++++++[<<<.>.>.<<+>+>>-]
```

Prints the alphabet as uppercase/lowercase pairs:

```
Aa
Bb
Cc
...
Zz
```
