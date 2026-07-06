#![allow(dead_code)]

use std::mem::{MaybeUninit, size_of};
use std::ptr::NonNull;

use bumpalo::Bump;

#[derive(Debug)]
struct ArenaNode<T> {
    value: MaybeUninit<T>,
    next: NonNull<ArenaNode<T>>,
    prev: NonNull<ArenaNode<T>>,
    occupied: bool,
}

#[derive(Debug)]
pub struct ArenaCircularList<T> {
    arena: Bump,
    free_nodes: Vec<NonNull<ArenaNode<T>>>,
    allocated_nodes: Vec<NonNull<ArenaNode<T>>>,
    current: NonNull<ArenaNode<T>>,
    len: usize,
}

impl<T> ArenaCircularList<T> {
    #[inline]
    pub fn new(initial: T) -> Self {
        Self::with_capacity(initial, 1)
    }

    #[inline]
    pub fn with_capacity(initial: T, capacity: usize) -> Self {
        let node_bytes = size_of::<ArenaNode<T>>().max(1);
        let arena = Bump::with_capacity(node_bytes.saturating_mul(capacity.max(1)));

        let first = NonNull::from(arena.alloc(ArenaNode {
            value: MaybeUninit::new(initial),
            next: NonNull::dangling(),
            prev: NonNull::dangling(),
            occupied: true,
        }));

        // SAFETY: `first` points to a valid node allocated in the arena.
        unsafe {
            (*first.as_ptr()).next = first;
            (*first.as_ptr()).prev = first;
        }

        Self {
            arena,
            free_nodes: Vec::new(),
            allocated_nodes: vec![first],
            current: first,
            len: 1,
        }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.len
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    #[inline]
    pub fn current(&self) -> &T {
        // SAFETY: current always points to an occupied node while len > 0.
        unsafe { (*self.current.as_ptr()).value.assume_init_ref() }
    }

    #[inline]
    pub fn current_mut(&mut self) -> &mut T {
        // SAFETY: current always points to an occupied node while len > 0.
        unsafe { (*self.current.as_ptr()).value.assume_init_mut() }
    }

    #[inline]
    pub fn forward(&mut self, n: usize) {
        for _ in 0..n {
            // SAFETY: links always point to valid nodes in the arena.
            self.current = unsafe { (*self.current.as_ptr()).next };
        }
    }

    #[inline]
    pub fn back(&mut self, n: usize) {
        for _ in 0..n {
            // SAFETY: links always point to valid nodes in the arena.
            self.current = unsafe { (*self.current.as_ptr()).prev };
        }
    }

    #[inline]
    pub fn insert_after(&mut self, value: T) {
        let current = self.current;
        // SAFETY: current points to a valid node.
        let next = unsafe { (*current.as_ptr()).next };
        let inserted = self.allocate_node(value, current, next);

        // SAFETY: all pointers refer to valid nodes in the same arena.
        unsafe {
            (*current.as_ptr()).next = inserted;
            (*next.as_ptr()).prev = inserted;
        }
        self.current = inserted;
        self.len += 1;
    }

    #[inline]
    pub fn insert_before(&mut self, value: T) {
        let current = self.current;
        // SAFETY: current points to a valid node.
        let prev = unsafe { (*current.as_ptr()).prev };
        let inserted = self.allocate_node(value, prev, current);

        // SAFETY: all pointers refer to valid nodes in the same arena.
        unsafe {
            (*prev.as_ptr()).next = inserted;
            (*current.as_ptr()).prev = inserted;
        }
        self.current = inserted;
        self.len += 1;
    }

    #[inline]
    pub fn remove(&mut self) -> Option<T> {
        if self.len == 1 {
            return None;
        }

        let removed = self.current;
        // SAFETY: removed points to a valid occupied node.
        let (next, prev) = unsafe { ((*removed.as_ptr()).next, (*removed.as_ptr()).prev) };

        // SAFETY: rewiring links between valid neighboring nodes.
        unsafe {
            (*prev.as_ptr()).next = next;
            (*next.as_ptr()).prev = prev;
        }

        // SAFETY: removed node is occupied; read moves out T without dropping in place.
        let value = unsafe {
            let node = &mut *removed.as_ptr();
            node.occupied = false;
            node.value.as_ptr().read()
        };

        self.free_nodes.push(removed);
        self.current = next;
        self.len -= 1;
        Some(value)
    }

    #[inline]
    fn allocate_node(
        &mut self,
        value: T,
        prev: NonNull<ArenaNode<T>>,
        next: NonNull<ArenaNode<T>>,
    ) -> NonNull<ArenaNode<T>> {
        if let Some(node_ptr) = self.free_nodes.pop() {
            // SAFETY: node_ptr comes from this list's free list and is valid arena memory.
            unsafe {
                let node = &mut *node_ptr.as_ptr();
                node.value = MaybeUninit::new(value);
                node.next = next;
                node.prev = prev;
                node.occupied = true;
            }
            node_ptr
        } else {
            let node_ptr = NonNull::from(self.arena.alloc(ArenaNode {
                value: MaybeUninit::new(value),
                next,
                prev,
                occupied: true,
            }));
            self.allocated_nodes.push(node_ptr);
            node_ptr
        }
    }
}

impl<T> Drop for ArenaCircularList<T> {
    fn drop(&mut self) {
        if std::mem::needs_drop::<T>() {
            for &node_ptr in &self.allocated_nodes {
                // SAFETY: pointers are valid for the lifetime of self.arena.
                unsafe {
                    let node = &mut *node_ptr.as_ptr();
                    if node.occupied {
                        node.value.assume_init_drop();
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ArenaCircularList;

    #[test]
    fn initializes_with_self_linked_single_node() {
        let list = ArenaCircularList::new(42);
        assert_eq!(list.len(), 1);
        assert_eq!(*list.current(), 42);
    }

    #[test]
    fn can_move_insert_and_remove() {
        let mut list = ArenaCircularList::new(0);
        list.insert_after(1);
        assert_eq!(*list.current(), 1);

        list.insert_before(2);
        assert_eq!(*list.current(), 2);

        list.back(1);
        assert_eq!(*list.current(), 0);

        list.forward(2);
        assert_eq!(*list.current(), 1);

        assert_eq!(list.remove(), Some(1));
        assert_eq!(list.len(), 2);
    }

    #[test]
    fn remove_last_node_is_rejected() {
        let mut list = ArenaCircularList::new(7);
        assert_eq!(list.remove(), None);
        assert_eq!(*list.current(), 7);
    }
}
