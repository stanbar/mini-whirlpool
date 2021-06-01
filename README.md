# mini-Whirlpool

[mini-Whirlpool hash function](https://en.wikipedia.org/wiki/Whirlpool_(hash_function)) written in Rust.

Whirlpool hash function is defined over extended field `GF(2^8) = Z_2[x]/f` where `f = x^8 + x^5 + x^3 + x + 1 (0x12B)`, and smaller 4x4 state matrices.

## Usage
In order to compile and execute the program, Rust toolchain (cargo) needs to be installed. The easiest way to install it is through [rustup.rs](https://rustup.rs/). 
In order to run the program in debug mode execute

```
cargo run
```

it will expect the input string on stdin. Or just
```
echo -n Hello World | cargo run
```

Alternatively the input can be passed as an argument
```
cargo run -- "Hello World"
```

In order to execute the project goal (finiding the preimages) execute

```
cargo run --bin reverse-hash --release
```
