use super::HGT;
pub trait Functor<'a, A>: HGT {
    fn fmap<B, F>(self, f: F) -> Self::F<B>
    where
        F: Copy + Fn(A) -> B + 'a;
}
impl<A> HGT for Option<A> {
    type F<T> = Option<T>;
}
impl<'a, A> Functor<'a, A> for Option<A> {
    fn fmap<B, F>(self, f: F) -> Self::F<B>
    where
        F: Copy + Fn(A) -> B + 'a,
    {
        self.map(|a| f(a))
    }
}
impl<A, E> HGT for Result<A, E> {
    type F<T> = Result<T, E>;
}
impl<'a, A, E> Functor<'a, A> for Result<A, E> {
    fn fmap<B, F>(self, f: F) -> Self::F<B>
    where
        F: Copy + Fn(A) -> B + 'a,
    {
        self.map(|a| f(a))
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn functor_test() {
        let r = Some(1).fmap(|i: usize| i.to_string()).fmap(|i| Some(i));
        println!("{:?}", r);
    }
}
