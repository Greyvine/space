#[derive(Clone, Debug)]
pub(crate) struct Tree<T, const CHILDREN: usize>;

impl<T, const CHILDREN: usize> Tree<T, CHILDREN> {

    pub fn add(
        &mut self,
        key: Key,
        value: T,
    ) {
        
    }

}

pub struct Key {
    
}

pub struct Node {

}