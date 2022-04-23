//! # Indexable containers
//!
//! This crate contains set and map implementations backed by a balanced binary tree with subtree
//! node count tracking, allowing easy random access by index of the value within the order implied
//! by the set value or map key. Tree nodes are stored on heap using [Box](`std::boxed::Box`).
//!
//! ## Features
//!
//! | Feature | [HiSet](hiset::HiSet<T>) | [HiMap](himap::HiMap<T>) |
//! | ------- | ------- | ------- |
//! | Zero allocation initialization | [new](`hiset::HiSet<T>::new`) | todo |
//! | Insert with automatic conversion | [insert](`hiset::HiSet<T>::insert`) | todo |
//! | Borrow value by index | [get_by_index](`hiset::HiSet<T>::get_by_index<B>`) <br>[get_mut_by_index](`hiset::HiSet<T>::get_mut_by_index<B>`) | todo |
//! | Borrow value by key | todo | todo |
//! | Remove first/last entry | [remove_first](`hiset::HiSet<T>::remove_first`) <br>[remove_last](`hiset::HiSet<T>::remove_last`) | todo |
//! | Remove entry by index | todo | todo |
//! | Remove entry by value/key | todo | todo |


/// # Indexable set (incomplete)
pub mod hiset;


/// # Indexable map (todo)
pub mod himap;

/// estimate maximum height of balanced binary tree containing this many nodes.
/// Assume all inner nodes are full, only leaf level can be partially filled
#[inline]
pub(crate) fn tree_height(count: usize) -> isize {
    (0_usize.leading_zeros()-count.leading_zeros()) as isize
}
