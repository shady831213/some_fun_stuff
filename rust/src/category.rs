// category-theory-for-programmers challenge 1.4
trait Morphism {
    type A;
    type B;
}
trait Category {
    type M<'a, A: 'a, B: 'a>: Morphism<A = A, B = B>;
    fn id<'a, T: 'a>() -> Self::M<'a, T, T>;
    fn compose<'a, A: 'a, B: 'a, C: 'a>(
        f: Self::M<'a, A, B>,
        g: Self::M<'a, B, C>,
    ) -> Self::M<'a, A, C>;
}

struct Function<'a, A, B>(Box<dyn 'a + Fn(A) -> B>);
impl<'a, A, B> Function<'a, A, B> {
    fn new<F: Fn(A) -> B + 'a>(f: F) -> Self {
        Function(Box::new(f))
    }
    fn run(&self, a: A) -> B {
        self.0(a)
    }
}
impl<'a, A, B> Morphism for Function<'a, A, B> {
    type A = A;
    type B = B;
}

struct Set;

impl Category for Set {
    type M<'a, A: 'a, B: 'a> = Function<'a, A, B>;
    fn id<'a, T: 'a>() -> Self::M<'a, T, T> {
        Function::new(|a| a)
    }
    fn compose<'a, A: 'a, B: 'a, C: 'a>(
        f: Self::M<'a, A, B>,
        g: Self::M<'a, B, C>,
    ) -> Self::M<'a, A, C> {
        Function::new(move |a| g.run(f.run(a)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn id_test() {
        let a = 1;
        assert_eq!(a, Set::id().run(a));
    }

    #[test]
    fn associative_test() {
        let f = |a: u32| -> f32 { (a + 3) as f32 };
        let g = |a: f32| -> f64 { (a / 8.0) as f64 };
        let h = |a: f64| -> u64 { (a * a) as u64 };
        let l_ass = Set::compose(
            Set::compose(Function::new(f), Function::new(g)),
            Function::new(h),
        );
        let r_ass = Set::compose(
            Function::new(f),
            Set::compose(Function::new(g), Function::new(h)),
        );
        let check = |a: u32| {
            println!(
                "l_ass.run({}) = {}, r_ass.run({}) = {}",
                a,
                l_ass.run(a),
                a,
                r_ass.run(a)
            );
            assert_eq!(l_ass.run(a), r_ass.run(a));
        };
        check(1);
        check(16);
        check(29);
    }

    #[test]
    fn uint_test() {
        let f = |a: u32| -> f32 { (a + 3) as f32 };
        let l_unit = Set::compose(Set::id(), Function::new(f));
        let r_unit = Set::compose(Function::new(f), Set::id());
        let check = |a: u32| {
            println!(
                "l_unit.run({}) = {}, r_unit.run({}) = {}, f({}).run = {}",
                a,
                l_unit.run(a),
                a,
                r_unit.run(a),
                a,
                f(a)
            );
            assert_eq!(l_unit.run(a), r_unit.run(a));
            assert_eq!(l_unit.run(a), f(a));
            assert_eq!(r_unit.run(a), f(a));
        };
        check(1);
        check(16);
        check(29);
    }
}
