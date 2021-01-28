<div align="center">

# NCDA (Noid Check Digit Algorithm)

A blazingly fast and lightweight Rust implementation of the NCDA checking algorithm

[![Project Status: Concept – Minimal or no implementation has been done yet, or the repository is only intended to be a limited example, demo, or proof-of-concept.](https://www.repostatus.org/badges/latest/concept.svg)](https://www.repostatus.org/#concept)

</div>

## NCDA in brief

NCDA (Noid Check Digit Algorithm) is an algorithm used to compute or validate NOID checksum char.

NCDA works well with identifiers such as ARKs. It can be used to assert that an ID doesn't contains transcription error.

NCDA is a part of the NOID project which was developed by John A. Kunze ([@jkunze](https://github.com/jkunze)).

## Motivation

This project is primarily a learning project. It has been inspired by the [ark-tools](https://github.com/BertrandCaron/ark-tools) project made by Bertrand Carron ([@BertrandCaron](https://github.com/BertrandCaron)). 

This project is an attempt to build a full featured Rust app from A to Z with few goals in mind:

* Learn basic concepts of [WebAssembly](https://webassembly.org/) 🕸
* Learn how to build python native modules with Rust and [Pyo3](https://github.com/PyO3/pyo3) 🐍
* Learn how to build Web Services with [Rocket.rs](https://www.rocket.rs/) 🚀
* Lean how to build GUI app with [Tauri](https://github.com/tauri-apps/tauri) ⛩️
* Keep learning things about Rust optimization and error handling 🦀

This project tries to produce real outcomes such as :

- [ ]  A CLI
- [ ]  A GUI
- [x]  A Web Service
- [x]  A Rust library
- [x]  A Nodejs library
- [x]  A Python library

This implementation of NCDA provides bindings with :

* Nodejs
* Python

## Build 

### Rust

> This section explains how to use the NCDA crate inside your Rust project.

1- Create a new Rust project with cargo :

```sh
cargo init ncda-checking
cd ncda-checking
```

1- Add the following code to `Cargo.toml` :

```toml
[dependencies]
ncda = "*"
```

2- Add the following code to `src/main.rs` :

```rust
use ncda;

fn main() {
    assert_eq!(ncda::checksum("cb32752361"), Ok('d'));
    assert_eq!(ncda::check("cb32752361d"), Ok(()));
}
```

3- Run your app :

```sh
cargo run
```

### Nodejs

> This section explains how to compile the ncda library into a Nodejs module and how to use this module inside your Nodejs app.

1- Compile the ncda library to Nodejs with wasm-pack :

```sh
wasm-pack build --release --target Nodejs
```

2- Create a basic Nodejs project :

```sh
mkdir Nodejs && cd Nodejs
npm init -y     
npm i -D ../pkg
touch index.js
```

3- Add the following code to `index.js`  :

```js
const ncda = require("ncda");

let checksum_char = ncda.checksum('cb32752361');
let checking_result = ncda.check('cb32752361d');

console.log(`checksum char : ${checksum_char}`);
console.log(`cheking result for cb32752361d : ${checking_result}`);
```

4- Run your app :

```sh
node index.js
```

### Python

> 🚧 TODO 🚧

## Benchmark

> Like any benchmark you might get slightly different results on your system. The following results are provided solely for information purposes.

<div align="center">

| Measure                  | ID length | Result             | Tool          |
| ------------------------ | :-------: | ------------------ | ------------- |
| `ncda::check`            |    11     | 23 ns/iter (+/- 3) | `cargo bench` |
| `ncda::checksum`         |    10     | 26 ns/iter (+/- 0) | `cargo bench` |
| Total memory consumption |    11     | 72.7 ko            | `heaptrack`   |

</div>

## Other implementations and related projects (not limited)

* [The Noid project on CPAN](https://metacpan.org/pod/distribution/Noid/noid) (Perl) [original implementation]
* [node-inist-ark](https://github.com/Inist-CNRS/node-inist-ark) (Nodejs)
* [Noid4Php](https://github.com/Daniel-KM/Noid4Php/blob/master/noid) (PHP)
* [ark-tools](https://github.com/BertrandCaron/ark-tools) (Python)

## References

- [The Noid project on CPAN](https://metacpan.org/pod/distribution/Noid/noid)
