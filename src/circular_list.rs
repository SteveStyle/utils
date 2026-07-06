#![allow(dead_code)]

#[derive(Debug, Clone, Copy)]
struct Node<T> {
    value: T,
    next: usize,
    prev: usize,
}

#[derive(Debug, Clone, Copy)]
enum Slot<T> {
    Occupied(Node<T>),
    Free { next: usize },
}

#[derive(Debug)]
pub struct CircularList<T> {
    slots: Vec<Slot<T>>,
    current: usize,
    len: usize,
    free_head: usize,
}

const NO_FREE: usize = usize::MAX;

impl<T> CircularList<T> {
    #[inline]
    pub fn new(initial: T) -> Self {
        Self::with_capacity(initial, 1)
    }

    #[inline]
    pub fn with_capacity(initial: T, capacity: usize) -> Self {
        let mut slots = Vec::with_capacity(capacity.max(1));
        slots.push(Slot::Occupied(Node {
            value: initial,
            next: 0,
            prev: 0,
        }));

        Self {
            slots,
            current: 0,
            len: 1,
            free_head: NO_FREE,
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
        &self.node(self.current).value
    }

    #[inline]
    pub fn current_mut(&mut self) -> &mut T {
        &mut self.node_mut(self.current).value
    }

    #[inline]
    pub fn forward(&mut self, n: usize) {
        for _ in 0..n {
            self.current = self.node(self.current).next;
        }
    }

    #[inline]
    pub fn back(&mut self, n: usize) {
        for _ in 0..n {
            self.current = self.node(self.current).prev;
        }
    }

    #[inline]
    pub fn insert_after(&mut self, value: T) {
        let current = self.current;
        let next = self.node(current).next;
        let inserted = self.allocate_slot(value, current, next);

        self.node_mut(current).next = inserted;
        self.node_mut(next).prev = inserted;
        self.current = inserted;
        self.len += 1;
    }

    #[inline]
    pub fn insert_before(&mut self, value: T) {
        let current = self.current;
        let prev = self.node(current).prev;
        let inserted = self.allocate_slot(value, prev, current);

        self.node_mut(prev).next = inserted;
        self.node_mut(current).prev = inserted;
        self.current = inserted;
        self.len += 1;
    }

    #[inline]
    pub fn remove(&mut self) -> Option<T> {
        if self.len == 1 {
            return None;
        }

        let removed = self.current;
        let next = self.node(removed).next;
        let prev = self.node(removed).prev;

        self.node_mut(prev).next = next;
        self.node_mut(next).prev = prev;

        let value = match std::mem::replace(
            &mut self.slots[removed],
            Slot::Free {
                next: self.free_head,
            },
        ) {
            Slot::Occupied(node) => node.value,
            Slot::Free { .. } => unreachable!("current index pointed to a free slot"),
        };

        self.free_head = removed;
        self.current = next;
        self.len -= 1;
        Some(value)
    }

    #[inline]
    fn allocate_slot(&mut self, value: T, prev: usize, next: usize) -> usize {
        if self.free_head != NO_FREE {
            let index = self.free_head;
            let next_free = match self.slots[index] {
                Slot::Free { next } => next,
                Slot::Occupied(_) => unreachable!("free list pointed to an occupied slot"),
            };
            self.free_head = next_free;
            self.slots[index] = Slot::Occupied(Node { value, next, prev });
            index
        } else {
            let index = self.slots.len();
            self.slots.push(Slot::Occupied(Node { value, next, prev }));
            index
        }
    }

    #[inline]
    fn node(&self, index: usize) -> &Node<T> {
        match &self.slots[index] {
            Slot::Occupied(node) => node,
            Slot::Free { .. } => unreachable!("attempted to read a free slot"),
        }
    }

    #[inline]
    fn node_mut(&mut self, index: usize) -> &mut Node<T> {
        match &mut self.slots[index] {
            Slot::Occupied(node) => node,
            Slot::Free { .. } => unreachable!("attempted to mutate a free slot"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::CircularList;

    #[test]
    fn initializes_with_self_linked_single_node() {
        let list = CircularList::new(42);
        assert_eq!(list.len(), 1);
        assert_eq!(*list.current(), 42);
    }

    #[test]
    fn can_move_insert_and_remove() {
        let mut list = CircularList::new(0);
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
        let mut list = CircularList::new(7);
        assert_eq!(list.remove(), None);
        assert_eq!(*list.current(), 7);
    }
}
