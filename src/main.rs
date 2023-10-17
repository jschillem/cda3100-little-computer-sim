mod parser;
pub mod types;
mod utils;

use parser::parse_instruction;
use types::*;
use utils::print_state;

use clap::Parser;
use std::fs::read_to_string;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;

const NUM_MEMORY: usize = u16::MAX as usize + 1;
const NUM_REGISTER: usize = 8;

/// LC3100 behavioral simulator written by Justin Schilleman (jas21ba)
#[derive(Parser, Debug)]
struct Args {
  /// Path of the machine code file to be executed
  input: String,
}

pub struct State {
  pc: u32,
  mem: [i32; NUM_MEMORY],
  reg: [i32; NUM_REGISTER],
  num_memory: u32,
  halted: bool,
}

fn main() {
  // Read in command line argument using CLAP package
  let args = Args::parse();
  let input_path = Path::new(&args.input);
  let file = match File::open(&input_path) {
    Err(why) => panic!("Couldn't open {}: {}", input_path.display(), why),
    Ok(file) => file,
  };

  let mut state = State {
    pc: 0,
    mem: [0; NUM_MEMORY],
    reg: [0; NUM_REGISTER],
    num_memory: 0,
    halted: false,
  };

  // Read instructions from file into memory
  for line in io::BufReader::new(file).lines().enumerate() {
    let instruction: i32 = line.1.unwrap().trim().parse().expect("NaN");
    state.mem[line.0] = instruction;
    state.num_memory += 1;
  }

  //TODO: Execution loop + parsing instruction in parser.rs
  while !state.halted {
    let current_instruction = state.mem[state.pc as usize];
    let current_instruction = parse_instruction(current_instruction);
    state.pc += 1;
  }

  print_state(&state);
}
