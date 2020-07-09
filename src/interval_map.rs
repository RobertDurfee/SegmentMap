use std::marker::PhantomData;
use std::borrow::Borrow;

use crate::Interval;

pub struct IntervalMap<K, V> {
    phantom_k: PhantomData<K>,
    phantom_v: PhantomData<V>,
}

impl<K, V> IntervalMap<K, V> {
    pub fn new() -> IntervalMap<K, V> {
        panic!("Not implemented")
    }

    pub fn intervals(&self) -> Box<dyn Iterator<Item = &Interval<K>>> {
        panic!("Not implemented")
    }

    pub fn values(&self) -> Box<dyn Iterator<Item = &V>> {
        panic!("Not implemented")
    }

    pub fn values_mut(&mut self) -> Box<dyn Iterator<Item = &mut V>> {
        panic!("Not implemented")
    }

    pub fn iter(&self) -> Box<dyn Iterator<Item = (&Interval<K>, &V)>> {
        panic!("Not implemented")
    }

    pub fn iter_mut(&mut self) -> Box<dyn Iterator<Item = (&Interval<K>, &mut V)>> {
        panic!("Not implemented")
    }

    pub fn span(&self) -> Interval<K> {
        panic!("Not implemented")
    }

    pub fn is_empty(&self) -> bool {
        panic!("Not implemented")
    }

    pub fn clear(&mut self) {
        panic!("Not implemented")
    }

    pub fn get<Q>(&self, _key: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: ?Sized,
    {
        panic!("Not implemented")
    }

    pub fn get_entry<Q>(&self, _key: &Q) -> Option<(&Interval<K>, &V)>
    where
        K: Borrow<Q>,
        Q: ?Sized,
    {
        panic!("Not implemented")
    }

    pub fn contains_key<Q>(&self, _key: &Q) -> bool
    where
        K: Borrow<Q>,
        Q: ?Sized,
    {
        panic!("Not implemented")
    }

    pub fn insert(&mut self, _interval: Interval<K>, _value: V) {
        panic!("Not implemented")
    }

    pub fn insert_if_absent(&mut self, _interval: Interval<K>, _value: V) {
        panic!("Not implemented")
    }

    pub fn remove(&mut self, _interval: &Interval<K>) -> IntervalMap<K, V> {
        panic!("Not implemented")
    }

    pub fn modify<F>(&mut self, _interval: &Interval<K>, _f: F) 
    where
        F: FnMut(&mut V)
    {
        panic!("Not implemented")
    }

    pub fn modify_entry<F>(&mut self, _inteval: &Interval<K>, _f: F)
    where
        F: FnMut(&Interval<K>, &mut V)
    {
        panic!("Not implemented")
    }
}

impl<K, V> Extend<(Interval<K>, V)> for IntervalMap<K, V> {
    fn extend<I: IntoIterator<Item = (Interval<K>, V)>>(&mut self, _iter: I) {
        panic!("Not implemented")
    }
}

impl<K, V> IntoIterator for IntervalMap<K, V> {
    type Item = (Interval<K>, V);
    type IntoIter = IntoIter<K, V>;

    fn into_iter(self) -> IntoIter<K, V> {
        panic!("Not implemented")
    }
}

pub struct IntoIter<K, V> {
    phantom_k: PhantomData<K>,
    phantom_v: PhantomData<V>,
}

impl<K, V> Iterator for IntoIter<K, V> {
    type Item = (Interval<K>, V);

    fn next(&mut self) -> Option<(Interval<K>, V)> {
        panic!("Not implemented")
    }
}

#[macro_export]
macro_rules! interval_map {
    ($($x:expr => $y:expr),*) => {{
        #[allow(unused_mut)]
        let mut temp_interval_map = $crate::IntervalMap::new();
        $(temp_interval_map.insert($x, $y);)*
        temp_interval_map
    }}
}
