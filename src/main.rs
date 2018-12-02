use std::iter::Iterator;

const ROUNDING: f64 = 1000000.0;

fn main() {
    let mut cycles = Vec::new();

    for a in 2200..4000 {
        let a: f64 = f64::from(a) * 0.001;
        let func = |x: f64| a * x * (1.0 - x);

        cycles.push(num_cycles(func, 0.8));
    }

    cycles
        .iter()
        .enumerate()
        .filter(|(_, x)| **x == 3)
        .for_each(|(i, x)| println!("{:.3}: {}", (i + 2200) as f64 * 0.001, x));
}

fn num_cycles<F: Fn(f64) -> f64>(func: F, start: f64) -> usize {
    let mut nums = Vec::new();
    for num in RecursiveIterator::new(&func, start).skip(1000).take(50) {
        let num = (num * ROUNDING).round() / ROUNDING;
        if !nums.contains(&num) {
            nums.push(num);
        }
    }
    nums.len()
}

struct RecursiveFunction<T, F>
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
    fn end_behavior(&self) -> Behavior<T> {
        let mut vals = Vec::new();
        for val in RecursiveIterator::new(&self.func, self.start).skip(1000).take(50) {
            if !vals.iter().any(|x: &T| x.close_enough_to(&val)) {
                vals.push(val);
            }
        }
        
        if vals.len() == 1 {
            Behavior::Convergence(vals[0])
        } else {
            Behavior::Cycle(vals)
        }
    }
}

enum Behavior<T> {
    Convergence(T),
    Cycle(Vec<T>),
}

struct RecursiveIterator<'a, T, F>
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
    fn new(func: &'a F, start: T) -> Self {
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

trait CloseEnough {
    fn close_enough_to(&self, other: &Self) -> bool;
}
