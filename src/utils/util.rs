pub struct Node<T> {
    pub(crate) data: T,
    pub(crate) right: Option<* mut Node<T>>,
    pub(crate) left: Option<* mut Node<T>>,
    pub(crate) count: i8,
}