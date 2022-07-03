pub mod tree;
pub mod array;

pub(crate) trait Insert<T> {
    fn insert(&mut self, data: T);
    fn insert_vec(&mut self, data: Vec<T>);
}



pub(crate) trait Remove<T> {
    fn remove(&mut self, data: T);
}

pub(crate) trait Contains<T> {
    fn contains(&self, data: T) -> bool;
}

pub(crate) trait Traversal<T> {
    fn pre_order(&self) -> Vec<T>;
    fn in_order(&self) -> Vec<T>;
    fn post_order(&self) -> Vec<T>;
    fn bfs(&self) -> Vec<T>;
}
