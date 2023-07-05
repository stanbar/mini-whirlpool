# mini-Whirlpool

[mini-Whirlpool hash function](https://en.wikipedia.org/wiki/Whirlpool_(hash_function)) written in Rust.

Whirlpool hash function is defined over extended field `GF(2^8) = Z_2[x]/f` where `f = x^8 + x^5 + x^3 + x + 1 (0x12B)`, and smaller 4x4 state matrices.

The whole report is available [here](report/report.pdf).

## Project

The goal of this project is to find a preimage for the output of the Whirlpool-like hash function. Formally, finding _x_ for defined upfront _h=H(x)_, where _H_ is a Whirpool-like hash function—modification of the original Whirlpool function.

## Whirlpool

Whirlpool is a hash function with a cipher-block-based compression function, meaning that its hash code results from encrypting message blocks using a recursive scheme. The output of encryption becomes an encryption key for the next block, the so-called chaining variable.

Whirlpool compression function is an encryption algorithm based on AES; it takes a 512-bit block of plaintext and a 512-bit key as input and produces a 512-bit block of ciphertext as output. The encryption algorithm involves the use of four transformations: Add Key (AK), Substitute Bytes (SB), Shift Columns (SC), and Mix Rows (MR)

This Whirlpool-like function differs from the original Whirlpool by using different padding, smaller hash code (128-bit over 512-bit), different irreducible polynomial (0x12B over 0x011D), different S-box, Diffusion Matrix, and Round Keys.

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

## Implementation

I decided to implement the program in [Rust](https://en.wikipedia.org/wiki/Rust_(programming_language))—relatively new (released on July 7, 2010) programming language. Rust offers C-level performance, a borrow checker, an excellent type system, and a modern toolchain, making it [the most loved programming language of 2020 (according to StackOverflow 2020 Developer Survey](https://insights.stackoverflow.com/survey/2020#most-loved-dreaded-and-wanted). It is also a popular choice for new projects where cryptography is involved.

Besides std, I used one external library([Rayon](https://github.com/rayon-rs/rayon)) to achieve easy multi-threaded execution.


| Input length | Preimage found | Execution time   |
|--------------|----------------|------------------|
| 2            | 0J             | 25.635827ms      |
| 3            | Ss1            | 1.89898985s      |
| 4            | @@-8           | 172.601573451s   |
| 5            | qDP0Z          | 10431.405587416s |
| 6            | Not Found      | Not Found        |
