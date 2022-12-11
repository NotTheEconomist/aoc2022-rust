pub mod parser;
use std::{cell::RefCell, collections::VecDeque};

pub struct Monkey {
    items: VecDeque<i64>,
    op: Box<dyn Fn(i64) -> i64>,
    test: Box<dyn Fn(i64) -> usize>,
}

pub trait Inspector {
    fn inspect(&mut self) -> Result<(i64, usize), String>;
    fn throw(&self, item: i64, recipient: &mut dyn Inspector);
    fn receive(&mut self, item: i64);
}

impl Inspector for Monkey {
    fn inspect(&mut self) -> Result<(i64, usize), String> {
        let item = self
            .items
            .pop_front()
            .ok_or_else(|| "Monkey has no items".to_string())?;
        let item = (self.op)(item);
        let item = item / 3;
        let recipient = (self.test)(item);
        Ok((item, recipient))
    }

    fn throw(&self, item: i64, recipient: &mut dyn Inspector) {
        recipient.receive(item)
    }

    fn receive(&mut self, item: i64) {
        self.items.push_back(item)
    }
}

pub struct Monkeys {
    pub monkeys: Vec<RefCell<Monkey>>,
    pub inspection_count: Vec<usize>,
}

impl Monkeys {
    pub fn new(monkeys: Vec<Monkey>) -> Self {
        let inspection_count = monkeys.iter().map(|_| 0usize).collect();
        let monkeys = monkeys.into_iter().map(RefCell::new).collect();
        Self {
            monkeys,
            inspection_count,
        }
    }
    pub fn do_round(&mut self) {
        for (idx, monkey) in self.monkeys.iter().enumerate() {
            while let Ok((item, recipient_idx)) = monkey.try_borrow_mut().unwrap().inspect() {
                if let Some(count) = self.inspection_count.get_mut(idx) {
                    *count += 1;
                }
                let recipient = self
                    .monkeys
                    .get(recipient_idx)
                    .expect("Inspector::inspect must never return an invalid index");
                recipient.try_borrow_mut().unwrap().receive(item);
            }
        }
    }
}
