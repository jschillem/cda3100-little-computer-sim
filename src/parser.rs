// Justin Schilleman (jas21ba) | CDA3100 | Assignment 2

use crate::types::*;

pub fn parse_instruction(instruction: i32) -> OpType {
    match get_opcode(instruction) {
        0b000 => OpType::R(RType {
            code: RTypeOpcode::Add,
            register_a: get_reg_a(instruction),
            register_b: get_reg_b(instruction),
            destination: get_dest_reg(instruction),
        }),
        0b001 => OpType::R(RType {
            code: RTypeOpcode::Nand,
            register_a: get_reg_a(instruction),
            register_b: get_reg_b(instruction),
            destination: get_dest_reg(instruction),
        }),
        0b010 => OpType::I(IType {
            code: ITypeOpcode::LoadWord,
            register_a: get_reg_a(instruction),
            register_b: get_reg_b(instruction),
            offset: get_offset(instruction),
        }),
        0b011 => OpType::I(IType {
            code: ITypeOpcode::StoreWord,
            register_a: get_reg_a(instruction),
            register_b: get_reg_b(instruction),
            offset: get_offset(instruction),
        }),
        0b100 => OpType::I(IType {
            code: ITypeOpcode::BranchEq,
            register_a: get_reg_a(instruction),
            register_b: get_reg_b(instruction),
            offset: get_offset(instruction),
        }),
        0b101 => OpType::O(OType {
            code: OTypeOpcode::X,
        }),
        0b110 => OpType::O(OType {
            code: OTypeOpcode::Halt,
        }),
        0b111 => OpType::O(OType {
            code: OTypeOpcode::NoOp,
        }),
        _ => panic!("Something went wrong when calculating the opcode"),
    }
}

fn get_opcode(instruction: i32) -> u8 {
    ((instruction >> 22) & 0b111) as u8
}

fn get_reg_a(instruction: i32) -> u8 {
    ((instruction >> 19) & 0b111) as u8
}

fn get_reg_b(instruction: i32) -> u8 {
    ((instruction >> 16) & 0b111) as u8
}

fn get_dest_reg(instruction: i32) -> u8 {
    (instruction & 0b111) as u8
}

fn get_offset(instruction: i32) -> i16 {
    (instruction & 0xFFFF) as i16
}
