/// Represents a type that you can apply arbitrary functions to.
/// Useful for when a method doesn't return the receiver but you want
/// to apply several of them to the object.
pub trait Also : Sized {

    /// Apply a function to this value and return the (possibly) modified value.
    fn also<F: FnOnce(&mut Self)>(mut self, block: F) -> Self {
        block(&mut self);
        self
    }
}

impl <T: Sized> Also for T {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_also() {
        let it = vec![3, 2, 1, 5]
            .also(|it| it.sort())
            .also(|it| it.push(7));
        assert_eq!(it, vec![1, 2, 3, 5, 7]);
    }
}
