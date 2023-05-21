[![Rust](https://github.com/tomBoddaert/lminc/actions/workflows/rust.yml/badge.svg?event=push)](https://github.com/tomBoddaert/lminc/actions/workflows/rust.yml)

# LMinC

### This is an assembler and simulator for the Little Minion Computer and Little Man Computer.
The [Little Minion Computer](https://mjrbordewich.webspace.durham.ac.uk/lmc/) model was created by [Professor Magnus Bordewich](https://mjrbordewich.webspace.durham.ac.uk/) of [Durham University](https://www.durham.ac.uk/homepage/), based on the [Little Man Computer](https://en.wikipedia.org/wiki/Little_man_computer) created by [Dr. Stuart Madnick](https://en.wikipedia.org/wiki/Stuart_Madnick) of [M.I.T.](https://www.mit.edu/) in 1965.

### It supports the assembly in both specifications.
[Little Minion Computer Assembly](https://mjrbordewich.webspace.durham.ac.uk/wp-content/uploads/sites/186/2021/04/LMC-Instruction-Set.pdf), [Little Man Computer](http://www.yorku.ca/sychen/research/LMC/LMCInstructions.html)

## Installing with cargo
Make sure you have [cargo](https://rustup.rs/) installed.
```sh
cargo install --git https://github.com/tomboddaert/lminc
```

## Examples

### Assembly examples
- There is an example of assembly in [examples/fib.txt](examples/fib.txt).
- There are an example of number assembly in [examples/fib_num.txt](examples/fib_num.txt).

### Library examples
- There is an example of assembling and running from assembly in [examples/fibonacci.rs](examples/fibonacci.rs)
- There is an example of assembling and running from numbers in [examples/fibonacci_from_nums.rs](examples/fibonacci_from_nums.rs)
- There is an example of saving and loading a computer's memory in [examples/save_and_load.rs](examples/save_and_load.rs)

## Extended mode (unstable)
I am working on an extended mode. The documentation is in [extended_mode.md](extended_mode.md).

## License
[LMinC](https://github.com/tomBoddaert/lminc) is dual-licensed under either the [Apache License Version 2.0](/server/LICENSE_APACHE) OR [MIT](/server/LICENSE_MIT) license at your option.
