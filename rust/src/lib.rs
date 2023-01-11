#![feature(type_alias_impl_trait)]
pub trait HGT {
    type F<T>;
}
mod category;
mod functor;
mod monad;
pub mod state;
