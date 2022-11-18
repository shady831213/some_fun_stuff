pub trait Functor<'a> {
    type A;
    type T<B>;
    fn fmap<B, F: 'a + Fn(Self::A) -> B>(self, f: F) -> Self::T<B>;
}
impl<'a, A> Functor<'a> for Option<A> {
    type A = A;
    type T<B> = Option<B>;
    fn fmap<B, F: 'a + Fn(Self::A) -> B>(self, f: F) -> Self::T<B> {
        self.map(|a| f(a))
    }
}
impl<'a, A, E> Functor<'a> for Result<A, E> {
    type A = A;
    type T<B> = Result<B, E>;
    fn fmap<B, F: 'a + Fn(Self::A) -> B>(self, f: F) -> Self::T<B> {
        self.map(|a| f(a))
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn functor_test() {
        let r = Some(1).fmap(|i| i.to_string()).fmap(|i| Some(i));
        println!("{:?}", r);
    }
}
