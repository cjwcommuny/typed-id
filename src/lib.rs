use std::fmt::{Debug, Formatter};
use std::marker::PhantomData;

/// Usage:
/// ```rust
/// use typed_id2::Id;
///
/// struct Customer {
///     name: String,
/// }
/// type CustomerId = Id<i32, Customer>;
///
/// let customer_id = CustomerId::new(1);
/// ```
pub struct Id<I, T> {
    id: I,
    phantom: PhantomData<fn() -> T>,
}

impl<I, T> Id<I, T> {
    pub fn new(id: I) -> Self {
        Self {
            id,
            phantom: PhantomData,
        }
    }

    pub fn inner(self) -> I {
        self.id
    }
}

impl<I, T> AsRef<I> for Id<I, T> {
    fn as_ref(&self) -> &I {
        &self.id
    }
}

impl<I: Debug, T> Debug for Id<I, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Id").field(&self.id).finish()
    }
}

impl<I: Clone, T> Clone for Id<I, T> {
    fn clone(&self) -> Self {
        Self {
            id: self.id.clone(),
            phantom: PhantomData,
        }
    }
}

impl<I: Copy, T> Copy for Id<I, T> {}

impl<I, T> From<I> for Id<I, T> {
    fn from(value: I) -> Self {
        Self::new(value)
    }
}

#[cfg(test)]
mod test {
    use crate::Id;

    trait IsSync: Sync {}

    impl<I: Sync, T> IsSync for Id<I, T> {}

    trait IsSend: Send {}

    impl<I: Send, T> IsSend for Id<I, T> {}
}
