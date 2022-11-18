use super::functor::*;
pub trait Monad<'a>: Functor<'a> {
    fn pure(a: Self::A) -> Self;
    // fn join<B>(b: Self::T<Self::T<B>>) -> Self::T<B>;
    // fn bind<B, F: Fn(Self::A) -> Self::T<B>>(self, f: F) -> Self::T<B>
    // where
    //     Self: Sized,
    // {
    //     Self::join(self.fmap(f))
    // }
    fn bind<B, F: 'a + Fn(Self::A) -> Self::T<B>>(self, f: F) -> Self::T<B>;
    fn then<B>(&self, b: Self::T<B>) -> Self::T<B> {
        b
    }
}
impl<'a, A> Monad<'a> for Option<A> {
    fn pure(a: Self::A) -> Self {
        Some(a)
    }
    fn bind<B, F: 'a + Fn(Self::A) -> Self::T<B>>(self, f: F) -> Self::T<B> {
        self.fmap(f).flatten()
    }
}
impl<'a, A, E> Monad<'a> for Result<A, E> {
    fn pure(a: Self::A) -> Self {
        Ok(a)
    }
    fn bind<B, F: 'a + Fn(Self::A) -> Self::T<B>>(self, f: F) -> Self::T<B> {
        self.fmap(f)?
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn monad_test() {
        let r = Result::<_, String>::pure(1)
            .bind(|i| {
                println!("1st bind {}!", i);
                Ok(i.to_string())
            })
            .bind(|i| {
                println!("2nd bind {}!", i);
                Ok(())
            })
            .then(Result::pure(2))
            .bind(|i| {
                println!("3st bind {}!", i);
                Ok(i.to_string())
            })
            .bind(|i| {
                println!("4th bind {}!", i);
                Ok(())
            });
        println!("{:?}", r);
    }
}
