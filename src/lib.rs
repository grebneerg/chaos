use std::fmt;
use std::iter::Iterator;

pub struct RecursiveFunction<T, F>
where
    T: Copy + CloseEnough,
    F: Fn(T) -> T,
{
    func: F,
    start: T,
}

impl<T, F> RecursiveFunction<T, F>
where
    T: Copy + CloseEnough,
    F: Fn(T) -> T,
{
    pub fn end_behavior(&self, max_cycles: usize) -> Behavior<T> {
        let mut vals = Vec::new();
        for val in self.iter().skip(10000).take(max_cycles) {
            if !vals.iter().any(|x: &T| x.close_enough_to(&val)) {
                vals.push(val);
            }
        }

        if vals.len() == 1 {
            Behavior::Convergence(vals[0])
        } else if vals.len() == max_cycles {
            Behavior::Chaos
        } else {
            Behavior::Cycle(vals)
        }
    }

    pub fn iter(&self) -> RecursiveIterator<T, F> {
        RecursiveIterator::new(&self.func, self.start)
    }

    pub fn new(func: F, start: T) -> Self {
        Self { func, start }
    }
}

pub enum Behavior<T> {
    Convergence(T),
    Cycle(Vec<T>),
    Chaos,
}

impl<T> fmt::Display for Behavior<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Behavior::Convergence(v) => write!(f, "Behavoir: Converges to {}", v),
            Behavior::Chaos => write!(f, "Behavior: Chaos!"),
            Behavior::Cycle(vec) => {
                write!(f, "Behavior: {}-Cycle", vec.len())?;
                // for ref val in vec {
                //     write!(f, "{}, ", val)?;
                // }

                Ok(())
            }
        }
    }
}

pub struct RecursiveIterator<'a, T, F>
where
    T: Copy,
    F: 'a + Fn(T) -> T,
{
    current: T,
    func: &'a F,
}

impl<'a, T, F> RecursiveIterator<'a, T, F>
where
    T: Copy,
    F: Fn(T) -> T,
{
    pub fn new(func: &'a F, start: T) -> Self {
        Self {
            func,
            current: start,
        }
    }
}

impl<'a, T, F> Iterator for RecursiveIterator<'a, T, F>
where
    T: Copy,
    F: Fn(T) -> T,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let temp = self.current;
        self.current = (self.func)(temp);
        Some(self.current)
    }
}

pub trait CloseEnough {
    fn close_enough_to(&self, other: &Self) -> bool;
}

impl CloseEnough for f64 {
    fn close_enough_to(&self, other: &Self) -> bool {
        const ROUNDING: f64 = 1000000.0;

        let other = (other * ROUNDING).round() / ROUNDING;
        let this = (self * ROUNDING).round() / ROUNDING;

        this == other
    }
}
