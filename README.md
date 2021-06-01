# mini-Whirlpool

[mini-Whirlpool hash function](https://en.wikipedia.org/wiki/Whirlpool_(hash_function)) written in Rust.

Whirlpool hash function is defined over extended field `GF(2^8) = Z_2[x]/f` where `f = x^8 + x^5 + x^3 + x + 1 (0x12B)`, and smaller 4x4 state matrices.

## Implementation
I decided to implement this program https://en.wikipedia.org/wiki/Rust_(programming_language)
## Usage

To calculate hash of some input you can either pass it as an argument

```
cargo run <some input>
```

or pass as an stdin

```
echo -n <some input> | cargo run
```


## Example

Finding the reverses of some hashes

```
cargo run --bin reverse-hash --release
```
