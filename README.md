# Chip-8

A basic chip-8 emulator written in rust. Currently I am still in the state of implementing all opcodes properly. The target is to have proper unit tests for all opcodes and verification run against a test suite of chip-8 roms.

This is mainly an **educational project**. I am aware of other implementations and will **always** prioritize readability and clearity over performance.

## Features

I am *only* planning on implementing the *base* chip-8 instruction set and display, not the extended set for the *super* chip-8.

- [ ] Base instruction set
- [ ] Sound at 60Hz
- [ ] Delay timer at 60Hz
- [ ] No-std support

### Notable exclusions

Rendering won't be handled by this library, as I want the library itself to be render agnostic. I will build a reference rendering implementation by compiling to *WASM* with a web frontend at some point. For this the library should probably at some point support *no-std*.
