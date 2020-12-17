use anyhow::Result;
use std::rc::Rc;
use std::cell::RefCell;

type Wradpter = Rc<RefCell<Adapter>>;

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
pub struct Adapter {
    output: u16,
    compatible: Vec<Wradpter>,
}

impl Adapter {
    fn new(output: u16) -> Rc<RefCell<Self>> {
        Rc::new(
            RefCell::new(
                Adapter {
                    output,
                    compatible: Vec::with_capacity(3),
                }
            )
        )
    }

    fn from_str(s: &str) -> Result<Rc<RefCell<Self>>> {
        Ok(Adapter::new(s.parse::<u16>()?))
    }

    fn record_compatible(&mut self, others: &[Wradpter]) {
        // Write as for with break as else branch
        for wradapter in others.iter().take(3) {
            if self.plugs_into(&wradapter) && *self != *wradapter.borrow() {
                self.compatible.push(wradapter.clone());
            } else {
                break;
            }
        }
    }

    fn plugs_into(&self, other: &Rc<RefCell<Self>>) -> bool {
        // Will never underflow as other is guaranteed to be larger
        other.borrow().output - self.output <= 3
    }
}

pub fn load_adapters_from_string(input: &str) -> Result<Vec<Wradpter>> {
    let mut adapters = input.lines()
        .map(Adapter::from_str)
        .collect::<Result<Vec<_>>>()?;
    adapters.sort_unstable();
    adapters.iter()
        .enumerate()
        .for_each(|(index, wradapter)| {
            let mut unwradapter = wradapter.borrow_mut();
            unwradapter.record_compatible(&adapters[index + 1..]);
        });
    Ok(adapters)
}

/*
// requires adapters to be sorted
pub fn get_longest_chain(adapters: &Vec<Wradpter>) -> Vec<Wradpter> {


    vec![]
}

fn explore_children(adapter: &Wradpter, chain: &mut Vec<Wradpter>, final_target: &u16, desired_depth: usize) -> Option<Wradpter> {
    if desired_depth == 0 {
        return None;
    }
    for child in adapter.borrow().compatible {
        if let Some(next) = explore_children(&child, final_target, desired_depth - 1) {
            return Some(next);
        }
    }
    None
}

pub fn get_differences_in_chain(adapters: &Vec<Wradpter>) -> (usize, usize, usize) {
    (0, 0, 0)
}
*/

pub fn part_one_fast(adapters: &Vec<Wradpter>) -> usize {
    let mut one_jolts = 0;
    let mut three_jolts = 1; // accounts for device's adapter

    // account for difference from wall to adapter
    if adapters[0].borrow().output == 1 {
        one_jolts += 1;
    } else if adapters[0].borrow().output == 3 {
        three_jolts += 1;
    }

    adapters.iter()
        .enumerate()
        .skip(1)
        .for_each(|(index, adapter)| {
            let diff = adapter.borrow().output - adapters[index - 1].borrow().output;
            if diff == 1 {
                one_jolts += 1;
            } else if diff == 3 {
                three_jolts += 1;
            }
        });
    println!("Ones:\t{}\nThrees:\t{}", one_jolts, three_jolts);
    one_jolts * three_jolts
}
