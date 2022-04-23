pub mod hiset;
pub mod himap;

/// estimate maximum height of balanced binary tree containing this many nodes.
/// Assume all inner nodes are full, only leaf level can be partially filled
#[inline]
pub(crate) fn tree_height(count: usize) -> isize {
    (0_usize.leading_zeros()-count.leading_zeros()) as isize
}
