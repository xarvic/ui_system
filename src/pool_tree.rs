use std::fmt::{Display, Error, Formatter};
use std::ops::{Deref, DerefMut};
use std::slice::Iter;

/// PoolTree is a structure which stores a Tree inside one allocation using pool allocation inside
/// the buffer, with unchanging IDs
///
/// the root Node is always present and located at index 0.
///
/// Since the tree can not contain cycles, it is safe to have mutable references to a node and its
/// Children
#[derive(Debug)]
pub struct PoolTree<T> {
    buffer: Vec<Item<T>>,
    next_free: Option<usize>,
}

impl<T> PoolTree<T> {
    pub fn new(root: T) -> Self {
        PoolTree {
            buffer: vec![
                Item {
                    used: true,
                    value: root,
                    //TODO: change this, its not true, but it works for the moment
                    parent: None,
                    childs: Vec::new(),
                }
            ],
            next_free: None,
        }
    }

    pub fn root_mut(&mut self) -> NodeTop<T> {
        //This is ok since the PoolTree enforces the rootNode to be present
        unsafe { self.get_unchecked_mut(0) }
    }

    pub fn get_mut(&mut self, index: usize) -> Option<NodeTop<T>> {
        if let Some(item) = self.buffer.get(index) {
            if item.used {
                return Some(unsafe { self.get_unchecked_mut(index) });
            }
        }
        None
    }
    pub unsafe fn get_unchecked_mut(&mut self, index: usize) -> NodeTop<T> {
        let rest = self as *mut Self;
        NodeTop {
            inner: NodeMut {
                current: self.buffer.get_unchecked_mut(index),
                rest,
                index,
            }
        }
    }
    pub fn root(&self) -> Node<T> {
        //This is ok since the PoolTree enforces the rootNode to be present
        unsafe { self.get_unchecked(0) }
    }

    pub fn get(&self, index: usize) -> Option<Node<T>> {
        if let Some(item) = self.buffer.get(index) {
            if item.used {
                return Some(unsafe { self.get_unchecked(index) });
            }
        }
        None
    }
    pub unsafe fn get_unchecked(&self, index: usize) -> Node<T> {
        Node {
            current: self.buffer.get_unchecked(index),
            rest: self,
            index,
        }
    }
    //May invalidate all pointers to the Buffer
    unsafe fn alloc_for(&mut self, value: T, parent: usize) -> (usize, &mut Item<T>){
        if let Some(index) = self.next_free {
            //get next free
            let buf = self.buffer.get_unchecked_mut(index);
            //parent gets reused for freed elements as linked list to store next free elements
            self.next_free = buf.parent;

            //Init Item
            buf.used = true;
            buf.value = value;
            buf.parent = Some(parent);

            (index, buf)
        } else {
            //all values are used => allocate a new one
            self.buffer.push(Item::new(value, parent));
            let index = self.buffer.len() - 1;
            (index, self.buffer.get_unchecked_mut(index))
        }
    }
    unsafe fn free(&mut self, index: usize) {
        let item = self.buffer.get_mut(index).expect("free non exsiting value!");
        item.used = false;
        item.parent = self.next_free;
        self.next_free = Some(index);
    }
}

impl<T: Display> Display for PoolTree<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        self.root().fmt(f)
    }
}

/// The in Buffer representation of a Node,
/// containing the value and indices of parent and childern
/// since this is the Buffer of the pool allocator, it should be a union, but since unions
/// with non copy fields arent stable at the time, the parent field gets reused to store the next
/// free index whereas parent == 0 means there are no next free Nodes, null cant be
#[derive(Debug)]
pub struct Item<T> {
    used: bool,
    value: T,
    parent: Option<usize>,
    childs: Vec<usize>,
}

impl<T> Item<T> {
    pub fn new(value: T, parent: usize) -> Item<T> {
        Item {
            used: false,
            value,
            parent: Some(parent),
            childs: Vec::new()
        }
    }
}
impl<T: Display> Item<T> {
    fn print(&self, f: &mut Formatter<'_>, rest: *const PoolTree<T>) -> Result<(), Error> {
        self.value.fmt(f)?;
        if !self.childs.is_empty() {
            let mut it = self.childs.iter();
            f.write_str("{")?;
            //childs is not empty first element exsists
            unsafe { (&*rest).buffer.get_unchecked(*it.next().unwrap()).print(f, rest)?; }
            for i in it {
                f.write_str(", ")?;
                unsafe { (&*rest).buffer.get_unchecked(*i).print(f, rest)?; }
            }

            f.write_str("}")?;
        }
        Ok(())
    }
}

/// a Node directly returned by the PoolTree
/// it is guaranteed to be the topmost reference into the PoolTree
/// therefore with this Node you can make changes to the tree structure
#[derive(Debug)]
pub struct NodeTop<'a, T> {
    inner: NodeMut<'a, T>,
}

impl<'a, T> NodeTop<'a, T> {
    pub fn to_parent(self) -> Option<NodeTop<'a, T>> {
        self.inner.current.parent
            .map(|index|unsafe { (&mut *self.inner.rest).get_unchecked_mut(index) })

    }
    pub fn to_child(self, index: usize) -> Option<NodeTop<'a, T>> {
        self.current.childs.get(index)
            .map(|index|unsafe { (&mut *self.inner.rest).get_unchecked_mut(*index) })
    }
    pub fn add_child(&mut self, value: T) -> NodeMut<'a, T> {
        //After this self.current is invalid
        let (index, item) = unsafe{ (&mut *self.rest).alloc_for(value, self.index)};
        *self = unsafe{(&mut *self.rest).get_unchecked_mut(self.index)};
        self.current.childs.push(index);
        NodeMut{
            current: item,
            index,
            rest: self.rest,
        }
    }
    pub fn as_mut(self) -> NodeMut<'a, T> {
        self.inner
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
    fn deref_mut(&mut self) -> &mut NodeMut<'a, T> {
        &mut self.inner
    }
}

#[derive(Debug)]
pub struct NodeMut<'a, T> {
    current: &'a mut Item<T>,
    rest: *mut PoolTree<T>,
    index: usize,
}

impl<'a, T> NodeMut<'a, T> {
    #[inline(always)]
    pub fn child_mut(&mut self, index: usize) -> Option<NodeMut<T>> {
        self.current.childs.get(index)
            .map(|index|unsafe{(&mut *self.rest).get_unchecked_mut(*index).inner})
    }
    #[inline(always)]
    pub fn childs_mut(&mut self) -> ChildIterMut<T> {
        ChildIterMut{
            inner: self.current.childs.iter(),
            buffer: self.rest,
        }
    }
    #[inline(always)]
    pub fn this(&mut self) -> NodeMut<T> {
        NodeMut{
            current: self.current,
            rest: self.rest,
            index: self.index,
        }
    }
    #[inline(always)]
    pub fn childs(&self) -> ChildIter<T> {
        ChildIter{
            inner: self.current.childs.iter(),
            buffer: unsafe{ &*self.rest },
        }
    }
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

pub struct ChildIterMut<'a, T: 'a> {
    inner: Iter<'a, usize>,
    buffer: *mut PoolTree<T>,
}

impl<'a, T> Iterator for ChildIterMut<'a, T> {
    type Item = NodeMut<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|index|unsafe{ (&mut *self.buffer).get_unchecked_mut(*index).inner})
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }

    fn count(self) -> usize where
        Self: Sized, {
        self.inner.count()
    }

    fn last(self) -> Option<Self::Item> where
        Self: Sized, {
        let buffer = self.buffer;
        self.inner.last().map(|index|unsafe{ (&mut *buffer).get_unchecked_mut(*index).inner})
    }
}

#[derive(Debug)]
pub struct Node<'a, T> {
    current: &'a Item<T>,
    rest: &'a PoolTree<T>,
    index: usize,
}

impl<'a, T> Node<'a, T> {
    pub fn child(&self, index: usize) -> Option<Node<T>> {
        self.current.childs.get(index)
            .map(|index|unsafe{self.rest.get_unchecked(*index) })

    }

    pub fn parent(&self) -> Option<Node<T>> {
        self.current.parent
            .map(|index|unsafe{ self.rest.get_unchecked(index) })
    }
    pub fn child_count(&self) -> usize {
        self.current.childs.len()
    }
    pub fn childs(&self) -> ChildIter<T> {
        ChildIter{
            inner: self.current.childs.iter(),
            buffer: self.rest,
        }
    }
}

impl<'a, T> Deref for Node<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.current.value
    }
}

impl<'a, T: Display> Display for Node<'a, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        self.current.print(f, self.rest)
    }
}

pub struct ChildIter<'a, T: 'a> {
    inner: Iter<'a, usize>,
    buffer: &'a PoolTree<T>,
}

impl<'a, T> Iterator for ChildIter<'a, T> {
    type Item = Node<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|index|unsafe{ self.buffer.get_unchecked(*index)})
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }

    fn count(self) -> usize where
        Self: Sized, {
        self.inner.count()
    }

    fn last(self) -> Option<Self::Item> where
        Self: Sized, {
        let buffer = self.buffer;
        self.inner.last().map(|index|unsafe{ buffer.get_unchecked(*index)})
    }
}