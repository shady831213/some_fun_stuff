// category-theory-for-programmers challenge 1.4
// No way to impl Kleisili Category...
trait Morphism<A, B> {
    type Output<G: Copy + Morphism<B, C>, C>;
    // fn id<T>() -> Self::M<T, T>;
    fn compose<G, C>(self, g: G) -> Self::Output<G, C>
    where
        G: Morphism<B, C> + Copy;
    fn eval(self, a: A) -> B;
}
trait Category {
    type M<T>: Morphism<T, T> + Copy;
    fn id<T>() -> Self::M<T>;
}

impl<A, B, F: Fn(A) -> B + Copy> Morphism<A, B> for F {
    type Output<G: Copy + Morphism<B, C>, C> = impl Fn(A) -> C + Copy;
    fn compose<G, C>(self, g: G) -> Self::Output<G, C>
    where
        G: Morphism<B, C> + Copy,
    {
        move |a| g.eval(self.eval(a))
    }
    fn eval(self, a: A) -> B {
        self(a)
    }
}

struct Set;
impl Category for Set {
    type M<T> = impl Fn(T) -> T + Copy;
    fn id<T>() -> Self::M<T> {
        move |a: T| a
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn id_test() {
        let a = 1;
        assert_eq!(a, Set::id().eval(a));
    }

    #[test]
    fn associative_test() {
        let f = |a: u32| -> f32 { (a + 3) as f32 };
        let g = |a: f32| -> f64 { (a / 8.0) as f64 };
        let h = |a: f64| -> u64 { (a * a) as u64 };
        let l_ass = f.compose(g).compose(h);
        let r_ass = f.compose(g.compose(h));
        let check = |a: u32| {
            println!(
                "l_ass.eval({}) = {}, r_ass.eval({}) = {}",
                a,
                l_ass.eval(a),
                a,
                r_ass.eval(a)
            );
            assert_eq!(l_ass.eval(a), r_ass.eval(a));
        };
        check(1);
        check(16);
        check(29);
    }

    #[test]
    fn uint_test() {
        let f = |a: u32| -> f32 { (a + 3) as f32 };
        let l_unit = Set::id().compose(f);
        let r_unit = f.compose(Set::id());
        let check = |a: u32| {
            println!(
                "l_unit.eval({}) = {}, r_unit.eval({}) = {}, f({}).eval = {}",
                a,
                l_unit.eval(a),
                a,
                r_unit.eval(a),
                a,
                f(a)
            );
            assert_eq!(l_unit.eval(a), r_unit.eval(a));
            assert_eq!(l_unit.eval(a), f(a));
            assert_eq!(r_unit.eval(a), f(a));
        };
        check(1);
        check(16);
        check(29);
    }
}
