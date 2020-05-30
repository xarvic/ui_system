use std::ops::{Deref, DerefMut};


/// PoolTree is a structure which stores a Tree inside one allocation using pool allocation inside
/// the buffer, the root Node is always present and located at index 0.
///
/// Since the tree can not contain cycles, it is safe to have mutable references to a node and its
/// Children
pub struct PoolTree<T> {
    buffer: Vec<Item<T>>,
}

impl<T> PoolTree<T> {
    pub fn new(root: T) -> Self {
        PoolTree {
            buffer: vec![
                Item{
                    state: State::Used,
                    value: root,
                    //TODO: change this, its not true, but it work for the moment
                    parent: 0,
                    childs: Vec::new(),
                }
            ]
        }
    }

    pub fn get_root_mut(&mut self) -> NodeTop<T>{
        //This is ok since the PoolTree enforces the rootNode to be present
        unsafe {self.get_unchecked_mut(0)}
    }

    pub fn get_mut(&mut self, index: usize) -> Option<NodeTop<T>> {
        if let Some(item) = self.buffer.get(index) {
            if let State::Used = item.state {
                return Some(unsafe{self.get_unchecked_mut(index)});
            }
        }
        None
    }
    pub unsafe fn get_unchecked_mut(&mut self, index: usize) -> NodeTop<T> {
        let rest = self as *mut Self;
        NodeTop { inner: NodeMut {
            current: unsafe{self.buffer.get_unchecked_mut(index)},
            rest,
            index,
        }}
    }
}

/// Represents the allocation state of the Node
/// - Used means present in the tree
/// - Free means free to alocate, refering to the next free
enum State {
    Used,
    Free(usize),
}

/// The in Buffer representation of a Node,
/// containing the value and indices of parent and childern
struct Item<T> {
    state: State,
    value: T,
    parent: usize,
    childs: Vec<usize>
}

/// a Node directly returned by the PoolTree
/// it is guaranteed to be the topmost reference into the PoolTree
/// therefore with this you can make changes to the tree structure
pub struct NodeTop<'a, T> {
    inner: NodeMut<'a, T>,
}

impl<'a, T> NodeTop<'a, T> {
    pub fn to_parent(self) -> Option<NodeTop<'a, T>> {
        if self.inner.index == 0 {
            None
        } else {
            Some(unsafe { (&mut *self.inner.rest).get_unchecked_mut(self.inner.current.parent) })
        }
    }
    pub fn to_child(self, index: usize) -> Option<NodeTop<'a, T>> {
        unimplemented!()
    }
    pub fn add_child(&mut self, index: usize, value: T) -> NodeMut<'a, T> {
        unimplemented!()
    }
    pub fn swap_children(&mut self, index1: usize, index2: usize) -> bool {
        unimplemented!()
    }
}

impl<'a, T> Deref for NodeTop<'a, T> {
    type Target = NodeMut<'a, T>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<'a, T> DerefMut for NodeTop<'a, T> {
    fn deref_mut(&mut self) -> &mut NodeMut<'a, T>{
        &mut self.inner
    }
}

pub struct NodeMut<'a, T> {
    current: &'a mut Item<T>,
    rest: *mut PoolTree<T>,
    index: usize,
}

impl<'a, T> NodeMut<'a, T> {

}

impl<'a, T> Deref for NodeMut<'a, T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.current.value
    }
}

impl<'a, T> DerefMut for NodeMut<'a, T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.current.value
    }
}

pub struct Node<'a, T> {
    current: &'a mut Item<T>,
    rest: *mut PoolTree<T>,
    index: usize,
}
