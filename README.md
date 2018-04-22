# BadAppleOS.rs

Re-implementation of [BadAppleOS](https://github.com/foreverbell/BadAppleOS) in Rust.

Inspired by,

* [Writing an OS in Rust](https://os.phil-opp.com/).
* [Singularity](https://www.microsoft.com/en-us/research/project/singularity/).

## Build

Requires these toolchains,

* nightly rust
  + tested with `rustc 1.25.0-nightly (b1f8e6fb0 2018-02-22)`.
* xargo
* qemu
* nasm
* python2
* docker
  + required to run `grub-mkrescue`.

Please check `Makefile` first before running these commands for an ISO.

```sh
$ make docker
$ make iso
$ make qemu
```
