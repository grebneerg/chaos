use std::iter::Iterator;
use std::fmt;

const ROUNDING: f64 = 1000000.0;

fn main() {
    let mut cycles = Vec::new();

    for a in 2200..4000 {
        let a: f64 = f64::from(a) * 0.001;
        let func = RecursiveFunction {
            func: |x: f64| a * x * (1.0 - x),
            start: 0.8,
        };

        cycles.push(func.end_behavior(50));
    }

    cycles
        .iter()
        .enumerate()
        .filter(|(_, x)| if let Behavior::Cycle(ref v) = x {
            v.len() == 3
        } else {
            false
        })
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
    fn end_behavior(&self, max_cycles: usize) -> Behavior<T> {
        let mut vals = Vec::new();
        for val in RecursiveIterator::new(&self.func, self.start).skip(1000).take(max_cycles) {
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
}

enum Behavior<T> {
    Convergence(T),
    Cycle(Vec<T>),
    Chaos,
}

impl<T> fmt::Display for Behavior<T> where T: fmt::Display {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Behavior::Convergence(v) => write!(f, "Behavoir: Converges to {}", v),
            Behavior::Chaos => write!(f, "Behavior: Chaos!"),
            Behavior::Cycle(vec) => {
                write!(f, "Behavior: {}-Cycle at these values: ", vec.len());
                for ref T in vec {
                    write!(f, "{}, ", T);
                }
                
                Ok(())
            }
        }
    }
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

impl CloseEnough for f64 {
    fn close_enough_to(&self, other: &Self) -> bool {
        let other = (other * ROUNDING).round() / ROUNDING;
        let this = (self * ROUNDING).round() / ROUNDING;
        
        this == other
    }
}
