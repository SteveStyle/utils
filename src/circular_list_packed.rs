#![allow(dead_code)]

use std::mem::MaybeUninit;

const NO_FREE: usize = usize::MAX;

#[derive(Debug)]
pub struct PackedCircularList<T> {
    values: Vec<MaybeUninit<T>>,
    next: Vec<usize>,
    prev: Vec<usize>,
    occupied: Vec<bool>,
    current: usize,
    len: usize,
    free_head: usize,
}

impl<T> PackedCircularList<T> {
    #[inline]
    pub fn new(initial: T) -> Self {
        Self::with_capacity(initial, 1)
    }

    #[inline]
    pub fn with_capacity(initial: T, capacity: usize) -> Self {
        let mut values = Vec::with_capacity(capacity.max(1));
        values.push(MaybeUninit::new(initial));

        Self {
            values,
            next: vec![0],
            prev: vec![0],
            occupied: vec![true],
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
        // SAFETY: current always points to an occupied element.
        unsafe { self.values[self.current].assume_init_ref() }
    }

    #[inline]
    pub fn current_mut(&mut self) -> &mut T {
        // SAFETY: current always points to an occupied element.
        unsafe { self.values[self.current].assume_init_mut() }
    }

    #[inline]
    pub fn forward(&mut self, n: usize) {
        for _ in 0..n {
            self.current = self.next[self.current];
        }
    }

    #[inline]
    pub fn back(&mut self, n: usize) {
        for _ in 0..n {
            self.current = self.prev[self.current];
        }
    }

    #[inline]
    pub fn insert_after(&mut self, value: T) {
        let current = self.current;
        let next = self.next[current];
        let inserted = self.allocate_slot(value);

        self.prev[inserted] = current;
        self.next[inserted] = next;
        self.next[current] = inserted;
        self.prev[next] = inserted;

        self.current = inserted;
        self.len += 1;
    }

    #[inline]
    pub fn insert_before(&mut self, value: T) {
        let current = self.current;
        let prev = self.prev[current];
        let inserted = self.allocate_slot(value);

        self.prev[inserted] = prev;
        self.next[inserted] = current;
        self.next[prev] = inserted;
        self.prev[current] = inserted;

        self.current = inserted;
        self.len += 1;
    }

    #[inline]
    pub fn remove(&mut self) -> Option<T> {
        if self.len == 1 {
            return None;
        }

        let removed = self.current;
        let next = self.next[removed];
        let prev = self.prev[removed];

        self.next[prev] = next;
        self.prev[next] = prev;

        self.occupied[removed] = false;
        self.next[removed] = self.free_head;
        self.free_head = removed;

        self.current = next;
        self.len -= 1;

        // SAFETY: removed was occupied and has just been unlinked.
        Some(unsafe { self.values[removed].as_ptr().read() })
    }

    #[inline]
    fn allocate_slot(&mut self, value: T) -> usize {
        if self.free_head != NO_FREE {
            let index = self.free_head;
            self.free_head = self.next[index];
            self.values[index] = MaybeUninit::new(value);
            self.occupied[index] = true;
            index
        } else {
            let index = self.values.len();
            self.values.push(MaybeUninit::new(value));
            self.next.push(index);
            self.prev.push(index);
            self.occupied.push(true);
            index
        }
    }
}

impl<T> Drop for PackedCircularList<T> {
    fn drop(&mut self) {
        if std::mem::needs_drop::<T>() {
            for (index, occupied) in self.occupied.iter().copied().enumerate() {
                if occupied {
                    // SAFETY: occupied slots contain initialized T values.
                    unsafe {
                        self.values[index].assume_init_drop();
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::PackedCircularList;

    #[test]
    fn initializes_with_self_linked_single_node() {
        let list = PackedCircularList::new(42);
        assert_eq!(list.len(), 1);
        assert_eq!(*list.current(), 42);
    }

    #[test]
    fn can_move_insert_and_remove() {
        let mut list = PackedCircularList::new(0);
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
        let mut list = PackedCircularList::new(7);
        assert_eq!(list.remove(), None);
        assert_eq!(*list.current(), 7);
    }
}
