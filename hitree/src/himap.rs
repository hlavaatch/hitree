#![allow(dead_code)]




pub struct HiMap<K,V>
    where K: Ord
{
    root: Ref<K,V>,
}

struct Ref<K,V>
    where K: Ord
{
    count: usize,
    node: Option<Box<Node<K,V>>>,
}

struct Node<K,V>
    where K: Ord
{
    key: K,
    value: V,
    left: Ref<K,V>,
    right: Ref<K,V>,
}
