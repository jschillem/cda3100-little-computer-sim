use crate::types::*;

pub fn parse_instruction(instruction: i32) -> OpType {
  let opcode = (instruction >> 22) & 0xFF;
  println!("{opcode:b}");
}
