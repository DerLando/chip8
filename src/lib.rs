#![cfg_attr(not(feature = "std"), no_std)]
use opcode::OpCode;

mod command;
pub mod config;
mod cpu;
mod display;
pub mod emulator;
mod io;
mod memory;
mod opcode;
