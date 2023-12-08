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
  tag: usize,
  // Technical detail for calculating LRU
  instructions_since_use: usize,
  // Not strictly necessary, but avoids calculations later down the line
  starting_address: usize,
  data: Vec<i32>,
}

#[derive(Clone)]
struct Set(Vec<Block>);

pub struct Cache {
  sets: Vec<Set>,
  offset_bit_count: usize,
  set_bit_count: usize,
  block_size: usize,
  set_count: usize,
}

impl Block {
  fn new(block_size_in_words: usize) -> Block {
    Block {
      valid: false,
      dirty: false,
      // Tag 0 by default, will not matter since valid bit shows its unused
      tag: 0,
      instructions_since_use: 0,
      starting_address: 0,
      data: vec![0; block_size_in_words],
    }
  }
}

impl Set {
  fn new(block_size_in_words: usize, blocks_per_set: usize) -> Set {
    Set(vec![Block::new(block_size_in_words); blocks_per_set])
  }

  /// Calculates the block in the set that is the least recently used and returns it's index
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
  // except the block index/tag used
  fn use_block(&mut self, newest_index: usize) {
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
      block_size: block_size_in_words,
      set_count: number_of_sets,
    }
  }

  /// Used for getting a word from the cache (if not in cache, will add from memory first)
  pub fn get_value(&mut self, address: usize, memory: &mut [i32]) -> i32 {
    // self.print_cache();
    let tag = self.get_tag(address);
    let set_index = self.get_set_index(address);
    let block_offset = self.get_block_offset(address);
    let (small_bound, large_bound) = self.get_block_bounds(address);

    // println!("address:      {} -- {:b}", address, address);
    // println!("tag:          {} -- {:b}", tag, tag);
    // println!("set_index:    {} -- {:b}", set_index, set_index);
    // println!("block_offset: {} -- {:b}", block_offset, block_offset);

    let set = self.sets.get_mut(set_index).expect("Incorrect set index");
    for (i, block) in set.0.iter_mut().enumerate() {
      if block.tag == tag && block.valid {
        let data = *block
          .data
          .get(block_offset)
          .expect("Incorrect block offset");

        // Edit instructions_since_use for all blocks for calculating LRU
        block.instructions_since_use = 0;
        set.use_block(i);

        utils::print_action(address, 1, Action::CacheToProcessor);
        return data;
      }
    }

    // Since the address is not in the cache, we have to go to memory and add it to the cache
    let block_data = memory[small_bound..(large_bound + 1)].to_owned().to_vec();
    let lru_idx = set.least_recently_used();
    let block_reference = set.0.get_mut(lru_idx).expect("Incorrect LRU index");
    let old_block = std::mem::replace(
      block_reference,
      Block {
        valid: true,
        dirty: false,
        tag,
        instructions_since_use: 0,
        starting_address: small_bound,
        data: block_data,
      },
    );

    // Increment all other blocks instructions_since_use for calculating the LRU
    set.use_block(lru_idx);

    // Now that we have replaced the old block in the cache, we have to determine 2 things:
    // - if that block is valid
    // - if the block was valid, we also have to check if it was dirty or not to write back to memory
    if old_block.valid {
      if old_block.dirty {
        let memory_block = &mut memory[small_bound..(large_bound + 1)];
        assert!(
          memory_block.len() == old_block.data.len(),
          "Cache block length does not match memory block length\ncache: {}\nmemory: {}",
          old_block.data.len(),
          memory_block.len(),
        );
        memory_block.copy_from_slice(&old_block.data);
        utils::print_action(
          old_block.starting_address,
          self.block_size,
          Action::CacheToMemory,
        );
      } else {
        // old_block is already out of the cache, the memory for it will be deallocated at the end of the function
        utils::print_action(
          old_block.starting_address,
          self.block_size,
          Action::CacheToNowhere,
        );
      }
    }

    // Block is invalid so the new data is just added to the cache
    utils::print_action(small_bound, self.block_size, Action::MemoryToCache);
    // Since the block is now in the cache, rather than reimplementing all of the logic, we can just
    // call the function recursively to send the cached result to the processor
    return self.get_value(address, memory);
  }

  /// Used for storing a word in the cache (if the word's block is not in the cache, will add it first)
  pub fn set_value(&mut self, address: usize, memory: &mut [i32], value: i32) {
    // self.print_cache();
    let tag = self.get_tag(address);
    let set_index = self.get_set_index(address);
    let block_offset = self.get_block_offset(address);
    let (small_bound, large_bound) = self.get_block_bounds(address);

    // println!("address:      {} -- {:b}", address, address);
    // println!("tag:          {} -- {:b}", tag, tag);
    // println!("set_index:    {} -- {:b}", set_index, set_index);
    // println!("block_offset: {} -- {:b}", block_offset, block_offset);

    let set = self.sets.get_mut(set_index).expect("Incorrect set index");
    for (i, block) in set.0.iter_mut().enumerate() {
      if block.tag == tag && block.valid {
        let row = block
          .data
          .get_mut(block_offset)
          .expect("Incorrect block offset");
        *row = value;
        block.dirty = true;

        // Edit instructions_since_use for all blocks for calculating LRU
        block.instructions_since_use = 0;
        set.use_block(i);

        utils::print_action(address, 1, Action::ProcessorToCache);
        return;
      }
    }

    // If we are here, it means that the address's block was not in the cache, so we must add it
    let block_data = memory[small_bound..(large_bound + 1)].to_owned().to_vec();
    let lru_idx = set.least_recently_used();
    let block_reference = set.0.get_mut(lru_idx).expect("Incorrect LRU index");
    let old_block = std::mem::replace(
      block_reference,
      Block {
        valid: true,
        dirty: false,
        tag,
        instructions_since_use: 0,
        starting_address: small_bound,
        data: block_data,
      },
    );

    // Increment all other blocks instructions_since_use for calculating the LRU
    set.use_block(lru_idx);

    // Now that we have replaced the old block in the cache, we have to determine 2 things:
    // - if that block is valid
    // - if the block was valid, we also have to check if it was dirty or not to write back to memory
    if old_block.valid {
      if old_block.dirty {
        let memory_block = &mut memory[small_bound..(large_bound + 1)];
        assert!(
          memory_block.len() == old_block.data.len(),
          "Cache block length does not match memory block length\ncache: {}\nmemory: {}",
          old_block.data.len(),
          memory_block.len(),
        );
        memory_block.copy_from_slice(&old_block.data);
        utils::print_action(
          old_block.starting_address,
          self.block_size,
          Action::CacheToMemory,
        );
      } else {
        // old_block is already out of the cache, the memory for it will be deallocated at the end of the function
        utils::print_action(
          old_block.starting_address,
          self.block_size,
          Action::CacheToNowhere,
        );
      }
    }

    // Block is invalid so the new data is just added to the cache
    utils::print_action(small_bound, self.block_size, Action::MemoryToCache);

    // Now that the block is in the cache, we can recursively call the function to update the value
    // at the given address.
    self.set_value(address, memory, value);
  }

  fn get_block_offset(&self, address: usize) -> usize {
    // If block size is 1 overwrite to zero to ensure no
    // incorrect indexing into the block
    if self.block_size == 1 {
      return 0;
    }

    let offset = self.offset_bit_count;
    // Generates a bit mask that is the size of the block offset bits
    let mask: usize = (1 << offset) - 1;
    address & mask
  }

  fn get_set_index(&self, address: usize) -> usize {
    if self.set_count == 1 {
      return 0;
    }

    let block_offset: usize = if self.block_size == 1 {
      0
    } else {
      self.offset_bit_count
    };

    let set_offset = self.set_bit_count;
    let mask: usize = (1 << set_offset) - 1;
    (address >> block_offset) & mask
  }

  fn get_tag(&self, address: usize) -> usize {
    let offset_amount = self.offset_bit_count + self.set_bit_count;
    address >> offset_amount
  }

  fn get_block_bounds(&self, address: usize) -> (usize, usize) {
    let smaller = (address / self.block_size) * self.block_size;
    let larger = smaller + self.block_size - 1;
    (smaller, larger)
  }

  /// debug print function of the whole cache
  fn print_cache(&self) {
    println!("--- --- --- Cache --- --- ---");
    println!("block_size:       {}", self.block_size);
    println!("set_bit_count:    {}", self.set_bit_count);
    println!("offset_bit_count: {}", self.offset_bit_count);
    println!("set_count:        {}", self.sets.len());
    println!("--- --- --- Sets  --- --- ---");
    for (i, set) in self.sets.iter().enumerate() {
      println!("\tSet #{}", i);
      for (j, block) in set.0.iter().enumerate() {
        let valid: String;
        let dirty: String;
        if block.valid {
          valid = "V".to_string();
        } else {
          valid = "N".to_string();
        }

        if block.dirty {
          dirty = "D".to_string();
        } else {
          dirty = "C".to_string();
        }

        println!("\t\tBlock #{} -- {} {}", j, valid, dirty);
        for (k, row) in block.data.iter().enumerate() {
          println!("\t\t\tRow #{}'s Data: {}", k, row);
        }
      }
    }
  }
}
