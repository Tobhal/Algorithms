pub mod tree;
pub mod array;

pub(crate) trait Insert<T> {
    fn insert(&mut self, data: T);
    fn insert_vec(&mut self, data: Vec<T>);
}

pub(crate) trait Counting {
    fn num_nodes(&self) -> u32;
    fn num_leaves(&self) -> u32;
    fn num_two_children(&self) -> u32;
    fn num_levels(&self) -> u32;
}

pub(crate) trait Remove<T> {
    fn remove(&mut self, data: T);
}

pub(crate) trait Contains<T> {
    fn contains(&self, data: T) -> bool;
}
