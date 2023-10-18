// Justin Schilleman (jas21ba) | CDA3100 | Assignment 1

pub enum OpType {
  R(RType),
  I(IType),
  O(OType),
}

pub struct RType {
  pub code: RTypeOpcode,
  pub register_a: u8,
  pub register_b: u8,
  pub destination: u8,
}

pub struct IType {
  pub code: ITypeOpcode,
  pub register_a: u8,
  pub register_b: u8,
  pub offset: i16,
}

pub struct OType {
  pub code: OTypeOpcode,
}

pub enum RTypeOpcode {
  Add,
  Nand,
}

pub enum ITypeOpcode {
  LoadWord,
  StoreWord,
  BranchEq,
}

pub enum OTypeOpcode {
  X,
  Halt,
  NoOp,
}
