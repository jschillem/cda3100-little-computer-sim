// Justin Schilleman (jas21ba) | CDA3100 | Assignment 2

use crate::cache::Action;
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

pub fn print_action(address: usize, size: usize, action: Action) {
  print!(
    "@@@ transferring word [{}-{}] ",
    address,
    address + size - 1
  );
  match action {
    Action::CacheToProcessor => println!("from the cache to the processor"),
    Action::ProcessorToCache => println!("from the processor to the cache"),
    Action::MemoryToCache => println!("from the memory to the cache"),
    Action::CacheToMemory => println!("from the cache to the memory"),
    Action::CacheToNowhere => println!("from the cache to nowhere"),
  }
}

pub fn is_power_of_two(n: usize) -> bool {
  // A power of two has only one bit set in its binary representation.
  // Therefore, subtracting 1 from a power of two results in a number
  // with all bits set to the right of the original bit.
  // For example, 8 (1000 in binary) minus 1 gives 7 (0111 in binary).
  // If ANDing the original number and the number one less than it equals 0,
  // then it is a power of 2.
  n != 0 && (n & (n - 1)) == 0
}

pub fn min_bits_required(mut num: usize) -> usize {
  if num <= 1 {
    return 1;
  }

  let mut count = 0;
  // subtract 1 to make it so it calculates # of bits for num options
  // min_bits_required(2) -> 1, min_bits_required(4) -> 2, etc.
  num -= 1;
  while num != 0 {
    // Right shift the number by 1 bit
    num >>= 1;
    count += 1;
  }

  count
}
