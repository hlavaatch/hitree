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
    ///     hiset.get_by_index_mut(0).map(|value| value.touch() );
    ///     hiset.get_by_index_mut(2).map(|value| { value.touch(); value.touch();} );
    ///
    ///     assert_eq!(hiset.get_by_index(0).unwrap().data, 1);
    ///     assert_eq!(hiset.get_by_index(1).unwrap().data, 0);
    ///     assert_eq!(hiset.get_by_index(2).unwrap().data, 2);
    /// ```
    ///
    pub fn get_by_index_mut<B>(&mut self, index: usize) -> Option<&mut B>
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


    /// Borrow value from set by key reference.
    /// Reference type of key must have the same `Ord` ordering as `&T`.
    ///
    /// # Examples:
    /// ```
    ///     # use hitree::hiset::HiSet;
    ///     let mut set = HiSet::<String>::new();
    ///     set.insert("This");
    ///     set.insert("is");
    ///     set.insert("a");
    ///     set.insert("test!");
    ///
    ///     assert_eq!(set.get("test!"), Some(&"test!".to_string()));
    ///     assert_eq!(set.get("not there"), None);
    ///     assert_eq!(set.get(&"This".to_string()), Some(&"This".to_string()));
    /// ```
    pub fn get<KEY>(&mut self, key: &KEY) -> Option<&T>
        where KEY: ?Sized + Ord, T: Borrow<KEY>
    {
        let mut current_node = self.root.node();
        loop {
            match current_node {
                None => return None,
                Some(node) => {
                    match Ord::cmp(node.value.borrow(), key) {
                        Ordering::Greater => {
                            // index must be in the left subtree
                            current_node = node.left.node();
                        },
                        Ordering::Equal => {
                            // found it, its this node
                            return Some(node.borrow_value::<T>())
                        },
                        Ordering::Less => {
                            // index must be in the right subtree
                            current_node = node.right.node();
                        }
                    }
                }
            }
        }
    }

    /// Borrow mutably value from set by key reference.
    /// Reference type of key must have the same `Ord` ordering as `&T`.
    ///
    /// # Examples:
    /// ```
    ///     # use hitree::hiset::HiSet;
    ///     let mut set = HiSet::<String>::new();
    ///     set.insert("This");
    ///     set.insert("is");
    ///     set.insert("a");
    ///     set.insert("test!");
    ///
    ///     assert_eq!(set.get_mut("test!"), Some(&mut "test!".to_string()));
    ///     assert_eq!(set.get_mut("not there"), None);
    ///     assert_eq!(set.get_mut(&"This".to_string()), Some(&mut "This".to_string()));
    ///```
    pub fn get_mut<KEY>(&mut self, key: &KEY) -> Option<&mut T>
        where KEY: ?Sized + Ord, T: Borrow<KEY>
    {
        let mut current_node = self.root.node_mut();
        loop {
            match current_node {
                None => return None,
                Some(node) => {
                    match Ord::cmp(node.value.borrow(), key) {
                        Ordering::Greater => {
                            // index must be in the left subtree
                            current_node = node.left.node_mut();
                        },
                        Ordering::Equal => {
                            // found it, its this node
                            return Some(node.borrow_value_mut::<T>())
                        },
                        Ordering::Less => {
                            // index must be in the right subtree
                            current_node = node.right.node_mut();
                        }
                    }
                }
            }
        }
    }



    /// Find index of value given by key reference.
    ///
    /// # Examples:
    /// ```
    ///     # use hitree::hiset::HiSet;
    ///     let mut set = HiSet::<String>::new();
    ///     set.insert("This");
    ///     set.insert("is");
    ///     set.insert("a");
    ///     set.insert("test!");
    ///
    ///     assert_eq!(set.index_of("This"), Some(0));
    ///     assert_eq!(set.index_of("a"), Some(1));
    ///     assert_eq!(set.index_of("is"), Some(2));
    ///     assert_eq!(set.index_of("test!"), Some(3));
    ///     assert_eq!(set.index_of("nonexistent"), None);
    ///
    /// ```
    pub fn index_of<KEY>(&mut self, key: &KEY) -> Option<usize>
        where KEY: ?Sized + Ord, T: Borrow<KEY>
    {
        let mut current_node = self.root.node();
        let mut current_index_shift = 0;
        loop {
            match current_node {
                None => return None,
                Some(node) => {
                    match Ord::cmp(node.value.borrow(), key) {
                        Ordering::Greater => {
                            // index must be in the left subtree
                            current_node = node.left.node();
                        },
                        Ordering::Equal => {
                            // found it, its this node
                            return Some(current_index_shift + node.left.count)
                        },
                        Ordering::Less => {
                            // index must be in the right subtree
                            current_node = node.right.node();
                            current_index_shift += 1 + node.left.count;
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
    ///     assert_eq!(set.take_first(), Some(5));
    ///     assert_eq!(set.len(), 2);
    ///     assert_eq!(set.take_first(), Some(10));
    ///     assert_eq!(set.len(), 1);
    ///     assert_eq!(set.take_first(), Some(15));
    ///     assert_eq!(set.len(), 0);
    ///     assert_eq!(set.take_first(), None);
    /// ```
    ///
    pub fn take_first(&mut self) -> Option<T> {
        self.root.take_leftmost_node().map(|node| node.value )
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
    ///     assert_eq!(set.take_last(), Some(15));
    ///     assert_eq!(set.len(), 2);
    ///     assert_eq!(set.take_last(), Some(10));
    ///     assert_eq!(set.len(), 1);
    ///     assert_eq!(set.take_last(), Some(5));
    ///     assert_eq!(set.len(), 0);
    ///     assert_eq!(set.take_last(), None);
    /// ```
    ///
    pub fn take_last(&mut self) -> Option<T> {
        self.root.take_rightmost_node().map(|node| node.value )
    }


    /// Take an entry by reference to another value and return it.
    /// Whatever you use as key must give the same Ord results as Ord on &T!
    ///
    ///  # Examples:
    ///
    /// ```
    ///     # use hitree::hiset::HiSet;
    ///     let mut set = HiSet::<String>::new();
    ///     assert_eq!(set.insert("first"), true);
    ///     assert_eq!(set.insert("second"), true);
    ///     assert_eq!(set.insert("third"), true);
    ///     assert_eq!(set.len(), 3);
    ///
    ///     assert_eq!(set.take("second").unwrap().as_str(), "second");
    ///     assert_eq!(set.len(), 2);
    ///     assert_eq!(set.take("second"), None);
    ///
    ///     assert_eq!(set.take(&"third".to_string()).unwrap().as_str(), "third");
    ///     assert_eq!(set.len(), 1);
    ///
    /// ```
    pub fn take<KEY>(&mut self, key: &KEY) -> Option<T>
        where KEY: ?Sized + Ord, T: Borrow<KEY>
    {
        let key = key.borrow();
        self.root.take_node_by_key(key).map(|node| node.value )
    }

    /// Take an entry by reference to another value and return it.
    /// Whatever you use as key must give the same Ord results as Ord on &T!
    ///
    ///  # Examples:
    ///
    /// ```
    ///     # use hitree::hiset::HiSet;
    ///     let mut set = HiSet::<String>::new();
    ///     assert_eq!(set.insert("first"), true);
    ///     assert_eq!(set.insert("second"), true);
    ///     assert_eq!(set.insert("third"), true);
    ///     assert_eq!(set.len(), 3);
    ///
    ///     assert_eq!(set.take_by_index(2).unwrap().as_str(), "third");
    ///     assert_eq!(set.len(), 2);
    ///
    ///     assert_eq!(set.take_by_index(3), None);
    ///
    ///     assert_eq!(set.take_by_index(1).unwrap().as_str(), "second");
    ///     assert_eq!(set.len(), 1);
    ///
    ///     assert_eq!(set.take_by_index(0).unwrap().as_str(), "first");
    ///     assert_eq!(set.len(), 0);
    /// ```
    pub fn take_by_index(&mut self, index: usize) -> Option<T> {
        self.root.take_node_by_index(index).map(|node| node.value )
    }




    /// Return iterator over all &T.
    ///
    ///
    pub fn iter(&self) -> HiSetIterator<'_,T> {
        HiSetIterator { set: self, start: 0, end: self.root.count }
    }


    /// Return double ended iterator over &T in given index range.
    ///
    /// # Examples:
    /// ```
    ///   #  use hitree::hiset::HiSet;
    ///     let s = HiSet::<i32>::from([0,1,2,3,4,5,6].into_iter());
    ///     let mut r = s.range(2..=5).map(|v| *v);
    ///     assert!(r.eq( [2,3,4,5].into_iter() ));
    /// ```
    pub fn range(&self, range: impl std::ops::RangeBounds<usize>) -> HiSetIterator<'_,T> {
        use std::ops::Bound::*;
        let start = match range.start_bound() {
            Included(index) => *index,
            Excluded(index) => *index + 1,
            Unbounded => 0
        };
        let end = match range.end_bound() {
            Included(index) => *index + 1,
            Excluded(index) => *index,
            Unbounded => self.root.count
        };

        HiSetIterator { set: self, start, end }
    }
}

#[test]
fn test_hiset_range() {
        let s = HiSet::<i32>::from([0,1,2,3,4,5,6].into_iter());
        let r = s.range(2..=5).map(|v| *v);
        assert!(r.eq( [2,3,4,5].into_iter() ));
}

pub struct HiSetOwnedIterator<T>
    where T: Ord
{
    root: Ref<T>,
}

impl <T> Iterator for HiSetOwnedIterator<T>
    where T: Ord
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        // take leftmost node without bothering to re-balance or maintain node counts
        self.root.consume_next()
    }
}

impl <T> IntoIterator for HiSet<T>
    where T: Ord
{
    type Item = T;
    type IntoIter = HiSetOwnedIterator<T>;

    /// Turn HiSet<T> into Iterator of owned T
    /// ```
    ///  # use hitree::hiset::HiSet;
    /// let mut s = HiSet::<String>::new();
    /// s.insert("This");
    /// s.insert("is");
    /// s.insert("a");
    /// s.insert("test!");
    ///
    /// let mut i = s.into_iter();
    /// assert_eq!(i.next(), Some("This".to_string()));
    /// assert_eq!(i.next(), Some("a".to_string()));
    /// assert_eq!(i.next(), Some("is".to_string()));
    /// assert_eq!(i.next(), Some("test!".to_string()));
    /// assert_eq!(i.next(), None);
    /// ```
    fn into_iter(self) -> Self::IntoIter {
        HiSetOwnedIterator { root: self.root }
    }
}



/// Get iterator over &T
///
/// # Examples:
///
/// ```
///  # use hitree::hiset::HiSet;
/// let mut s = HiSet::<String>::new();
/// s.insert("This");
/// s.insert("is");
/// s.insert("a");
/// s.insert("test!");
///
/// let mut i = s.iter();
///
/// assert_eq!(i.next(), Some(&"This".to_string()));
/// assert_eq!(i.next(), Some(&"a".to_string()));
/// assert_eq!(i.next(), Some(&"is".to_string()));
/// assert_eq!(i.next(), Some(&"test!".to_string()));
/// assert_eq!(i.next(), None);
///
/// ```
pub struct HiSetIterator<'set,T>
    where T: Ord
{
    set:    &'set HiSet<T>,
    start:  usize,
    end:    usize,
}

impl <'set,T> Iterator for HiSetIterator<'set,T>
    where T: Ord
{
    type Item = &'set T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.start >= self.end {
            None
        } else {
            let index_to_return = self.start;
            self.start += 1;
            self.set.get_by_index(index_to_return)
        }
    }
}

impl <'set,T> DoubleEndedIterator for HiSetIterator<'set,T>
    where T: Ord
{
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.start >= self.end {
            None
        } else {
            self.end -= 1;
            self.set.get_by_index(self.end)
        }
    }
}




impl <'set,T> IntoIterator for &'set HiSet<T>
    where T: Ord
{
    type Item = &'set T;
    type IntoIter = HiSetIterator<'set,T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}


/// Get iterator over &mut T
///
/// # Examples:
///
/// ```
///  # use hitree::hiset::HiSet;
/// let mut s = HiSet::<String>::new();
/// s.insert("This");
/// s.insert("is");
/// s.insert("a");
/// s.insert("test!");
///
/// let mut i = s.iter_mut();
///
/// assert_eq!(i.next(), Some(&mut "This".to_string()));
/// assert_eq!(i.next(), Some(&mut "a".to_string()));
/// assert_eq!(i.next(), Some(&mut "is".to_string()));
/// assert_eq!(i.next(), Some(&mut "test!".to_string()));
/// assert_eq!(i.next(), None);
///
/// ```
pub struct HiSetIteratorMut<'set,T>
    where T: Ord
{
    set:    &'set mut HiSet<T>,
    start:  usize,
    end:    usize,
}

impl <'set,T> Iterator for HiSetIteratorMut<'set,T>
    where T: Ord,
{
    type Item = &'set mut T;

    fn next<'iter>(&mut self) -> Option<Self::Item> {
        if self.start >= self.end {
            None
        } else {
            let index_to_return = self.start;
            self.start += 1;
            unsafe { std::mem::transmute(self.set.get_by_index_mut(index_to_return)) }
        }
    }
}

impl <'set,T> IntoIterator for &'set mut HiSet<T>
    where T: Ord
{
    type Item = &'set mut T;
    type IntoIter = HiSetIteratorMut<'set,T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl <T> HiSet<T>
    where T: Ord
{
    pub fn iter_mut(&mut self) -> HiSetIteratorMut<'_,T> {
        let end = self.root.count;
        HiSetIteratorMut { set: self, start: 0, end }
    }

}


impl <T,I,X,O> From<I> for HiSet<T>
    where T: Ord,
          I: Iterator<Item=X>,
          O: Into<T>,
          X: ToOwned<Owned=O>
{
    /// Construct HiSet from Iterator of values that can be made into owned instances of T
    ///
    /// # Examples:
    ///
    /// ```
    /// # use hitree::hiset::HiSet;
    /// let s = HiSet::<String>::from( ["This","is","a","test!"].into_iter() );
    ///
    /// assert!(s.iter().eq(["This","a","is","test!"].iter()));
    /// ```
    fn from(mut iterator: I) -> Self {
        let mut s = HiSet::<T>::new();
        while let Some(value) = iterator.next() {
            s.insert(value.to_owned());
        }
        s
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

    fn take_left_subtree(&mut self) -> Ref<T> {
        match self.node_mut() {
            None => Ref::default(),
            Some(node) => {
                let left = node.left.take();
                self.count -= left.count;
                left
            },
        }
    }

    fn take_right_subtree(&mut self) -> Ref<T> {
        match self.node_mut() {
            None => Ref::default(),
            Some(node) => {
                let right = node.right.take();
                self.count -= right.count;
                right
            },
        }
    }

    fn is_empty(&self) -> bool {
        self.node.is_none()
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
        let mut new_root = old_root.take_right_subtree();
        let mid_subtree = new_root.take_left_subtree();
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
        let mut new_root = old_root.take_left_subtree();
        let mid_subtree = new_root.take_right_subtree();
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
    fn take_leftmost_node(&mut self) -> Option<Box<Node<T>>> {
        match self.node_mut() {
            None => None,   // no node here, tell caller to remove his node
            Some(node) => {
                match node.left.take_leftmost_node() {
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
    fn take_rightmost_node(&mut self) -> Option<Box<Node<T>>> {
        match self.node_mut() {
            None => None,   // no node here, tell caller to remove his node
            Some(node) => {
                match node.right.take_leftmost_node() {
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

    fn take_node_by_key<KEY>(&mut self, key: &KEY) -> Option<Box<Node<T>>>
        where KEY: ?Sized + Ord,
            T: Borrow<KEY>
    {
        let res = if let Some(node) = self.node_mut() {
            match Ord::cmp(node.value.borrow(), key) {
                Ordering::Equal => {    // this is the node to remove
                    match (node.left.is_empty(), node.right.is_empty()) {
                        (true, true) => {    // leaf node, can be removed directly without consequences
                            self.node.take()
                        },
                        (false, true) => {   // there is a left subtree, move it up
                            let mut removed_node = self.node.take().unwrap();
                            *self = removed_node.left.take();
                            Some(removed_node)
                        },
                        (true, false) => {   // there is a right subtree, move it up
                            let mut removed_node = self.node.take().unwrap();
                            *self = removed_node.right.take();
                            Some(removed_node)
                        }
                        (false, false) => {  // there are two subtrees, take the closest node from the one with more nodes and replace the removed node with it
                            let mut removed_node = self.node.take().unwrap();
                            let mut left_subtree = removed_node.left.take();
                            let mut right_subtree = removed_node.right.take();
                            let mut new_subtree_root_node = if left_subtree.count > right_subtree.count {
                                left_subtree.take_rightmost_node().unwrap()
                            } else {
                                right_subtree.take_leftmost_node().unwrap()
                            };
                            new_subtree_root_node.left = left_subtree;
                            new_subtree_root_node.right = right_subtree;
                            let new_count = new_subtree_root_node.count();
                            self.node = Some(new_subtree_root_node);
                            self.count = new_count;
                            // balance should not be an issue, we took from the bigger one
                            Some(removed_node)
                        }
                    }
                },
                Ordering::Less => {     // node must be in the right subtree
                    let removed_node_maybe = node.right.take_node_by_key(key);
                    match removed_node_maybe {
                        None => None,   // not found
                        Some(removed_node) => {
                            Some(removed_node)
                        }
                    }
                },
                Ordering::Greater => {  // node must be in the left subtree
                    match node.left.take_node_by_key(key) {
                        None => None,
                        Some(removed_node) => {
                            Some(removed_node)
                        }
                    }
                }
            }
        } else {
            None
        };
        if res.is_some() {
            self.rebalance();
        }
        res
    }

    fn take_node_by_index(&mut self, index_to_take: usize) -> Option<Box<Node<T>>> {
        let res = if let Some(node) = self.node_mut() {
            let index_of_this_node = node.left.count;
            match Ord::cmp(&index_of_this_node, &index_to_take) {
                Ordering::Equal => {    // this is the node to remove
                    match (node.left.is_empty(), node.right.is_empty()) {
                        (true, true) => {    // leaf node, can be removed directly without consequences
                            self.node.take()
                        },
                        (false, true) => {   // there is a left subtree, move it up
                            let mut removed_node = self.node.take().unwrap();
                            *self = removed_node.left.take();
                            Some(removed_node)
                        },
                        (true, false) => {   // there is a right subtree, move it up
                            let mut removed_node = self.node.take().unwrap();
                            *self = removed_node.right.take();
                            Some(removed_node)
                        }
                        (false, false) => {  // there are two subtrees, take the closest node from the one with more nodes and replace the removed node with it
                            let mut removed_node = self.node.take().unwrap();
                            let mut left_subtree = removed_node.left.take();
                            let mut right_subtree = removed_node.right.take();
                            let mut new_subtree_root_node = if left_subtree.count > right_subtree.count {
                                left_subtree.take_rightmost_node().unwrap()
                            } else {
                                right_subtree.take_leftmost_node().unwrap()
                            };
                            new_subtree_root_node.left = left_subtree;
                            new_subtree_root_node.right = right_subtree;
                            let new_count = new_subtree_root_node.count();
                            self.node = Some(new_subtree_root_node);
                            self.count = new_count;
                            // balance should not be an issue, we took from the bigger one
                            Some(removed_node)
                        }
                    }
                },
                Ordering::Less => {     // node must be in the right subtree
                    let removed_node_maybe = node.right.take_node_by_index(index_to_take - index_of_this_node - 1);
                    match removed_node_maybe {
                        None => None,   // not found
                        Some(removed_node) => {
                            Some(removed_node)
                        }
                    }
                },
                Ordering::Greater => {  // node must be in the left subtree
                    match node.left.take_node_by_index(index_to_take) {
                        None => None,
                        Some(removed_node) => {
                            Some(removed_node)
                        }
                    }
                }
            }
        } else {
            None
        };
        if res.is_some() {
            self.rebalance();
        }
        res
    }

    fn rebalance(&mut self) {
        if let Some(node) = self.node() {
            self.count = node.count();
            let balance = self.balance();
            if balance < -1 {
                self.rotate_right();
            } else if balance > 1 {
                self.rotate_left();
            }
        } else {
            self.count = 0;
        };
    }


    /// Take fist value without bothering to re-balance or maintain node counts. For use within owned iterator.
    fn consume_next(&mut self) -> Option<T> {
        // Take node from left subtree if any, or
        // Take node from yourself, replacing it with the right subtree root node if any
        match &mut self.node {
            None => None,   // no nodes left, end of iteration
            Some(node) => {
                if let Some(from_left) = node.left.consume_next() {
                    Some(from_left)
                } else {
                    let right_node = node.right.node.take();
                    let my_node = unsafe { std::mem::replace(&mut self.node, right_node ).unwrap_unchecked() };
                    Some(my_node.value)
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








