use crate::Interval;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct IntervalMapNode<K, V> {
    interval: Interval<K>,
    value: V,
    left: Box<Option<IntervalMapNode<K, V>>>,
    right: Box<Option<IntervalMapNode<K, V>>>
}

impl<K: Clone + PartialOrd, V: Clone> IntervalMapNode<K, V> {
    pub fn new(interval: Interval<K>, value: V, left: Option<IntervalMapNode<K, V>>, right: Option<IntervalMapNode<K, V>>) -> IntervalMapNode<K, V> {
        IntervalMapNode {
            interval,
            value,
            left: Box::new(left),
            right: Box::new(right),
        }
    }

    pub fn min_key(&self) -> &K {
        self.min_node().interval.lower()
    }

    pub fn min_node(&self) -> &IntervalMapNode<K, V> {
        if let Some(left) = self.left.as_ref() {
            left.min_node()
        } else { self }
    }

    pub fn min_node_mut(&mut self) -> &mut IntervalMapNode<K, V> {
        if let Some(ref mut left) = *self.left {
            left.min_node_mut()
        } else { self }
    }

    pub fn remove_min_node(mut self) -> (Option<IntervalMapNode<K, V>>, IntervalMapNode<K, V>) {
        if let Some(left) = self.left.take() {
            let (left, min_node) = left.remove_max_node();
            self.left = Box::new(left);
            (Some(self), min_node)
        } else { (None, self) }
    }

    pub fn max_key(&self) -> &K {
        self.max_node().interval.upper()
    }

    pub fn max_node(&self) -> &IntervalMapNode<K, V> {
        if let Some(right) = self.right.as_ref() {
            right.max_node()
        } else { self }
    }

    pub fn max_node_mut(&mut self) -> &mut IntervalMapNode<K, V> {
        if let Some(ref mut right) = *self.right {
            right.max_node_mut()
        } else { self }
    }

    pub fn remove_max_node(mut self) -> (Option<IntervalMapNode<K, V>>, IntervalMapNode<K, V>) {
        if let Some(right) = self.right.take() {
            let (right, max_node) = right.remove_max_node();
            self.right = Box::new(right);
            (Some(self), max_node)
        } else { (None, self) }
    }

    pub fn span(&self) -> Interval<&K> {
        Interval::new(self.min_key(), self.max_key())
    }

    pub fn get_entry(&self, key: &K) -> Option<(&Interval<K>, &V)> {
        if self.interval.contains(key) {
            Some((&self.interval, &self.value))
        } else if key < self.interval.lower() {
            if let Some(left) = self.left.as_ref() {
                left.get_entry(key)
            } else { None }
        } else {
            if let Some(right) = self.right.as_ref() {
                right.get_entry(key)
            } else { None }
        }
    }

    pub fn insert(&mut self, interval: Interval<K>, value: V) {
        if interval.upper() <= self.interval.lower() {
            if let Some(left) = self.left.as_mut() {
                left.insert(interval, value);
            } else {
                self.left = Box::new(Some(IntervalMapNode::new(interval, value, None, None)));
            }
        } else if interval.lower() >= self.interval.upper() {
            if let Some(right) = self.right.as_mut() {
                right.insert(interval, value);
            } else {
                self.right = Box::new(Some(IntervalMapNode::new(interval, value, None, None)));
            }
        } else {
            panic!("intervals must not overlap");
        }
    }

    pub fn remove(mut self, interval: &Interval<K>) -> Option<IntervalMapNode<K, V>> {
        if interval.is_empty() {
            Some(self)
        } else if let Some(intersection) = interval.intersection(&self.interval) {
            if intersection.is_empty() {
                if interval.lower() >= self.interval.upper() {
                    if let Some(right) = self.right.take() {
                        self.right = Box::new(right.remove(interval));
                    }
                } else {
                    if let Some(left) = self.left.take() {
                        self.left = Box::new(left.remove(interval));
                    }
                }
                Some(self)
            } else {
                let mut result = match (*self.left, *self.right) {
                    (Some(left), Some(right)) => {
                        let (right, mut result) = right.remove_min_node();
                        result.right = Box::new(right);
                        result.left = Box::new(Some(left));
                        Some(result)
                    },
                    (Some(left), None) => Some(left),
                    (None, Some(right)) => Some(right),
                    (None, None) => None,
                };
                if interval.lower() < intersection.lower() {
                    result = if let Some(result) = result {
                        result.remove(&Interval::new(interval.lower().clone(), intersection.lower().clone()))
                    } else { None };
                } else if self.interval.lower() < intersection.lower() {
                    let interval = Interval::new(self.interval.lower().clone(), intersection.lower().clone());
                    if let Some(result) = result.as_mut() {
                        result.insert(interval, self.value.clone());
                    } else {
                        result = Some(IntervalMapNode::new(interval, self.value.clone(), None, None));
                    }
                }
                if interval.upper() > intersection.upper() {
                    result = if let Some(result) = result {
                        result.remove(&Interval::new(intersection.upper().clone(), interval.upper().clone()))
                    } else { None };
                } else if self.interval.upper() > intersection.upper() {
                    let interval = Interval::new(intersection.upper().clone(), self.interval.upper().clone());
                    if let Some(result) = result.as_mut() {
                        result.insert(interval, self.value);
                    } else {
                        result = Some(IntervalMapNode::new(interval, self.value, None, None));
                    }
                }
                result
            }
        } else {
            if interval.lower() > self.interval.upper() {
                if let Some(right) = self.right.take() {
                    self.right = Box::new(right.remove(interval));
                }
            } else {
                if let Some(left) = self.left.take() {
                    self.left = Box::new(left.remove(interval));
                }
            }
            Some(self)
        }
    }

    pub fn update_entry<F>(&mut self, _interval: &Interval<K>, _f: F)
    where
        F: FnMut(&Interval<K>, Option<V>) -> Option<V>
    {
        panic!("Not implemented")
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct IntervalMap<K, V> {
    root: Option<IntervalMapNode<K, V>>,
}

impl<K: Clone + PartialOrd, V: Clone> IntervalMap<K, V> {
    pub fn new() -> IntervalMap<K, V> {
        IntervalMap { root: None }
    }

    pub fn intervals(&self) -> Intervals<'_, K, V> {
        Intervals { inner: self.iter() }
    }

    pub fn values(&self) -> Values<'_, K, V> {
        Values { inner: self.iter() }
    }

    pub fn values_mut(&mut self) -> ValuesMut<'_, K, V> {
        ValuesMut { inner: self.iter_mut() }
    }

    pub fn iter(&self) -> Iter<'_, K, V> {
        Iter {
            current: self.root.as_ref(),
            stack: Vec::new(),
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, K, V> {
        IterMut {
            current: self.root.as_mut(),
            stack: Vec::new(),
        }
    }

    pub fn span(&self) -> Option<Interval<&K>> {
        self.root.as_ref().map(|root| root.span())
    }

    pub fn is_empty(&self) -> bool {
        self.root.is_none()
    }

    pub fn clear(&mut self) {
        self.root = None;
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        self.get_entry(key).map(|(_, value)| value)
    }

    pub fn get_entry(&self, key: &K) -> Option<(&Interval<K>, &V)> {
        self.root.as_ref().and_then(|root| root.get_entry(key))
    }

    pub fn contains_key(&self, key: &K) -> bool {
        self.get_entry(key).is_some()
    }

    pub fn insert(&mut self, interval: Interval<K>, value: V) {
        if let Some(root) = self.root.as_mut() {
            root.insert(interval, value);
        } else {
            self.root = Some(IntervalMapNode::new(interval, value, None, None));
        }
    }

    pub fn remove(&mut self, interval: &Interval<K>) {
        if let Some(root) = self.root.take() {
            self.root = root.remove(interval);
        }
    }

    pub fn update<F>(&mut self, interval: &Interval<K>, mut f: F) 
    where
        F: FnMut(Option<V>) -> Option<V>
    {
        self.update_entry(interval, |_, value| f(value));
    }

    pub fn update_entry<F>(&mut self, _inteval: &Interval<K>, _f: F)
    where
        F: FnMut(&Interval<K>, Option<V>) -> Option<V>
    {
        panic!("Not implemented")
    }
}

pub struct Intervals<'a, K, V> {
    inner: Iter<'a, K, V>
}

impl<'a, K, V> Iterator for Intervals<'a, K, V> {
    type Item = &'a Interval<K>;

    fn next(&mut self) -> Option<&'a Interval<K>> {
        self.inner.next().map(|(interval, _)| interval)
    }
}

pub struct Values<'a, K, V> {
    inner: Iter<'a, K, V>
}

impl<'a, K, V> Iterator for Values<'a, K, V> {
    type Item = &'a V;

    fn next(&mut self) -> Option<&'a V> {
        self.inner.next().map(|(_, value)| value)
    }
}

pub struct ValuesMut<'a, K, V> {
    inner: IterMut<'a, K, V>
}

impl<'a, K, V> Iterator for ValuesMut<'a, K, V> {
    type Item = &'a mut V;

    fn next(&mut self) -> Option<&'a mut V> {
        self.inner.next().map(|(_, value)| value)
    }
}

pub struct Iter<'a, K, V> {
    current: Option<&'a IntervalMapNode<K, V>>,
    stack: Vec<(&'a Interval<K>, &'a V, Option<&'a IntervalMapNode<K, V>>)>,
}

impl<'a, K, V> Iterator for Iter<'a, K, V> {
    type Item = (&'a Interval<K>, &'a V);

    fn next(&mut self) -> Option<(&'a Interval<K>, &'a V)> {
        while let Some(current) = self.current.take() {
            self.stack.push((&current.interval, &current.value, (*current.right).as_ref()));
            self.current = (*current.left).as_ref();
        }
        if let Some((interval, value, right)) = self.stack.pop() {
            self.current = right;
            Some((interval, value))
        } else { None }
    }
}

pub struct IterMut<'a, K, V> {
    current: Option<&'a mut IntervalMapNode<K, V>>,
    stack: Vec<(&'a Interval<K>, &'a mut V, Option<&'a mut IntervalMapNode<K, V>>)>,
}

impl<'a, K, V> Iterator for IterMut<'a, K, V> {
    type Item = (&'a Interval<K>, &'a mut V);

    fn next(&mut self) -> Option<(&'a Interval<K>, &'a mut V)> {
        while let Some(current) = self.current.take() {
            self.stack.push((&current.interval, &mut current.value, (*current.right).as_mut()));
            self.current = (*current.left).as_mut();
        }
        if let Some((interval, value, right)) = self.stack.pop() {
            self.current = right;
            Some((interval, value))
        } else { None }
    }
}

impl<K: Clone + PartialOrd, V: Clone> Extend<(Interval<K>, V)> for IntervalMap<K, V> {
    fn extend<I: IntoIterator<Item = (Interval<K>, V)>>(&mut self, iter: I) {
        for (interval, value) in iter {
            self.insert(interval, value);
        }
    }
}

impl<K, V> IntoIterator for IntervalMap<K, V> {
    type Item = (Interval<K>, V);
    type IntoIter = IntoIter<K, V>;

    fn into_iter(self) -> IntoIter<K, V> {
        IntoIter {
            current: self.root,
            stack: Vec::new(),
        }
    }
}

pub struct IntoIter<K, V> {
    current: Option<IntervalMapNode<K, V>>,
    stack: Vec<(Interval<K>, V, Option<IntervalMapNode<K, V>>)>,
}

impl<K, V> Iterator for IntoIter<K, V> {
    type Item = (Interval<K>, V);

    fn next(&mut self) -> Option<(Interval<K>, V)> {
        while let Some(current) = self.current.take() {
            self.stack.push((current.interval, current.value, *current.right));
            self.current = *current.left;
        }
        if let Some((interval, value, right)) = self.stack.pop() {
            self.current = right;
            Some((interval, value))
        } else { None }
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

#[cfg(test)]
mod tests {
    use crate::{
        Interval,
        IntervalMap,
    };

    #[test]
    fn test_remove() {

        let permutations = vec![

            // [0----)
            //      \
            //     [1----)
            //          \
            //         [2----)

            vec![0, 1, 2],

            // [0----)
            //      \
            //     [2----)
            //      /
            // [1----)

            vec![0, 2, 1],

            //     [1----)
            //      /   \
            // [0----) [2----)

            vec![1, 0, 2],

            //     [2----)
            //      /
            // [0----)
            //      \
            //     [1----)

            vec![2, 0, 1],

            //         [2----)
            //          /
            //     [1----)
            //      /
            // [0----)

            vec![2, 1, 0]

        ];

        let cases = vec![

            // [-----------------)
            //                       -> -------------------
            // [0----|1----|2----)

            (
                Interval::new(0, 18),
                vec![
                    (Interval::new(0, 6), 0),
                    (Interval::new(6, 12), 1),
                    (Interval::new(12, 18), 2)
                ],
                vec![],
            ),

            // [--------------)---
            //                       -> ---------------[2-)
            // [0----|1----|2----)

            (
                Interval::new(0, 15),
                vec![
                    (Interval::new(0, 6), 0),
                    (Interval::new(6, 12), 1),
                    (Interval::new(12, 18), 2)
                ],
                vec![
                    (Interval::new(15, 18), 2)
                ],
            ),

            // [-----------)------
            //                       -> ------------[2----)
            // [0----|1----|2----)

            (
                Interval::new(0, 12),
                vec![
                    (Interval::new(0, 6), 0),
                    (Interval::new(6, 12), 1),
                    (Interval::new(12, 18), 2)
                ],
                vec![
                    (Interval::new(12, 18), 2)
                ],
            ),

            // [--------)---------
            //                       -> ---------[1-|2----)
            // [0----|1----|2----)

            (
                Interval::new(0, 9),
                vec![
                    (Interval::new(0, 6), 0),
                    (Interval::new(6, 12), 1),
                    (Interval::new(12, 18), 2)
                ],
                vec![
                    (Interval::new(9, 12), 1),
                    (Interval::new(12, 18), 2)
                ],
            ),

            // [-----)------------
            //                       -> ------[1----|2----)
            // [0----|1----|2----)

            (
                Interval::new(0, 6),
                vec![
                    (Interval::new(0, 6), 0),
                    (Interval::new(6, 12), 1),
                    (Interval::new(12, 18), 2)
                ],
                vec![
                    (Interval::new(6, 12), 1),
                    (Interval::new(12, 18), 2)
                ],
            ),

            // [--)---------------
            //                       -> ---[0-|1----|2----)
            // [0----|1----|2----)

            (
                Interval::new(0, 3),
                vec![
                    (Interval::new(0, 6), 0),
                    (Interval::new(6, 12), 1),
                    (Interval::new(12, 18), 2)
                ],
                vec![
                    (Interval::new(3, 6), 0),
                    (Interval::new(6, 12), 1),
                    (Interval::new(12, 18), 2)
                ],
            ),

            // |------------------
            //                       -> [0----|1----|2----)
            // [0----|1----|2----)

            (
                Interval::new(0, 0),
                vec![
                    (Interval::new(0, 6), 0),
                    (Interval::new(6, 12), 1),
                    (Interval::new(12, 18), 2)
                ],
                vec![
                    (Interval::new(0, 6), 0),
                    (Interval::new(6, 12), 1),
                    (Interval::new(12, 18), 2)
                ],
            ),

            // ---[--------------)
            //                       -> [0-)---------------
            // [0----|1----|2----)

            (
                Interval::new(3, 18),
                vec![
                    (Interval::new(0, 6), 0),
                    (Interval::new(6, 12), 1),
                    (Interval::new(12, 18), 2)
                ],
                vec![
                    (Interval::new(0, 3), 0)
                ],
            ),

            // ---[-----------)---
            //                       -> [0-)-----------[2-)
            // [0----|1----|2----)

            (
                Interval::new(3, 15),
                vec![
                    (Interval::new(0, 6), 0),
                    (Interval::new(6, 12), 1),
                    (Interval::new(12, 18), 2)
                ],
                vec![
                    (Interval::new(0, 3), 0),
                    (Interval::new(15, 18), 2)
                ],
            ),

            // ---[--------)------
            //                       -> [0-)--------[2----)
            // [0----|1----|2----)

            (
                Interval::new(3, 12),
                vec![
                    (Interval::new(0, 6), 0),
                    (Interval::new(6, 12), 1),
                    (Interval::new(12, 18), 2)
                ],
                vec![
                    (Interval::new(0, 3), 0),
                    (Interval::new(12, 18), 2)
                ],
            ),

            // ---[-----)---------
            //                       -> [0-)-----[1-|2----)
            // [0----|1----|2----)

            (
                Interval::new(3, 9),
                vec![
                    (Interval::new(0, 6), 0),
                    (Interval::new(6, 12), 1),
                    (Interval::new(12, 18), 2)
                ],
                vec![
                    (Interval::new(0, 3), 0),
                    (Interval::new(9, 12), 1),
                    (Interval::new(12, 18), 2)
                ],
            ),

            // ---[--)------------
            //                       -> [0-)--[1----|2----)
            // [0----|1----|2----)

            (
                Interval::new(3, 6),
                vec![
                    (Interval::new(0, 6), 0),
                    (Interval::new(6, 12), 1),
                    (Interval::new(12, 18), 2)
                ],
                vec![
                    (Interval::new(0, 3), 0),
                    (Interval::new(6, 12), 1),
                    (Interval::new(12, 18), 2)
                ],
            ),

            // --[-)--------------
            //                       -> [0)-[0|1----|2----)
            // [0----|1----|2----)

            (
                Interval::new(2, 4),
                vec![
                    (Interval::new(0, 6), 0),
                    (Interval::new(6, 12), 1),
                    (Interval::new(12, 18), 2)
                ],
                vec![
                    (Interval::new(0, 2), 0),
                    (Interval::new(4, 6), 0),
                    (Interval::new(6, 12), 1),
                    (Interval::new(12, 18), 2)
                ],
            ),

            // ---|---------------
            //                       -> [0----|1----|2----)
            // [0----|1----|2----)

            (
                Interval::new(3, 3),
                vec![
                    (Interval::new(0, 6), 0),
                    (Interval::new(6, 12), 1),
                    (Interval::new(12, 18), 2)
                ],
                vec![
                    (Interval::new(0, 6), 0),
                    (Interval::new(6, 12), 1),
                    (Interval::new(12, 18), 2)
                ],
            ),

            // ------[-----------)
            //                       -> [0----)------------
            // [0----|1----|2----)

            (
                Interval::new(6, 18),
                vec![
                    (Interval::new(0, 6), 0),
                    (Interval::new(6, 12), 1),
                    (Interval::new(12, 18), 2)
                ],
                vec![
                    (Interval::new(0, 6), 0)
                ],
            ),

            // ------[--------)---
            //                       -> [0----)--------[2-)
            // [0----|1----|2----)

            (
                Interval::new(6, 15),
                vec![
                    (Interval::new(0, 6), 0),
                    (Interval::new(6, 12), 1),
                    (Interval::new(12, 18), 2)
                ],
                vec![
                    (Interval::new(0, 6), 0),
                    (Interval::new(15, 18), 2)
                ],
            ),

            // ------[-----)------
            //                       -> [0----)-----[2----)
            // [0----|1----|2----)

            (
                Interval::new(6, 12),
                vec![
                    (Interval::new(0, 6), 0),
                    (Interval::new(6, 12), 1),
                    (Interval::new(12, 18), 2)
                ],
                vec![
                    (Interval::new(0, 6), 0),
                    (Interval::new(12, 18), 2)
                ],
            ),

            // ------[--)---------
            //                       -> [0----)--[1-|2----)
            // [0----|1----|2----)

            (
                Interval::new(6, 9),
                vec![
                    (Interval::new(0, 6), 0),
                    (Interval::new(6, 12), 1),
                    (Interval::new(12, 18), 2)
                ],
                vec![
                    (Interval::new(0, 6), 0),
                    (Interval::new(9, 12), 1),
                    (Interval::new(12, 18), 2)
                ],
            ),

            // ------|------------
            //                       -> [0----|1----|2----)
            // [0----|1----|2----)

            (
                Interval::new(6, 6),
                vec![
                    (Interval::new(0, 6), 0),
                    (Interval::new(6, 12), 1),
                    (Interval::new(12, 18), 2)
                ],
                vec![
                    (Interval::new(0, 6), 0),
                    (Interval::new(6, 12), 1),
                    (Interval::new(12, 18), 2)
                ],
            ),

        ];

        for (remove_interval, insert_intervals, expected_intervals) in cases {
            for indices in &permutations {
                let mut interval_map = IntervalMap::new();
                for &index in indices {
                    let (insert_interval, insert_value) = insert_intervals[index];
                    interval_map.insert(insert_interval, insert_value);
                }
                interval_map.remove(&remove_interval);
                assert_eq!(expected_intervals, interval_map.into_iter().collect::<Vec<_>>());
            }
        }
    }

    #[test]
    fn test_update() {

        let permutations = vec![

            vec![],

            vec![

                // [0----)

                vec![0]

            ],

            vec![

                // [0----)
                //      \
                //     [1----)

                vec![0, 1],

                //     [1----)
                //      /
                // [0----)

                vec![1, 0]
            ],

            vec![

                // [0----)
                //      \
                //     [1----)
                //          \
                //         [2----)

                vec![0, 1, 2],

                // [0----)
                //      \
                //     [2----)
                //      /
                // [1----)

                vec![0, 2, 1],

                //     [1----)
                //      /   \
                // [0----) [2----)

                vec![1, 0, 2],

                //     [2----)
                //      /
                // [0----)
                //      \
                //     [1----)

                vec![2, 0, 1],

                //         [2----)
                //          /
                //     [1----)
                //      /
                // [0----)

                vec![2, 1, 0]

            ]

        ];

        let cases = vec![

            // [3----------------)
            //                       -> [3----|3----|3----)
            // [0----|1----|2----)

            (
                (Interval::new(0, 18), 3),
                vec![
                    (Interval::new(0, 6), 0),
                    (Interval::new(6, 12), 1),
                    (Interval::new(12, 18), 2)
                ],
                vec![
                    (Interval::new(0, 6), 3),
                    (Interval::new(6, 12), 3),
                    (Interval::new(12, 18), 3)
                ],
            ),

            // [3-------------)---
            //                       -> [3----|3----|3-|2-)
            // [0----|1----|2----)

            (
                (Interval::new(0, 15), 3),
                vec![
                    (Interval::new(0, 6), 0),
                    (Interval::new(6, 12), 1),
                    (Interval::new(12, 18), 2)
                ],
                vec![
                    (Interval::new(0, 6), 3),
                    (Interval::new(6, 12), 3),
                    (Interval::new(12, 15), 3),
                    (Interval::new(15, 18), 2)
                ],
            ),

            // [3----------)------
            //                       -> [3----|3----|2----)
            // [0----|1----|2----)

            (
                (Interval::new(0, 12), 3),
                vec![
                    (Interval::new(0, 6), 0),
                    (Interval::new(6, 12), 1),
                    (Interval::new(12, 18), 2)
                ],
                vec![
                    (Interval::new(0, 6), 3),
                    (Interval::new(6, 12), 3),
                    (Interval::new(12, 18), 2)
                ],
            ),

            // [3-------)---------
            //                       -> [3----|3-|1-|2----)
            // [0----|1----|2----)

            (
                (Interval::new(0, 9), 3),
                vec![
                    (Interval::new(0, 6), 0),
                    (Interval::new(6, 12), 1),
                    (Interval::new(12, 18), 2)
                ],
                vec![
                    (Interval::new(0, 6), 3),
                    (Interval::new(6, 9), 3),
                    (Interval::new(9, 12), 1),
                    (Interval::new(12, 18), 2)
                ],
            ),

            // [3----)------------
            //                       -> [3----|1----|2----)
            // [0----|1----|2----)

            (
                (Interval::new(0, 6), 3),
                vec![
                    (Interval::new(0, 6), 0),
                    (Interval::new(6, 12), 1),
                    (Interval::new(12, 18), 2)
                ],
                vec![
                    (Interval::new(0, 6), 3),
                    (Interval::new(6, 12), 1),
                    (Interval::new(12, 18), 2)
                ],
            ),

            // [3-)---------------
            //                       -> [3-|0-|1----|2----)
            // [0----|1----|2----)

            (
                (Interval::new(0, 3), 3),
                vec![
                    (Interval::new(0, 6), 0),
                    (Interval::new(6, 12), 1),
                    (Interval::new(12, 18), 2)
                ],
                vec![
                    (Interval::new(0, 3), 3),
                    (Interval::new(3, 6), 0),
                    (Interval::new(6, 12), 1),
                    (Interval::new(12, 18), 2)
                ],
            ),

            // |------------------
            //                       -> [0----|1----|2----)
            // [0----|1----|2----)

            (
                (Interval::new(0, 0), 3),
                vec![
                    (Interval::new(0, 6), 0),
                    (Interval::new(6, 12), 1),
                    (Interval::new(12, 18), 2)
                ],
                vec![
                    (Interval::new(0, 6), 0),
                    (Interval::new(6, 12), 1),
                    (Interval::new(12, 18), 2)
                ],
            ),

            // ---[3-------------)
            //                       -> [0-|3-|3----|3----)
            // [0----|1----|2----)

            (
                (Interval::new(3, 18), 3),
                vec![
                    (Interval::new(0, 6), 0),
                    (Interval::new(6, 12), 1),
                    (Interval::new(12, 18), 2)
                ],
                vec![
                    (Interval::new(0, 3), 0),
                    (Interval::new(3, 6), 3),
                    (Interval::new(6, 12), 3),
                    (Interval::new(12, 18), 3)
                ],
            ),

            // ---[3----------)---
            //                       -> [0-|3-|3----|3-|2-)
            // [0----|1----|2----)

            (
                (Interval::new(3, 15), 3),
                vec![
                    (Interval::new(0, 6), 0),
                    (Interval::new(6, 12), 1),
                    (Interval::new(12, 18), 2)
                ],
                vec![
                    (Interval::new(0, 3), 0),
                    (Interval::new(3, 6), 3),
                    (Interval::new(6, 12), 3),
                    (Interval::new(12, 15), 3),
                    (Interval::new(15, 18), 2)
                ],
            ),

            // ---[3-------)------
            //                       -> [0-|3-|3----|2----)
            // [0----|1----|2----)

            (
                (Interval::new(3, 12), 3),
                vec![
                    (Interval::new(0, 6), 0),
                    (Interval::new(6, 12), 1),
                    (Interval::new(12, 18), 2)
                ],
                vec![
                    (Interval::new(0, 3), 0),
                    (Interval::new(3, 6), 3),
                    (Interval::new(6, 12), 3),
                    (Interval::new(12, 18), 2)
                ],
            ),

            // ---[3----)---------
            //                       -> [0-|3-|3-|1-|2----)
            // [0----|1----|2----)

            (
                (Interval::new(3, 9), 3),
                vec![
                    (Interval::new(0, 6), 0),
                    (Interval::new(6, 12), 1),
                    (Interval::new(12, 18), 2)
                ],
                vec![
                    (Interval::new(0, 3), 0),
                    (Interval::new(3, 6), 3),
                    (Interval::new(6, 9), 3),
                    (Interval::new(9, 12), 1),
                    (Interval::new(12, 18), 2)
                ],
            ),

            // ---[3-)------------
            //                       -> [0-|3-|1----|2----)
            // [0----|1----|2----)

            (
                (Interval::new(3, 6), 3),
                vec![
                    (Interval::new(0, 6), 0),
                    (Interval::new(6, 12), 1),
                    (Interval::new(12, 18), 2)
                ],
                vec![
                    (Interval::new(0, 3), 0),
                    (Interval::new(3, 6), 3),
                    (Interval::new(6, 12), 1),
                    (Interval::new(12, 18), 2)
                ],
            ),

            // --[3)--------------
            //                       -> [0|3|0|1----|2----)
            // [0----|1----|2----)

            (
                (Interval::new(2, 4), 3),
                vec![
                    (Interval::new(0, 6), 0),
                    (Interval::new(6, 12), 1),
                    (Interval::new(12, 18), 2)
                ],
                vec![
                    (Interval::new(0, 2), 0),
                    (Interval::new(2, 4), 3),
                    (Interval::new(4, 6), 0),
                    (Interval::new(6, 12), 1),
                    (Interval::new(12, 18), 2)
                ],
            ),

            // ---|---------------
            //                       -> [0----|1----|2----)
            // [0----|1----|2----)

            (
                (Interval::new(3, 3), 3),
                vec![
                    (Interval::new(0, 6), 0),
                    (Interval::new(6, 12), 1),
                    (Interval::new(12, 18), 2)
                ],
                vec![
                    (Interval::new(0, 6), 0),
                    (Interval::new(6, 12), 1),
                    (Interval::new(12, 18), 2)
                ],
            ),

            // ------[3----------)
            //                       -> [0----|3----|3----)
            // [0----|1----|2----)

            (
                (Interval::new(6, 18), 3),
                vec![
                    (Interval::new(0, 6), 0),
                    (Interval::new(6, 12), 1),
                    (Interval::new(12, 18), 2)
                ],
                vec![
                    (Interval::new(0, 6), 0),
                    (Interval::new(6, 12), 3),
                    (Interval::new(12, 18), 3)
                ],
            ),

            // ------[3-------)---
            //                       -> [0----|3----|3-|2-)
            // [0----|1----|2----)

            (
                (Interval::new(6, 15), 3),
                vec![
                    (Interval::new(0, 6), 0),
                    (Interval::new(6, 12), 1),
                    (Interval::new(12, 18), 2)
                ],
                vec![
                    (Interval::new(0, 6), 0),
                    (Interval::new(6, 12), 3),
                    (Interval::new(12, 15), 3),
                    (Interval::new(15, 18), 2)
                ],
            ),

            // ------[3----)------
            //                       -> [0----|3----|2----)
            // [0----|1----|2----)

            (
                (Interval::new(6, 12), 3),
                vec![
                    (Interval::new(0, 6), 0),
                    (Interval::new(6, 12), 1),
                    (Interval::new(12, 18), 2)
                ],
                vec![
                    (Interval::new(0, 6), 0),
                    (Interval::new(6, 12), 3),
                    (Interval::new(12, 18), 2)
                ],
            ),

            // ------[3-)---------
            //                       -> [0----|3-|1-|2----)
            // [0----|1----|2----)

            (
                (Interval::new(6, 9), 3),
                vec![
                    (Interval::new(0, 6), 0),
                    (Interval::new(6, 12), 1),
                    (Interval::new(12, 18), 2)
                ],
                vec![
                    (Interval::new(0, 6), 0),
                    (Interval::new(6, 9), 3),
                    (Interval::new(9, 12), 1),
                    (Interval::new(12, 18), 2)
                ],
            ),

            // ------|------------
            //                       -> [0----|1----|2----)
            // [0----|1----|2----)

            (
                (Interval::new(6, 6), 3),
                vec![
                    (Interval::new(0, 6), 0),
                    (Interval::new(6, 12), 1),
                    (Interval::new(12, 18), 2)
                ],
                vec![
                    (Interval::new(0, 6), 0),
                    (Interval::new(6, 12), 1),
                    (Interval::new(12, 18), 2)
                ],
            ),

            // [3----------------)
            //                       -> [3----|3----|3----)
            // ------[1----|2----)

            (
                (Interval::new(0, 18), 3),
                vec![
                    (Interval::new(6, 12), 1),
                    (Interval::new(12, 18), 2)
                ],
                vec![
                    (Interval::new(0, 6), 3),
                    (Interval::new(6, 12), 3),
                    (Interval::new(12, 18), 3)
                ],
            ),

            // [3-------------)---
            //                       -> [3----|3----|3-|2-)
            // ------[1----|2----)

            (
                (Interval::new(0, 15), 3),
                vec![
                    (Interval::new(6, 12), 1),
                    (Interval::new(12, 18), 2)
                ],
                vec![
                    (Interval::new(0, 6), 3),
                    (Interval::new(6, 12), 3),
                    (Interval::new(12, 15), 3),
                    (Interval::new(15, 18), 2)
                ],
            ),

            // [3----------)------
            //                       -> [3----|3----|2----)
            // ------[1----|2----)

            (
                (Interval::new(0, 12), 3),
                vec![
                    (Interval::new(6, 12), 1),
                    (Interval::new(12, 18), 2)
                ],
                vec![
                    (Interval::new(0, 6), 3),
                    (Interval::new(6, 12), 3),
                    (Interval::new(12, 18), 2)
                ],
            ),

            // [3-------)---------
            //                       -> [3----|3-|1-|2----)
            // ------[1----|2----)

            (
                (Interval::new(0, 9), 3),
                vec![
                    (Interval::new(6, 12), 1),
                    (Interval::new(12, 18), 2)
                ],
                vec![
                    (Interval::new(0, 6), 3),
                    (Interval::new(6, 9), 3),
                    (Interval::new(9, 12), 1),
                    (Interval::new(12, 18), 2)
                ],
            ),

            // [3----)------------
            //                       -> [3----|1----|2----)
            // ------[1----|2----)

            (
                (Interval::new(0, 6), 3),
                vec![
                    (Interval::new(6, 12), 1),
                    (Interval::new(12, 18), 2)
                ],
                vec![
                    (Interval::new(0, 6), 3),
                    (Interval::new(6, 12), 1),
                    (Interval::new(12, 18), 2)
                ],
            ),

            // [3-)---------------
            //                       -> [3-)--[1----|2----)
            // ------[1----|2----)

            (
                (Interval::new(0, 3), 3),
                vec![
                    (Interval::new(6, 12), 1),
                    (Interval::new(12, 18), 2)
                ],
                vec![
                    (Interval::new(0, 3), 3),
                    (Interval::new(6, 12), 1),
                    (Interval::new(12, 18), 2)
                ],
            ),

            // |------------------
            //                       -> ------[1----|2----)
            // ------[1----|2----)

            (
                (Interval::new(0, 0), 3),
                vec![
                    (Interval::new(6, 12), 1),
                    (Interval::new(12, 18), 2)
                ],
                vec![
                    (Interval::new(0, 6), 0),
                    (Interval::new(6, 12), 1),
                    (Interval::new(12, 18), 2)
                ],
            ),

            // [3----------------)
            //                       -> [3----|3----|3----)
            // [0----)-----[2----)

            (
                (Interval::new(0, 18), 3),
                vec![
                    (Interval::new(0, 6), 0),
                    (Interval::new(12, 18), 2)
                ],
                vec![
                    (Interval::new(0, 6), 3),
                    (Interval::new(6, 12), 3),
                    (Interval::new(12, 18), 3)
                ],
            ),

            // [3-------------)---
            //                       -> [3----|3----|3-|2-)
            // [0----)-----[2----)

            (
                (Interval::new(0, 15), 3),
                vec![
                    (Interval::new(0, 6), 0),
                    (Interval::new(12, 18), 2)
                ],
                vec![
                    (Interval::new(0, 6), 3),
                    (Interval::new(6, 12), 3),
                    (Interval::new(12, 15), 3),
                    (Interval::new(15, 18), 2)
                ],
            ),

            // [3----------)------
            //                       -> [3----|3----|2----)
            // [0----)-----[2----)

            (
                (Interval::new(0, 12), 3),
                vec![
                    (Interval::new(0, 6), 0),
                    (Interval::new(12, 18), 2)
                ],
                vec![
                    (Interval::new(0, 6), 3),
                    (Interval::new(6, 12), 3),
                    (Interval::new(12, 18), 2)
                ],
            ),

            // [3-------)---------
            //                       -> [3----|3-)--[2----)
            // [0----)-----[2----)

            (
                (Interval::new(0, 9), 3),
                vec![
                    (Interval::new(0, 6), 0),
                    (Interval::new(12, 18), 2)
                ],
                vec![
                    (Interval::new(0, 6), 3),
                    (Interval::new(6, 9), 3),
                    (Interval::new(12, 18), 2)
                ],
            ),

            // ---[3-------------)
            //                       -> [0-|3-|3----|3----)
            // [0----)-----[2----)

            (
                (Interval::new(3, 18), 3),
                vec![
                    (Interval::new(0, 6), 0),
                    (Interval::new(12, 18), 2)
                ],
                vec![
                    (Interval::new(0, 3), 0),
                    (Interval::new(3, 6), 3),
                    (Interval::new(6, 12), 3),
                    (Interval::new(12, 18), 3)
                ],
            ),

            // ---[3----------)---
            //                       -> [0-|3-|3----|3-|2-)
            // [0----)-----[2----)

            (
                (Interval::new(3, 15), 3),
                vec![
                    (Interval::new(0, 6), 0),
                    (Interval::new(12, 18), 2)
                ],
                vec![
                    (Interval::new(0, 3), 0),
                    (Interval::new(3, 6), 3),
                    (Interval::new(6, 12), 3),
                    (Interval::new(12, 15), 3),
                    (Interval::new(15, 18), 2)
                ],
            ),

            // ---[3-------)------
            //                       -> [0-|3-|3----|2----)
            // [0----)-----[2----)

            (
                (Interval::new(3, 12), 3),
                vec![
                    (Interval::new(0, 6), 0),
                    (Interval::new(12, 18), 2)
                ],
                vec![
                    (Interval::new(0, 3), 0),
                    (Interval::new(3, 6), 3),
                    (Interval::new(6, 12), 3),
                    (Interval::new(12, 18), 2)
                ],
            ),

            // ---[3----)---------
            //                       -> [0-|3-|3-)--[2----)
            // [0----)-----[2----)

            (
                (Interval::new(3, 9), 3),
                vec![
                    (Interval::new(0, 6), 0),
                    (Interval::new(12, 18), 2)
                ],
                vec![
                    (Interval::new(0, 3), 0),
                    (Interval::new(3, 6), 3),
                    (Interval::new(6, 9), 3),
                    (Interval::new(12, 18), 2)
                ],
            ),

            // [3----------------)
            //                       -> [3----|3----|3----)
            // [0----|1----)------

            (
                (Interval::new(0, 18), 3),
                vec![
                    (Interval::new(0, 6), 0),
                    (Interval::new(6, 12), 1)
                ],
                vec![
                    (Interval::new(0, 6), 3),
                    (Interval::new(6, 12), 3),
                    (Interval::new(12, 18), 3)
                ],
            ),

            // [3-------------)---
            //                       -> [3----|3----|3-)---
            // [0----|1----)------

            (
                (Interval::new(0, 15), 3),
                vec![
                    (Interval::new(0, 6), 0),
                    (Interval::new(6, 12), 1)
                ],
                vec![
                    (Interval::new(0, 6), 3),
                    (Interval::new(6, 12), 3),
                    (Interval::new(12, 15), 3)
                ],
            ),

            // ---[3-------------)
            //                       -> [0-|3-|3----|3----)
            // [0----|1----)------

            (
                (Interval::new(3, 18), 3),
                vec![
                    (Interval::new(0, 6), 0),
                    (Interval::new(6, 12), 1)
                ],
                vec![
                    (Interval::new(0, 3), 0),
                    (Interval::new(3, 6), 3),
                    (Interval::new(6, 12), 3),
                    (Interval::new(12, 18), 3)
                ],
            ),

            // [3----------------)
            //                       -> [3----|3----|3----)
            // ------[1----)------

            (
                (Interval::new(0, 18), 3),
                vec![
                    (Interval::new(6, 12), 1)
                ],
                vec![
                    (Interval::new(0, 6), 3),
                    (Interval::new(6, 12), 3),
                    (Interval::new(12, 18), 3)
                ],
            ),

        ];

        for (update_interval, insert_intervals, expected_intervals) in cases {
            for indices in &permutations[insert_intervals.len()] {
                let mut interval_map = IntervalMap::new();
                for &index in indices {
                    let (insert_interval, insert_value) = insert_intervals[index];
                    interval_map.insert(insert_interval, insert_value);
                }
                let (update_interval, update_value) = update_interval;
                interval_map.update(&update_interval, |_| Some(update_value));
                assert_eq!(expected_intervals, interval_map.into_iter().collect::<Vec<_>>());
            }
        }
    }
}
