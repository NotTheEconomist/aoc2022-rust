pub mod parser;
use std::{cell::RefCell, collections::VecDeque, num::ParseIntError, ops::Rem, str::FromStr};

pub trait MonkeyMath {
    fn add_constant(&mut self, constant: usize);
    fn mul_constant(&mut self, constant: usize);
    fn square_self(&mut self);
    fn divisible_by(&self, divisor: usize) -> bool;
}

impl MonkeyMath for i64 {
    fn add_constant(&mut self, constant: usize) {
        *self += constant as i64;
    }

    fn mul_constant(&mut self, constant: usize) {
        *self *= constant as i64;
    }

    fn square_self(&mut self) {
        *self = self.pow(2)
    }

    fn divisible_by(&self, divisor: usize) -> bool {
        *self % divisor as i64 == 0
    }
}

#[derive(Clone, Copy, Debug)]
pub struct MonkeyNum(i64);
impl FromStr for MonkeyNum {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let n = s.parse::<i64>()?;
        Ok(Self(n))
    }
}
impl From<i64> for MonkeyNum {
    fn from(n: i64) -> Self {
        Self(n)
    }
}

impl Rem<i64> for MonkeyNum {
    type Output = i64;

    fn rem(self, rhs: i64) -> Self::Output {
        self.0 % rhs
    }
}
#[cfg(test)]
mod monkeynum_tests {

    use super::MonkeyNum;
    #[test]
    fn parse() {
        "22".parse::<MonkeyNum>().unwrap();
    }
}
impl MonkeyMath for MonkeyNum {
    fn add_constant(&mut self, constant: usize) {
        self.0 += constant as i64;
    }

    fn mul_constant(&mut self, constant: usize) {
        self.0 *= constant as i64;
    }

    fn square_self(&mut self) {
        self.0 *= self.0;
    }

    fn divisible_by(&self, divisor: usize) -> bool {
        self.0 % divisor as i64 == 0
    }
}
pub struct Test {
    divisor: i64,
    true_recipient: usize,
    false_recipient: usize,
}

impl Test {
    fn get_recipient<T>(&self, item: T) -> usize
    where
        T: Rem<i64, Output = i64>,
        T: Copy,
    {
        if item % self.divisor == 0 {
            self.true_recipient
        } else {
            self.false_recipient
        }
    }
}

pub struct Monkey<T> {
    items: VecDeque<T>,
    op: Box<dyn Fn(&mut T)>,
    test: Test,
}

pub trait Inspector<T> {
    fn inspect(&mut self) -> Result<(T, usize), String>;
    fn throw(&self, item: T, recipient: &mut dyn Inspector<T>);
    fn receive(&mut self, item: T);
}

impl Inspector<i64> for Monkey<i64> {
    fn inspect(&mut self) -> Result<(i64, usize), String> {
        let mut item = self
            .items
            .pop_front()
            .ok_or_else(|| "Monkey has no items".to_string())?;
        (self.op)(&mut item);
        let item = item / 3;
        let recipient = self.test.get_recipient(item);
        Ok((item, recipient))
    }

    fn throw(&self, item: i64, recipient: &mut dyn Inspector<i64>) {
        recipient.receive(item)
    }

    fn receive(&mut self, item: i64) {
        self.items.push_back(item)
    }
}

impl Inspector<MonkeyNum> for Monkey<MonkeyNum> {
    fn inspect(&mut self) -> Result<(MonkeyNum, usize), String> {
        let mut item = self
            .items
            .pop_front()
            .ok_or_else(|| "Monkey has no items".to_string())?;
        (self.op)(&mut item);
        let recipient = self.test.get_recipient(item);
        Ok((item, recipient))
    }

    fn throw(&self, item: MonkeyNum, recipient: &mut dyn Inspector<MonkeyNum>) {
        recipient.receive(item)
    }

    fn receive(&mut self, item: MonkeyNum) {
        self.items.push_back(item)
    }
}

pub struct Monkeys<T> {
    pub monkeys: Vec<RefCell<Monkey<T>>>,
    pub inspection_count: Vec<usize>,
    pub divisor_product: i64,
}

impl<T> Monkeys<T> {
    pub fn new(monkeys: Vec<Monkey<T>>) -> Self {
        let inspection_count = monkeys.iter().map(|_| 0usize).collect();
        let divisor_product = monkeys.iter().map(|monkey| monkey.test.divisor).product();
        let monkeys = monkeys.into_iter().map(RefCell::new).collect();
        Self {
            monkeys,
            inspection_count,
            divisor_product,
        }
    }
    pub fn do_round(&mut self)
    where
        Monkey<T>: Inspector<T>,
        T: Rem<i64, Output = i64>,
        T: From<i64>,
    {
        for (idx, monkey) in self.monkeys.iter().enumerate() {
            while let Ok((item, recipient_idx)) = monkey.try_borrow_mut().unwrap().inspect() {
                if let Some(count) = self.inspection_count.get_mut(idx) {
                    *count += 1;
                }
                let recipient = self
                    .monkeys
                    .get(recipient_idx)
                    .expect("Inspector::inspect must never return an invalid index");
                let item = (item % self.divisor_product).into();
                recipient.try_borrow_mut().unwrap().receive(item);
            }
        }
    }
}
