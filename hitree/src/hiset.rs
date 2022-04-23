//use std::fmt::{Debug,Display,Formatter};

use std::borrow::{Borrow, BorrowMut};
use std::cmp::Ordering;
use super::tree_height;

/// Ordered set of values, accessible by value or index of value in the set.
/// Stores values in a balanced binary tree with subtree node count tracking.
/// Nodes are allocated on the heap using `Box`.
pub struct HiSet<T: Ord> {
    root: Ref<T>,
}

/// Reference to a subtree of Nodes, including node count of subtree pointed to by it.
struct Ref<T>
    where T: Ord
{
    count: usize,
    node: Option<Box<Node<T>>>,
}

/// Node holding a value and references to the left (lesser) and right (greater) subtrees.
/// Left and right subtrees are always balanced - they may differ by at most one level of depth,
/// and all the inner nodes of the tree (all levels except the one furthest from the root)
/// must contain both left and right subtrees that are also balanced.
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
    ///     # #[allow(unused_mut)]
    ///     # use hitree::hiset::HiSet;
    ///     let mut set = HiSet::<String>::new();
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
    ///     # use hitree::hiset::HiSet;
    ///     let hiset = HiSet::<i32>::new();
    ///     assert_eq!(hiset.len(), 0);
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
    ///     # use hitree::hiset::HiSet;
    ///     let mut hiset = HiSet::<i32>::new();
    ///     assert_eq!(hiset.insert(1), true);
    ///     assert_eq!(hiset.insert(2), true);
    ///     assert_eq!(hiset.insert(1), false);
    ///     assert_eq!(hiset.len(), 2);
    /// ```
    /// You can insert &str into `HiSet<String>` for example:
    /// ```
    ///     # use hitree::hiset::HiSet;
    ///     let mut hiset = HiSet::<String>::new(); // This is a set of Strings
    ///     assert_eq!(hiset.insert("This can be converted to a String"), true);
    /// ```
    pub fn insert(&mut self, value: impl Into<T>) -> bool {
        self.root.insert(Node::new(value))
    }


    /// Get a shared borrow of value from set by index.
    /// Values in the set are sorted according to their Ord trait,
    /// index 0 is the smallest value.
    /// Borrowed value can be any shared reference type that can be borrowed from T.
    /// You can use it to borrow `&str` from `HiSet<String>` for example.
    ///
    /// # Examples:
    ///
    /// ```
    ///     # use hitree::hiset::HiSet;
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
              B: ?Sized
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

    /// Get a mutable borrow of value from set by index.
    /// Values in the set are sorted according to their Ord trait,
    /// index 0 is the smallest value.
    /// Borrowed value can be any mutable reference type that can be borrowed from T.
    /// WARNING: You must never change the borrowed value in a way that would affect its ordering according to
    /// its Ord trait implementation!
    ///
    /// # Examples:
    ///
    /// ```
    ///     # use std::cmp::Ordering;
    ///     # use hitree::hiset::HiSet;
    ///
    ///     struct TestValue {
    ///         ordering: String,
    ///         data: usize,
    ///     }
    ///
    ///     impl TestValue {
    ///         pub fn new(ordering: impl Into<String>) -> Self { TestValue { ordering: ordering.into(), data: 0 }}
    ///         pub fn touch(&mut self) { self.data += 1; }
    ///     }
    ///     impl PartialEq for TestValue { fn eq(&self, other: &Self) -> bool { self.ordering.eq(&other.ordering) } }
    ///     impl Eq for TestValue {}
    ///     impl PartialOrd for TestValue { fn partial_cmp(&self, other: &Self) -> Option<Ordering> { self.ordering.partial_cmp(&other.ordering) } }
    ///     impl Ord for TestValue { fn cmp(&self, other: &Self) -> Ordering { self.ordering.cmp(&other.ordering) } }
    ///
    ///
    ///     let mut hiset = HiSet::<TestValue>::new();
    ///     hiset.insert(TestValue::new("first"));
    ///     hiset.insert(TestValue::new("second"));
    ///     hiset.insert(TestValue::new("third"));
    ///
    ///     hiset.get_mut_by_index(0).map(|value| value.touch() );
    ///     hiset.get_mut_by_index(2).map(|value| { value.touch(); value.touch();} );
    ///
    ///     assert_eq!(hiset.get_by_index(0).unwrap().data, 1);
    ///     assert_eq!(hiset.get_by_index(1).unwrap().data, 0);
    ///     assert_eq!(hiset.get_by_index(2).unwrap().data, 2);
    /// ```
    ///
    pub fn get_mut_by_index<B>(&mut self, index: usize) -> Option<&mut B>
        where T: BorrowMut<B>,
              B: ?Sized
    {
        let mut index_to_find = index;
        let mut current_node = self.root.node_mut();
        loop {
            match current_node {
                None => return None,
                Some(node) => {
                    match node.left.count.cmp(&index_to_find) {
                        Ordering::Greater => {
                            // index must be in the left subtree
                            current_node = node.left.node_mut();
                        },
                        Ordering::Equal => {
                            // found it, its this node
                            return Some(node.borrow_value_mut())
                        },
                        Ordering::Less => {
                            // index must be in the right subtree
                            index_to_find = index_to_find - 1 - node.left.count;
                            current_node = node.right.node_mut();
                        }
                    }
                }
            }
        }
    }

    /// Remove the smallest value from the set and return it.
    ///
    /// Examples:
    ///
    /// ```
    ///     # use hitree::hiset::HiSet;
    ///     let mut set = HiSet::<i32>::new();
    ///     set.insert(10);
    ///     set.insert(15);
    ///     set.insert(5);
    ///
    ///     assert_eq!(set.len(), 3);
    ///     assert_eq!(set.remove_first(), Some(5));
    ///     assert_eq!(set.len(), 2);
    ///     assert_eq!(set.remove_first(), Some(10));
    ///     assert_eq!(set.len(), 1);
    ///     assert_eq!(set.remove_first(), Some(15));
    ///     assert_eq!(set.len(), 0);
    ///     assert_eq!(set.remove_first(), None);
    /// ```
    ///
    pub fn remove_first(&mut self) -> Option<T> {
        self.root.remove_first().map(|node| node.value )
    }

    /// Remove the largest value from the set and return it.
    ///
    /// Examples:
    ///
    /// ```
    ///     # use hitree::hiset::HiSet;
    ///     let mut set = HiSet::<i32>::new();
    ///     set.insert(10);
    ///     set.insert(15);
    ///     set.insert(5);
    ///
    ///     assert_eq!(set.len(), 3);
    ///     assert_eq!(set.remove_last(), Some(15));
    ///     assert_eq!(set.len(), 2);
    ///     assert_eq!(set.remove_last(), Some(10));
    ///     assert_eq!(set.len(), 1);
    ///     assert_eq!(set.remove_last(), Some(5));
    ///     assert_eq!(set.len(), 0);
    ///     assert_eq!(set.remove_first(), None);
    /// ```
    ///
    pub fn remove_last(&mut self) -> Option<T> {
        self.root.remove_last().map(|node| node.value )
    }





}

//---------------- Ref -------------------------------------------------------

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

    /*
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
    */

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



    /// insert is recursive as it needs to balance the tree on the way back up
    fn insert(&mut self, new_node: Box<Node<T>>) -> bool {
        match self.node_mut() {
            None => {   // there are no nodes in subtree rooted at this Ref.
                *self = Ref::to(new_node);
                true    // we have inserted a value, return true
            },
            Some(node) => {     // There is at least one node
                match Ord::cmp(&node.value,&new_node.value) {
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

    /// Remove leftmost node from the subtree.
    fn remove_first(&mut self) -> Option<Box<Node<T>>> {
        match self.node_mut() {
            None => None,   // no node here, tell caller to remove his node
            Some(node) => {
                match node.left.remove_first() {
                    None => {
                        // there is no left node, we are the node to remove!
                        let mut removed_node = self.node.take().unwrap();
                        *self = removed_node.right.take();
                        Some(removed_node)
                    },
                    Some(removed_node) => {
                        self.count -= 1;    // one node has been removed
                        if self.balance() > 1 {     // if we are too right leaning now, restore balance
                            self.rotate_left();
                        }
                        Some(removed_node)
                    }
                }
            }
        }
    }

    /// Remove rightmost node from the subtree.
    fn remove_last(&mut self) -> Option<Box<Node<T>>> {
        match self.node_mut() {
            None => None,   // no node here, tell caller to remove his node
            Some(node) => {
                match node.right.remove_first() {
                    None => {
                        // there is no left node, we are the node to remove!
                        let mut removed_node = self.node.take().unwrap();
                        *self = removed_node.left.take();
                        Some(removed_node)
                    },
                    Some(removed_node) => {
                        self.count -= 1;    // one node has been removed
                        if self.balance() < -1 {     // if we are too right leaning now, restore balance
                            self.rotate_right();
                        }
                        Some(removed_node)
                    }
                }
            }
        }
    }

}

impl <T> Default for Ref<T>
    where T: Ord
{
    /// Empty reference
    fn default() -> Self {
        Self { count: 0, node: None }
    }
}


//--------------- Node ------------------------------------------------------------




impl <T> Node<T>
    where T: Ord
{
    /// Creates a new Node with given value and empty left & right refs
    fn new(value: impl Into<T>) -> Box<Node<T>> {
        Box::new( Node { value: value.into(), left: Ref::default(), right: Ref::default() } )
    }

    /// Calculate number of nodes including this node and any subtrees pointed to by left & right
    fn count(&self) -> usize {
        self.left.count + self.right.count + 1
    }

    /// returns difference in height between right and left subtrees. >0 right is bigger, <0 left is bigger.
    #[inline]
    fn balance(&self) -> isize {
        tree_height(self.right.count) - tree_height(self.left.count)
    }

    /// Borrow value of this node immutably
    pub fn borrow_value<B>(&self) -> &B
        where   T: Borrow<B>,
                B: ?Sized
    {
        self.value.borrow()
    }

    /// Borrow value of this node mutably
    pub fn borrow_value_mut<B>(&mut self) -> &mut B
        where   T: BorrowMut<B>,
                B: ?Sized
    {
        self.value.borrow_mut()
    }




}
/*
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
*/








//-------------- unit tests ---------------------
#[test]
fn test_hiset_new() {
    let _set = HiSet::<String>::new();
}


