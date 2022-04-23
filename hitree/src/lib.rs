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
//! | Borrow value by index | [get_by_index](`hiset::HiSet<T>::get_by_index<B>`) <br>[get_by_index_mut](`hiset::HiSet<T>::get_by_index_mut<B>`) | todo |
//! | Borrow value by key | [get](`hiset::HiSet<T>::get<KEY>`) <br>[get_mut](`hiset::HiSet<T>::get_mut<KEY>`) | todo |
//! | Find index of value by key | [index_of](`hiset::HiSet<T>::index_of<KEY>`) | todo |
//! | Remove first/last entry | [take_first](`hiset::HiSet<T>::take_first`) <br>[take_last](`hiset::HiSet<T>::take_last`) | todo |
//! | Remove entry by index | [take_by_index](`hiset::HiSet<T>::take_by_index`) | todo |
//! | Remove entry by key reference | [take](`hiset::HiSet<T>::take`) | todo |


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
