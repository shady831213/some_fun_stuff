pub trait Functor<'a, A> {
    type F<B>;
    fn fmap<B, F>(self, f: F) -> Self::F<B>
    where
        F: Copy + Fn(A) -> B + 'a;
}
impl<'a, A> Functor<'a, A> for Option<A> {
    type F<B> = Option<B>;
    fn fmap<B, F>(self, f: F) -> Self::F<B>
    where
        F: Copy + Fn(A) -> B + 'a,
    {
        self.map(|a| f(a))
    }
}
impl<'a, A, E> Functor<'a, A> for Result<A, E> {
    type F<B> = Result<B, E>;
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
