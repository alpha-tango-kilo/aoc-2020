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
        others.iter()
            .take(3)
            .for_each(|wradapter| {
                //println!("Borrowing: {:?}", wradapter);
                if self.plugs_into(&wradapter) && *self != *wradapter.borrow() {
                    self.compatible.push(wradapter.clone());
                }
            });
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
