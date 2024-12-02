#[derive(Debug, Clone, PartialEq)]
pub struct Node<T, M> {
    data: Box<T>,
    meta: M,
}

impl<T, M> Node<T, M> {
    pub fn new(data: T, meta: M) -> Self {
        Self {
            data: Box::new(data),
            meta,
        }
    }

    pub fn data(&self) -> &T {
        &self.data.as_ref()
    }

    pub fn meta(&self) -> &M {
        &self.meta
    }
}
