# Whirlpool-like

[Whirlpool-like hash function](https://en.wikipedia.org/wiki/Whirlpool_(hash_function)) written in Rust.

This Whirlpool implementation is defined on extended field `GF(2^8) = Z_2[x]/f` where `f = x^8 + x^5 + x^3 + x + 1 (0x12B)`, and smaller 4x4 state matrices.

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

Finding the reverse of a hash

```
cargo run --bin reverse-hash
```
