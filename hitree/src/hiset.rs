//use std::fmt::{Debug,Display,Formatter};

use std::borrow::{Borrow, BorrowMut, ToOwned};
use std::cmp::Ordering;
use super::tree_height;

pub struct HiSet<T: Ord> {
    root: Ref<T>,
}

struct Ref<T>
    where T: Ord
{
    count: usize,
    node: Option<Box<Node<T>>>,
}

struct Node<T>
    where T: Ord
{
    value: T,
    left: Ref<T>,
    right: Ref<T>,
}



impl <T> HiSet<T>
    where T: Ord
{
    /// Create new empty HiSet.
    ///
    /// Does not allocate anything.
    ///
    /// # Examples:
    ///
    /// ```
    /// # #[allow(unused_mut)]
    ///     use hitree::hiset::HiSet;
    ///     let mut set = HiSet::<String>::new();
    ///
    /// ```
    pub fn new() -> HiSet<T> {
        HiSet { root: Ref::default() }
    }

    /// Return current number of entries in the set.
    ///
    /// Extremely cheap.
    ///
    /// # Examples:
    ///
    /// ```
    ///     use hitree::hiset::HiSet;
    ///
    ///     let hiset = HiSet::<i32>::new();
    ///     assert_eq!(hiset.len(), 0);
    ///
    /// ```
    pub fn len(&self) -> usize {
        self.root.count
    }


    /// Insert a new value into the set.
    /// If the value was not in the set, return true.
    /// If the value was already in the set, return false and don't touch the old value.
    /// Value can be any type that can be converted into the value type using Into trait.
    ///
    /// # Examples:
    ///
    /// ```
    ///     use hitree::hiset::HiSet;
    ///     let mut hiset = HiSet::<i32>::new();
    ///     assert_eq!(hiset.insert(1), true);
    ///     assert_eq!(hiset.insert(2), true);
    ///     assert_eq!(hiset.insert(1), false);
    ///     assert_eq!(hiset.len(), 2);
    /// ```
    /// You can insert &str into HiSet<String> for example:
    /// ```
    ///     use hitree::hiset::HiSet;
    ///     let mut hiset = HiSet::<String>::new(); // This is a set of Strings
    ///     assert_eq!(hiset.insert("This can be converted to a String"), true);
    /// ```
    pub fn insert(&mut self, value: impl Into<T>) -> bool {
        self.root.insert(Node::new(value))
    }


    /// Get a borrowed value from set by index.
    /// Values in the set are sorted according to their Ord trait,
    /// index 0 is the smallest value.
    /// Borrowed value can be any reference type that can be borrowed from T.
    /// You can use it to borrow &str from HiSet<String> for example.
    ///
    /// # Examples:
    ///
    /// ```
    ///     use hitree::hiset::HiSet;
    ///
    ///     let mut hiset = HiSet::<String>::new();
    ///     hiset.insert("This");
    ///     hiset.insert("is");
    ///     hiset.insert("a");
    ///     hiset.insert("test!");
    ///
    ///     // you can ask for Option<&str>
    ///     assert_eq!(hiset.get_by_index::<str>(0), Some("This"));
    ///     // or Option<&String> if you want, whatever you can borrow from the T type.
    ///     assert_eq!(hiset.get_by_index::<String>(1), Some(&"a".to_string()));
    ///     assert_eq!(hiset.get_by_index::<str>(2), Some("is"));
    ///     assert_eq!(hiset.get_by_index::<str>(3), Some("test!"));
    ///     assert_eq!(hiset.get_by_index::<str>(4), None );
    /// ```
    ///

    pub fn get_by_index<B>(&self, index: usize) -> Option<&B>
        where T: Borrow<B>,
              B: ?Sized + ToOwned<Owned=T>
    {
        let mut index_to_find = index;
        let mut current_node = self.root.node();
        loop {
            match current_node {
                None => return None,
                Some(node) => {
                    match node.left.count.cmp(&index_to_find) {
                        Ordering::Greater => {
                            // index must be in the left subtree
                            current_node = node.left.node();
                        },
                        Ordering::Equal => {
                            // found it, its this node
                            return Some(node.borrow_value())
                        },
                        Ordering::Less => {
                            // index must be in the right subtree
                            index_to_find = index_to_find - 1 - node.left.count;
                            current_node = node.right.node();
                        }
                    }
                }
            }
        }
    }
}

impl <T> Ref<T>
    where T: Ord
{

    pub fn to(node: Box<Node<T>>) -> Ref<T> {
        let count = 1 + node.left.count + node.right.count;
        Ref { count, node: Some(node) }
    }


    fn node(&self) -> Option<&Node<T>> {
        self.node.as_deref()
    }

    fn node_mut(&mut self) -> Option<&mut Node<T>> {
        self.node.as_deref_mut()
    }


    fn take(&mut self) -> Ref<T> {
        std::mem::take(&mut *self)
    }

    fn take_left(&mut self) -> Ref<T> {
        match self.node_mut() {
            None => Ref::default(),
            Some(node) => {
                let left = node.left.take();
                self.count -= left.count;
                left
            },
        }
    }

    fn take_right(&mut self) -> Ref<T> {
        match self.node_mut() {
            None => Ref::default(),
            Some(node) => {
                let right = node.right.take();
                self.count -= right.count;
                right
            },
        }
    }

    #[inline]
    fn left_count(&self) -> usize {
        match self.node() {
            None => 0,
            Some(node) => node.left.count,
        }
    }

    #[inline]
    fn right_count(&self) -> usize {
        match self.node() {
            None => 0,
            Some(node) => node.right.count,
        }
    }

    #[inline]
    fn balance(&self) -> isize {
        self.node.as_deref().unwrap().balance()     // balance only makes sense if there is a node, hence unwrap()
    }


    fn set_left(&mut self, subtree: Ref<T>) {
        let node = self.node_mut().unwrap();
        node.left = subtree;
        self.count = node.count();
    }

    fn set_right(&mut self, subtree: Ref<T>) {
        let node = self.node_mut().unwrap();
        node.right = subtree;
        self.count = node.count();
    }

    /*
                self                                                       self
                  |                                                          |
              [old_root]                                                 [new_root]
              /         \                       ---->                   /          \
   [left_subtree]       [new_root]                               [old_root]       [right_subtree]
                        /         \                             /         \
               [mid_subtree]   [right_subtree]        [left_subtree]   [mid subtree]

    */
    #[inline]
    fn rotate_left(&mut self) {
        let mut old_root = self.take();
        let mut new_root = old_root.take_right();
        let mid_subtree = new_root.take_left();
        old_root.set_right(mid_subtree);
        new_root.set_left(old_root);
        *self = new_root;
    }

    /*
                   self                                              self
                     |                                                 |
                 [old_root]                                        [new_root]
                /         \                  ---->                /          \
          [new_root]     [right_subtree]                 [left_subtree]      [old_root]
         /          \                                                        /         \
[left_subtree]  [mid_subtree]                                        [mid subtree]   [right_subtree]

    */
    #[inline]
    fn rotate_right(&mut self) {
        let mut old_root = self.take();
        let mut new_root = old_root.take_left();
        let mid_subtree = new_root.take_right();
        old_root.set_left(mid_subtree);
        new_root.set_right(old_root);
        *self = new_root;
    }




    fn insert(&mut self, new_node: Box<Node<T>>) -> bool {
        match self.node_mut() {
            None => {   // there are no nodes in subtree rooted at this Ref.
                *self = Ref::to(new_node);
                true    // we have inserted a value, return true
            },
            Some(node) => {     // There is at least one node
                match Ord::cmp(node,&new_node.as_ref()) {
                    Ordering::Equal => {
                        false   // already in there, return false
                    },
                    Ordering::Less => { // insert into right subtree
                        if node.right.insert(new_node) {
                            self.count += 1;    // increase number of entries for subtree
                            if self.balance() > 1 { // too right heavy
                                // difference in height has become greater than 1, rotate subtree left
                                self.rotate_left();
                            }
                            true
                        } else {
                            false
                        }
                    },
                    Ordering::Greater => {
                        if node.left.insert(new_node) {
                            self.count += 1;    // increase number of entries for subtree
                            if self.balance() < -1 {    // too left heavy
                                // difference in height has become greater than 1, rotate subtree left
                                self.rotate_right();
                            }
                            true
                        } else {
                            false
                        }
                    }
                }





            }
        }
    }
}

impl <T> Default for Ref<T>
    where T: Ord
{
    fn default() -> Self {
        Self { count: 0, node: None }
    }
}

impl <T> Node<T>
    where T: Ord
{

    fn new(value: impl Into<T>) -> Box<Node<T>> {
        Box::new( Node { value: value.into(), left: Ref::default(), right: Ref::default() } )
    }

    fn count(&self) -> usize {
        self.left.count + self.right.count + 1
    }

    // returns difference in height between right and left subtrees. >0 right is bigger, <0 left is bigger.
    #[inline]
    fn balance(&self) -> isize {
        tree_height(self.right.count) - tree_height(self.left.count)
    }

    pub fn borrow_value<B>(&self) -> &B
        where   T: Borrow<B>,
                B: ?Sized + ToOwned<Owned=T>,
    {
        self.value.borrow()
    }

    pub fn borrow_value_mut<B>(&mut self) -> &mut B
        where   T: BorrowMut<B>,
                B: ?Sized + ToOwned<Owned=T>,
    {
        self.value.borrow_mut()
    }




}

// Compare Nodes by their values
impl <T: Ord> PartialEq for Node<T> {
    fn eq(&self, other: &Self) -> bool {
        self.value.eq(&other.value)
    }
}
impl <T: Ord> Eq for Node<T> {}

impl <T: Ord> PartialOrd for Node<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

impl <T: Ord> Ord for Node<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value.cmp(&other.value)
    }
}









//-------------- unit tests ---------------------
#[test]
fn test_hiset_new() {
    let _set = HiSet::<String>::new();
}


