pub enum OpType {
  R(RType),
  I(IType),
  O(OType),
}

pub struct RType {
  code: RTypeOpcode,
  register_a: u8,
  register_b: u8,
  destination: u8,
}

pub struct IType {
  code: ITypeOpcode,
  register_a: u8,
  register_b: u8,
  offset: i16,
}

pub struct OType {
  code: OTypeOpcode,
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
