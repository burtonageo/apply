#![deny(missing_docs)]

//! A tiny library for chaining free functions into method call chains.

/// Represents a type which can have functions applied to it (implemented
/// by default for all types).
pub trait Apply<Res> {
    /// Apply a function which takes the parameter by value.
    fn apply<F: Fn(Self) -> Res>(self, f: F) -> Res where Self: Sized {
        f(self)
    }

    /// Apply a function which takes the parameter by reference.
    fn apply_ref<F: Fn(&Self) -> Res>(&self, f: F) -> Res {
        f(self)
    }

    /// Apply a function which takes the parameter by mutable reference.
    fn apply_mut<F: Fn(&mut Self) -> Res>(&mut self, f: F) -> Res {
        f(self)
    }
}

impl<T, Res> Apply<Res> for T {
    // use default definitions...
}