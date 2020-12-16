use anyhow::{Result, Error};

#[derive(Debug)]
pub struct Buffer {
    store: Vec<u32>,
    size: usize,
}

impl Buffer {
    pub fn new(size: &usize) -> Self {
        Buffer {
            store: Vec::with_capacity(*size),
            size: *size,
        }
    }

    pub fn checked_insert(&mut self, elem: u32) -> Result<()> {
        //println!("Trying to insert {} into {:?}", elem, self);
        if self.check_valid(&elem) {
            if self.store.len() == self.size {
                self.store.remove(0);
            }
            self.store.push(elem);
            Ok(())
        } else {
            Err(Error::msg("Found number not following XMAS protocol"))
        }
    }

    fn check_valid(&self, elem: &u32) -> bool {
        if self.store.len() == self.size {
            for (index, item_a) in self.store.iter().enumerate() {
                for item_b in self.store.iter().skip(index) {
                    if item_a + item_b == *elem {
                        //println!("{} + {} == {}", item_a, item_b, elem);
                        return true;
                    }
                }
            }
            false
        } else {
            true
        }
    }
}

pub fn find_contiguous_slice_summing_to(v: &Vec<u32>, n: &u32) -> Option<Box<[u32]>> {
    for start_index in 0..v.len() {
        let mut total = 0;
        for (relative_index, current) in v[start_index..].iter().enumerate() {
            total += current;
            if total == *n {
                let slice: Box<_> = v[start_index..start_index + relative_index + 1].into();
                println!("\n{:?} sums to {}", slice, n);
                return Some(slice);
            } else if total > *n {
                break;
            }
        }
    }
    None
}
