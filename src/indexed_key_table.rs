use std::ops::IndexMut;
use std::{marker::PhantomData, ops::Index};

pub trait Key: Copy + Ord {}
pub trait ForeignKey: Copy + Ord {}
impl ForeignKey for usize {}
impl<T: Key> ForeignKey for T {}

pub trait Entry<K, F>: Clone
where
    K: Key,
    F: ForeignKey,
{
    fn key(&self) -> K;
}
pub trait MappableEntry<K: Key>: Entry<K, K> {
    type Output: Entry<K, usize>;
    fn map_key<F>(&self, key2index: F) -> Self::Output
    where
        F: Fn(&K) -> usize;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IndexedKeyTable<K, F, E>(
    pub Vec<E>,
    std::marker::PhantomData<K>,
    std::marker::PhantomData<F>,
)
where
    K: Key,
    F: ForeignKey,
    E: Entry<K, F>;

impl<K, F, E> IndexedKeyTable<K, F, E>
where
    K: Key,
    F: ForeignKey,
    E: Entry<K, F>,
{
    fn key2index(&self, key: &K) -> usize {
        let mut floor = 0;
        let mut above = self.0.len();
        while floor < above {
            let sample = (floor + above) / 2;
            match self.0[sample].key().cmp(key) {
                std::cmp::Ordering::Less => floor = sample + 1,
                std::cmp::Ordering::Equal => return sample,
                std::cmp::Ordering::Greater => above = sample,
            }
        }
        unreachable!()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl<K, F, E> Index<usize> for IndexedKeyTable<K, F, E>
where
    K: Key,
    F: ForeignKey,
    E: Entry<K, F>,
{
    type Output = E;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}
impl<K, F, E> IndexMut<usize> for IndexedKeyTable<K, F, E>
where
    K: Key,
    F: ForeignKey,
    E: Entry<K, F>,
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl<K, F, E> Index<K> for IndexedKeyTable<K, F, E>
where
    K: Key,
    F: ForeignKey,
    E: Entry<K, F>,
{
    type Output = E;

    fn index(&self, key: K) -> &Self::Output {
        let index = self.key2index(&key);
        &self.0[index]
    }
}

impl<K, E> IndexedKeyTable<K, K, E>
where
    K: Key,
    E: MappableEntry<K> + std::fmt::Debug,
{
    pub fn new(mut input: Vec<E>) -> IndexedKeyTable<K, usize, E::Output> {
        input.sort_by_key(|e| e.key());
        let input = IndexedKeyTable(input, PhantomData, PhantomData);

        let mut v = Vec::with_capacity(input.0.len());
        for e in &input.0 {
            v.push(e.map_key(|k| input.key2index(k)));
        }

        IndexedKeyTable(v, PhantomData, PhantomData)
    }
}

//
//
//
//  example
//

#[cfg(test)]
mod tests {
    use super::*;
    impl Key for char {}

    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
    struct MyEntry<R: ForeignKey> {
        key: char,
        reference: R,
        value: u8,
    }
    impl<R> Entry<char, R> for MyEntry<R>
    where
        R: ForeignKey,
    {
        fn key(&self) -> char {
            self.key
        }
    }
    impl MappableEntry<char> for MyEntry<char> {
        type Output = MyEntry<usize>;

        fn map_key<F>(&self, key2index: F) -> Self::Output
        where
            F: Fn(&char) -> usize,
        {
            MyEntry {
                key: self.key,
                reference: key2index(&self.reference),
                value: self.value,
            }
        }
    }

    #[test]
    fn test_indexed_key_table() {
        let entries = vec![
            MyEntry {
                key: 'a',
                reference: 'b',
                value: 1,
            },
            MyEntry {
                key: 'b',
                reference: 'c',
                value: 2,
            },
            MyEntry {
                key: 'c',
                reference: 'a',
                value: 3,
            },
        ];

        let table = IndexedKeyTable::new(entries);
        assert_eq!(table['a'].key(), 'a');
        assert_eq!(table['b'].key(), 'b');
        assert_eq!(table['c'].key(), 'c');
        assert_eq!(table['a'].reference, 1);
        assert_eq!(table['b'].reference, 2);
        assert_eq!(table['c'].reference, 0);
        assert_eq!(table['a'].value, 1);
        assert_eq!(table['b'].value, 2);
        assert_eq!(table['c'].value, 3);
    }
}
