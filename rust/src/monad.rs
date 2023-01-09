use super::functor::*;
pub trait Monad<'a, A>: Functor<'a, A> {
    fn pure(a: A) -> Self;
    fn bind<B, F>(self, f: F) -> Self::F<B>
    where
        F: Copy + Fn(A) -> Self::F<B> + 'a;
    fn then<B>(&self, b: Self::F<B>) -> Self::F<B> {
        b
    }
}
impl<'a, A> Monad<'a, A> for Option<A> {
    fn pure(a: A) -> Self {
        Some(a)
    }
    fn bind<B, F>(self, f: F) -> Self::F<B>
    where
        F: Copy + Fn(A) -> Self::F<B> + 'a,
    {
        self.fmap(f).flatten()
    }
}
impl<'a, A, E> Monad<'a, A> for Result<A, E> {
    fn pure(a: A) -> Self {
        Ok(a)
    }
    fn bind<B, F>(self, f: F) -> Self::F<B>
    where
        F: Copy + Fn(A) -> Self::F<B> + 'a,
    {
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
