pub struct Node<T> {
    pub(crate) data: T,
    pub(crate) right: Option<* mut Node<T>>,
    pub(crate) left: Option<* mut Node<T>>,
    pub(crate) count: i8,
}

impl<T> Default for Node<T> {
    fn default() -> Self {
        Node {
            data: 1,
            right: None,
            left: None,
            count: 0
        }
    }
}