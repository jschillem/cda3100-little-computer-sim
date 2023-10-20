// Justin Schilleman (jas21ba) | CDA3100 | Assignment 1

mod parser;
pub mod types;
mod utils;

use parser::parse_instruction;
use types::*;
use utils::print_state;

use clap::Parser;
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
}

fn main() {
  // Read in command line argument using CLAP package for simplicity
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
  };

  // Read instructions from file into memory
  for line in io::BufReader::new(file).lines().enumerate() {
    let instruction: i32 = line.1.unwrap().trim().parse().expect("NaN");
    state.mem[line.0] = instruction;
    state.num_memory += 1;
  }

  let mut halted = false;
  let mut count = 0;

  while !halted && state.pc as usize <= NUM_MEMORY {
    print_state(&state);

    let current_instruction = state.mem[state.pc as usize];
    state.pc += 1;

    let current_instruction = parse_instruction(current_instruction);
    match current_instruction {
      OpType::O(o_type) => match o_type.code {
        OTypeOpcode::Halt => halted = true,
        OTypeOpcode::NoOp => continue,
        // unused so treating as NoOp
        OTypeOpcode::X => continue,
      },
      OpType::R(r_type) => match r_type.code {
        RTypeOpcode::Add => {
          let reg_a = state.reg[r_type.register_a as usize];
          let reg_b = state.reg[r_type.register_b as usize];
          state.reg[r_type.destination as usize] = reg_a + reg_b;
        }
        RTypeOpcode::Nand => {
          let reg_a = state.reg[r_type.register_a as usize];
          let reg_b = state.reg[r_type.register_b as usize];
          state.reg[r_type.destination as usize] = !(reg_a & reg_b);
        }
      },
      OpType::I(i_type) => match i_type.code {
        ITypeOpcode::LoadWord => {
          let reg_a = state.reg[i_type.register_a as usize];
          state.reg[i_type.register_b as usize] =
            state.mem[(i_type.offset as i32 + reg_a) as usize];
        }
        ITypeOpcode::StoreWord => {
          let address = state.reg[i_type.register_a as usize] + i_type.offset as i32;
          state.mem[address as usize] = state.reg[i_type.register_b as usize];
        }
        ITypeOpcode::BranchEq => {
          let reg_a = state.reg[i_type.register_a as usize];
          let reg_b = state.reg[i_type.register_b as usize];
          if reg_a == reg_b {
            state.pc = (state.pc as i32 + i_type.offset as i32) as u32;
          }
        }
      },
    }

    count += 1;
  }

  println!("\nmachine halted");
  println!("total of {} instructions executed", count);
  println!("final state of the machine:");
  print_state(&state);
}
