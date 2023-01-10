// category-theory-for-programmers challenge 1.4
// No way to impl Kleisili Category...
trait Morphism<A, B> {
    type Eval;
    type Output<G: Copy + Morphism<B, C>, C>;
    // fn id<T>() -> Self::M<T, T>;
    fn compose<G, C>(self, g: G) -> Self::Output<G, C>
    where
        G: Morphism<B, C> + Copy;
    fn eval(self, a: A) -> Self::Eval;
}
trait Category {
    type M<T: Copy>: Morphism<T, T> + Copy;
    fn id<T: Copy>() -> Self::M<T>;
}
#[derive(Copy, Clone)]
struct Function<A, B, F: Fn(A) -> B + Copy> {
    f: F,
    _marker: std::marker::PhantomData<(A, B)>,
}

impl<A, B, F: Fn(A) -> B + Copy> Function<A, B, F> {
    fn new(f: F) -> Self {
        Function {
            f,
            _marker: std::marker::PhantomData,
        }
    }
}

impl<A: Copy, B: Copy, F: Fn(A) -> B + Copy> Morphism<A, B> for Function<A, B, F> {
    type Eval = B;
    type Output<G: Copy + Morphism<B, C>, C> =
        Function<A, <G as Morphism<B, C>>::Eval, impl Fn(A) -> <G as Morphism<B, C>>::Eval + Copy>;
    fn compose<G, C>(self, g: G) -> Self::Output<G, C>
    where
        G: Morphism<B, C> + Copy,
    {
        Function::new(move |a| g.eval(self.eval(a)))
    }
    fn eval(self, a: A) -> Self::Eval {
        (self.f)(a)
    }
}

struct Set;
impl Category for Set {
    type M<T: Copy> = Function<T, T, impl Fn(T) -> T + Copy>;
    fn id<T: Copy>() -> Self::M<T> {
        Function::new(move |a: T| a)
    }
}

// use super::monad::Monad;
// #[derive(Copy, Clone)]
// struct KleisiliArrow<A, B, M: for<'a> Monad<'a, B>, F: Fn(A) -> M + Copy> {
//     f: F,
//     _marker: std::marker::PhantomData<(A, B, M)>,
// }

// impl<A, B, M: for<'a> Monad<'a, B>, F: Fn(A) -> M + Copy> KleisiliArrow<A, B, M, F> {
//     fn new(f: F) -> Self {
//         KleisiliArrow {
//             f,
//             _marker: std::marker::PhantomData,
//         }
//     }
// }

// impl<A, B, M: for<'a> Monad<'a, B>, F: Fn(A) -> M + Copy> Morphism<A, B>
//     for KleisiliArrow<A, B, M, F>
// {
//     type Eval = M;
//     type Output<G: Copy + Morphism<B, C>, C> = KleisiliArrow<
//         A,
//         C,
//         <G as Morphism<B, C>>::Eval,
//         impl Fn(A) -> <G as Morphism<B, C>>::Eval + Copy,
//     >;
//     fn compose<G, C>(self, g: G) -> Self::Output<G, C>
//     where
//         G: Morphism<B, C> + Copy,
//     {
//         KleisiliArrow::new(move |a| self.eval(a).bind(|b| g.eval(b)))
//     }
//     fn eval(self, a: A) -> Self::Eval {
//         (self.f)(a)
//     }
// }

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
        let f = Function::new(|a: u32| -> f32 { (a + 3) as f32 });
        let g = Function::new(|a: f32| -> f64 { (a / 8.0) as f64 });
        let h = Function::new(|a: f64| -> u64 { (a * a) as u64 });
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
        let f = Function::new(|a: u32| -> f32 { (a + 3) as f32 });
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
                f.eval(a)
            );
            assert_eq!(l_unit.eval(a), r_unit.eval(a));
            assert_eq!(l_unit.eval(a), f.eval(a));
            assert_eq!(r_unit.eval(a), f.eval(a));
        };
        check(1);
        check(16);
        check(29);
    }
}
