pub enum MaybeVec<T> {
    Empty,
    One(T),
    Vec(Vec<T>),
}

impl<T> Default for MaybeVec<T> {
    fn default() -> Self {
        Self::Empty
    }
}

impl<T> MaybeVec<T> {
    #[must_use]
    pub fn push(self, value: T) -> Self {
        match self {
            Self::Empty => Self::One(value),
            Self::One(previous) => Self::Vec(vec![previous, value]),
            Self::Vec(mut list) => {
                list.push(value);
                Self::Vec(list)
            }
        }
    }

    #[must_use]
    pub fn pop(self) -> (Self, Option<T>) {
        match self {
            Self::Empty => (Self::Empty, None),
            Self::One(previous) => (Self::Empty, Some(previous)),
            Self::Vec(mut list) => {
                let value = list.pop();
                (Self::Vec(list), value)
            }
        }
    }
}
