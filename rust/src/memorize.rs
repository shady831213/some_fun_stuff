// category-theory-for-programmers challenge 2.7
use std::sync::Mutex;
struct MemorizedFunction<A: PartialEq, B, F: Fn(A) -> B + Copy> {
    f: F,
    his: Mutex<Vec<(A, B)>>,
}

impl<A: PartialEq, B, F: Fn(A) -> B + Copy> MemorizedFunction<A, B, F> {
    fn new(f: F) -> Self {
        MemorizedFunction {
            f,
            his: Mutex::new(vec![]),
        }
    }
    fn eval(&self, a: A) -> B
    where
        A: Copy,
        B: Copy,
    {
        let mut his = self.his.lock().unwrap();
        for (i, o) in his.iter() {
            if *i == a {
                println!("cached!");
                return *o;
            }
        }
        let b = (self.f)(a);
        println!("eval!");
        his.push((a, b));
        b
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::category::*;
    #[test]
    fn memorize_function_test() {
        let f = MemorizedFunction::new(|a| a * a);
        let g = MemorizedFunction::new(|b| b - 1);
        let g_f = Function::new(|a: usize| f.eval(a)).compose(Function::new(|b: usize| g.eval(b)));
        println!("r = {}", g_f.eval(2));
        println!("r = {}", g_f.eval(4));
        println!("r = {}", g_f.eval(2));
        println!("g_r = {}", g.eval(16));
    }
    #[test]
    fn memorize_random_test() {
        let f = MemorizedFunction::new(|_| rand::random::<usize>());
        println!("r = {}", f.eval(()));
        println!("r = {}", f.eval(()));
        println!("r = {}", f.eval(()));
    }
}
