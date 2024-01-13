# magi

A GameBoy emulator written in Rust.

## About

Currently, this project is very far from being able to run a game.

As my first emulator, my only real goals are to learn more about emulation, and
to get at least one game to run.

One decision I've made for this project is to use as few external dependencies
as possible. The intention here is twofold: limiting the amount of external
crates I use will force me to learn how to solve complex problems on my own, and
it should (hypothetically) make the project a bit more timeless with regards to
crate resolution (AKA, in 10 years, it won't run into problems compiling due to
some crate disappearing from the internet).

## Motivation

I've always wanted to write an emulator, and I've also been looking for a
serious project that would actually push me to properly learn Rust. This seemed
like the most logical solution to both desires, so here we are. Rust is also a
natural fit for an emulator like this, since we're emulating low-level hardware.
