use crate::State;

pub fn print_state(state: &State) {
  println!("\n@@@");
  println!("state:");
  println!("\tpc {}", state.pc);
  println!("\tmemory:");
  for idx in 0..state.num_memory as usize {
    println!("\t\tmem[ {} ] {}", idx, state.mem[idx]);
  }
  println!("\tregisters:");
  for register in state.reg.iter().enumerate() {
    println!("\t\treg[ {} ] {}", register.0, register.1);
  }
  println!("end state");
}
