use super::functor::*;
pub trait Monad: Functor {
    fn pure(a: <Self as Functor>::A) -> Self;
    fn join<B>(b: <Self as Functor>::T<<Self as Functor>::T<B>>) -> <Self as Functor>::T<B>;
    fn bind<B, F: Fn(Self::A) -> <Self as Functor>::T<B>>(self, f: F) -> <Self as Functor>::T<B>
    where
        Self: Sized,
    {
        Self::join(self.fmap(f))
    }
    fn then<B>(&self, b: <Self as Functor>::T<B>) -> <Self as Functor>::T<B> {
        b
    }
}
impl<A> Monad for Option<A> {
    fn pure(a: <Self as Functor>::A) -> Self {
        Some(a)
    }
    fn join<B>(b: <Self as Functor>::T<<Self as Functor>::T<B>>) -> <Self as Functor>::T<B> {
        b.flatten()
    }
}
impl<A, E> Monad for Result<A, E> {
    fn pure(a: <Self as Functor>::A) -> Self {
        Ok(a)
    }
    fn join<B>(b: <Self as Functor>::T<<Self as Functor>::T<B>>) -> <Self as Functor>::T<B> {
        b?
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
