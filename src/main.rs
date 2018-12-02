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
    for num in RecursiveIterator::new(func, start).skip(1000).take(50) {
        let num = (num * ROUNDING).round() / ROUNDING;
        if !nums.contains(&num) {
            nums.push(num);
        }
    }
    nums.len()
}

enum Behavior<T> {
    Convergence(T),
    Cycle(Vec<T>),
}

struct RecursiveIterator<T, F>
where
    T: Copy,
    F: Fn(T) -> T,
{
    current: T,
    func: F,
}

impl<T, F> RecursiveIterator<T, F>
where
    T: Copy,
    F: Fn(T) -> T,
{
    fn new(func: F, start: T) -> Self {
        Self {
            func,
            current: start,
        }
    }
}

impl<T, F> Iterator for RecursiveIterator<T, F>
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
