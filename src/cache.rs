use crate::utils;

pub enum Action {
  CacheToProcessor,
  ProcessorToCache,
  MemoryToCache,
  CacheToMemory,
  CacheToNowhere,
}

#[derive(Clone)]
struct Block {
  valid: bool,
  dirty: bool,
  // Technical detail for calculating LRU
  instructions_since_use: usize,
  data: Vec<i32>,
}

#[derive(Clone)]
struct Set(Vec<Block>);

pub struct Cache {
  sets: Vec<Set>,
  offset_bit_count: usize,
  set_bit_count: usize,
}

impl Block {
  fn new(block_size_in_words: usize) -> Block {
    Block {
      valid: false,
      dirty: false,
      instructions_since_use: 0,
      data: vec![0; block_size_in_words],
    }
  }
}

impl Set {
  fn new(block_size_in_words: usize, blocks_per_set: usize) -> Set {
    Set(vec![Block::new(block_size_in_words); blocks_per_set])
  }

  // Calculates the block in the set that is the least recently used
  fn least_recently_used(&self) -> usize {
    let mut lru_idx: usize = 0;
    let mut lru_instruction_count: Option<usize> = None;
    for (i, block) in self.0.iter().enumerate() {
      // Invalid blocks will always get first priority
      if !block.valid {
        return i;
      }

      if lru_instruction_count.is_none() {
        lru_idx = i;
        lru_instruction_count = Some(block.instructions_since_use);
      } else {
        let lru_count = lru_instruction_count.unwrap();

        if block.instructions_since_use > lru_count {
          lru_instruction_count = Some(block.instructions_since_use);
          lru_idx = i;
        }
      }
    }
    return lru_idx;
  }

  // Increments the blocks' instructions_since_use value for every block
  // except the block index/tag provided
  fn increment_instruction_counts(&mut self, newest_index: usize) {
    for (i, block) in self.0.iter_mut().enumerate() {
      if i != newest_index {
        block.instructions_since_use += 1;
      }
    }
  }
}

impl Cache {
  pub fn new(block_size_in_words: usize, number_of_sets: usize, blocks_per_set: usize) -> Cache {
    Cache {
      sets: vec![Set::new(block_size_in_words, blocks_per_set); number_of_sets],
      offset_bit_count: utils::min_bits_required(block_size_in_words),
      set_bit_count: utils::min_bits_required(number_of_sets),
    }
  }

  pub fn get_value(&mut self, address: usize, memory: &[i32]) {
    let set_tag = self.get_set_tag(address);
    let block_offset = self.get_block_offset(address);
    println!("set_tag: {:b}", set_tag);
    println!("block_offset: {:b}", block_offset);
  }

  fn print_action(address: usize, block_size: usize, action: Action) {
    print!(
      "@@@ transferring word [{}-{}] ",
      address,
      address + block_size - 1
    );
    match action {
      Action::CacheToProcessor => println!("from the cache to the processor"),
      Action::ProcessorToCache => println!("from the processor to the cache"),
      Action::MemoryToCache => println!("from the memory to the cache"),
      Action::CacheToMemory => println!("from the cache to the memory"),
      Action::CacheToNowhere => println!("from the cache to nowhere"),
    }
  }

  fn get_block_offset(&self, address: usize) -> usize {
    let offset = self.offset_bit_count;
    // Generates a bit mask that is the size of the block offset bits
    let mask: usize = (1 << offset) - 1;
    address & mask
  }

  fn get_set_tag(&self, address: usize) -> usize {
    let block_offset = self.offset_bit_count;
    let set_offset = self.set_bit_count;
    let mask: usize = (1 << set_offset) - 1;
    (address >> block_offset) & mask
  }
}
