use crate::{
    interval_map_node::IntervalMapNode,
    Interval,
};

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
        self.root.as_ref().and_then(|root| root.get(key))
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

    pub fn update<F>(&mut self, interval: &Interval<K>, value: F) 
    where
        F: Fn(Option<V>) -> Option<V> + Clone
    {
        if let Some(root) = self.root.take() {
            self.root = root.update(interval, value);
        } else if let Some(value) = value(None) {
            self.insert(interval.clone(), value);
        }
    }

    pub fn update_entry<F>(&mut self, interval: &Interval<K>, value: F)
    where
        F: Fn(&Interval<K>, Option<V>) -> Option<V> + Clone
    {
        if let Some(root) = self.root.take() {
            self.root = root.update_entry(interval, value);
        } else if let Some(value) = value(interval, None) {
            self.insert(interval.clone(), value);
        }
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
        let permutations = vec![(
                format!("{}\n{}\n{}\n{}\n{}\n",
                    "  [0----)",
                    "       \\",
                    "      [1----)",
                    "           \\",
                    "          [2----)"
                ),
                vec![0, 1, 2]
            ), (
                format!("{}\n{}\n{}\n{}\n{}\n",
                    "  [0----)",
                    "       \\",
                    "      [2----)",
                    "       /",
                    "  [1----)"
                ),
                vec![0, 2, 1]
            ), (
                format!("{}\n{}\n{}\n",
                    "      [1----)",
                    "       /   \\",
                    "  [0----) [2----)",
                ),
                vec![1, 0, 2]
            ), (
                format!("{}\n{}\n{}\n{}\n{}\n",
                    "      [2----)",
                    "       /",
                    "  [0----)",
                    "       \\",
                    "      [1----)"
                ),
                vec![2, 0, 1]
            ), (
                format!("{}\n{}\n{}\n{}\n{}\n",
                    "          [2----)",
                    "           /",
                    "      [1----)",
                    "       /",
                    "  [0----)"
                ),
                vec![2, 1, 0]
            )
        ];
        let cases = vec![(
                format!("{}\n{}\n{}\n",
                    "  [-----------------)",
                    "                      -> -------------------",
                    "  [0----|1----|2----)"
                ),
                Interval::new(0, 18),
                vec![
                    (Interval::new(0, 6), 0),
                    (Interval::new(6, 12), 1),
                    (Interval::new(12, 18), 2)
                ],
                vec![],
            ), (
                format!("{}\n{}\n{}\n",
                    "  [--------------)---",
                    "                      -> ---------------[2-)",
                    "  [0----|1----|2----)"
                ),
                Interval::new(0, 15),
                vec![
                    (Interval::new(0, 6), 0),
                    (Interval::new(6, 12), 1),
                    (Interval::new(12, 18), 2)
                ],
                vec![
                    (Interval::new(15, 18), 2)
                ],
            ), (
                format!("{}\n{}\n{}\n",
                    "  [-----------)------",
                    "                      -> ------------[2----)",
                    "  [0----|1----|2----)"
                ),
                Interval::new(0, 12),
                vec![
                    (Interval::new(0, 6), 0),
                    (Interval::new(6, 12), 1),
                    (Interval::new(12, 18), 2)
                ],
                vec![
                    (Interval::new(12, 18), 2)
                ],
            ), (
                format!("{}\n{}\n{}\n",
                    "  [--------)---------",
                    "                      -> ---------[1-|2----)",
                    "  [0----|1----|2----)"
                ),
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
            ), (
                format!("{}\n{}\n{}\n",
                    "  [-----)------------",
                    "                      -> ------[1----|2----)",
                    "  [0----|1----|2----)"
                ),
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
            ), (
                format!("{}\n{}\n{}\n",
                    "  [--)---------------",
                    "                      -> ---[0-|1----|2----)",
                    "  [0----|1----|2----)"
                ),
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
            ), (
                format!("{}\n{}\n{}\n",
                    "  |------------------",
                    "                      -> [0----|1----|2----)",
                    "  [0----|1----|2----)"
                ),
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
            ), (
                format!("{}\n{}\n{}\n",
                    "  ---[--------------)",
                    "                      -> [0-)---------------",
                    "  [0----|1----|2----)"
                ),
                Interval::new(3, 18),
                vec![
                    (Interval::new(0, 6), 0),
                    (Interval::new(6, 12), 1),
                    (Interval::new(12, 18), 2)
                ],
                vec![
                    (Interval::new(0, 3), 0)
                ],
            ), (
                format!("{}\n{}\n{}\n",
                    "  ---[-----------)---",
                    "                      -> [0-)-----------[2-)",
                    "  [0----|1----|2----)"
                ),
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
            ), (
                format!("{}\n{}\n{}\n",
                    "  ---[--------)------",
                    "                      -> [0-)--------[2----)",
                    "  [0----|1----|2----)"
                ),
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
            ), (
                format!("{}\n{}\n{}\n",
                    "  ---[-----)---------",
                    "                      -> [0-)-----[1-|2----)",
                    "  [0----|1----|2----)"
                ),
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
            ), (
                format!("{}\n{}\n{}\n",
                    "  ---[--)------------",
                    "                      -> [0-)--[1----|2----)",
                    "  [0----|1----|2----)"
                ),
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
            ), (
                format!("{}\n{}\n{}\n",
                    "  --[-)--------------",
                    "                      -> [0)-[0|1----|2----)",
                    "  [0----|1----|2----)"
                ),
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
            ), (
                format!("{}\n{}\n{}\n",
                    "  ---|---------------",
                    "                      -> [0----|1----|2----)",
                    "  [0----|1----|2----)"
                ),
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
            ), (
                format!("{}\n{}\n{}\n",
                    "  ------[-----------)",
                    "                      -> [0----)------------",
                    "  [0----|1----|2----)"
                ),
                Interval::new(6, 18),
                vec![
                    (Interval::new(0, 6), 0),
                    (Interval::new(6, 12), 1),
                    (Interval::new(12, 18), 2)
                ],
                vec![
                    (Interval::new(0, 6), 0)
                ],
            ), (
                format!("{}\n{}\n{}\n",
                    "  ------[--------)---",
                    "                      -> [0----)--------[2-)",
                    "  [0----|1----|2----)"
                ),
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
            ), (
                format!("{}\n{}\n{}\n",
                    "  ------[-----)------",
                    "                      -> [0----)-----[2----)",
                    "  [0----|1----|2----)"
                ),
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
            ), (
                format!("{}\n{}\n{}\n",
                    "  ------[--)---------",
                    "                      -> [0----)--[1-|2----)",
                    "  [0----|1----|2----)"
                ),
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
            ), (
                format!("{}\n{}\n{}\n",
                    "  ------|------------",
                    "                      -> [0----|1----|2----)",
                    "  [0----|1----|2----)"
                ),
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
        for (case_description, remove_interval, insert_intervals, expected_intervals) in cases {
            for (permutation_description, indices) in &permutations {
                let mut interval_map = IntervalMap::new();
                for &index in indices {
                    let (insert_interval, insert_value) = insert_intervals[index];
                    interval_map.insert(insert_interval, insert_value);
                }
                interval_map.remove(&remove_interval);
                assert_eq!(expected_intervals, interval_map.into_iter().collect::<Vec<_>>(), "\npermutation:\n\n{}\ncase:\n\n{}\n", permutation_description, case_description);
            }
        }
    }

    #[test]
    fn test_update() {
        let permutations = vec![
            vec![
            ], vec![(
                    format!("{}\n",
                        "  [0----)"
                    ),
                    vec![0]
                )
            ], vec![(
                    format!("{}\n{}\n{}\n",
                        "  [0----)",
                        "       \\",
                        "      [1----)"
                    ),
                    vec![0, 1]
                ), (
                    format!("{}\n{}\n{}\n",
                        "      [1----)",
                        "       /",
                        "  [0----)"
                    ),
                    vec![1, 0]
                )
            ], vec![(
                    format!("{}\n{}\n{}\n{}\n{}\n",
                        "  [0----)",
                        "       \\",
                        "      [1----)",
                        "           \\",
                        "          [2----)"
                    ),
                    vec![0, 1, 2]
                ), (
                    format!("{}\n{}\n{}\n{}\n{}\n",
                        "  [0----)",
                        "       \\",
                        "      [2----)",
                        "       /",
                        "  [1----)"
                    ),
                    vec![0, 2, 1]
                ), (
                    format!("{}\n{}\n{}\n",
                        "      [1----)",
                        "       /   \\",
                        "  [0----) [2----)",
                    ),
                    vec![1, 0, 2]
                ), (
                    format!("{}\n{}\n{}\n{}\n{}\n",
                        "      [2----)",
                        "       /",
                        "  [0----)",
                        "       \\",
                        "      [1----)"
                    ),
                    vec![2, 0, 1]
                ), (
                    format!("{}\n{}\n{}\n{}\n{}\n",
                        "          [2----)",
                        "           /",
                        "      [1----)",
                        "       /",
                        "  [0----)"
                    ),
                    vec![2, 1, 0]
                )
            ]
        ];
        let cases = vec![(
                format!("{}\n{}\n{}\n",
                    "  [3----------------)",
                    "                      -> [3----|3----|3----)",
                    "  [0----|1----|2----)"
                ),
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
            ), (
                format!("{}\n{}\n{}\n",
                    "  [3-------------)---",
                    "                      -> [3----|3----|3-|2-)",
                    "  [0----|1----|2----)"
                ),
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
            ), (
                format!("{}\n{}\n{}\n",
                    "  [3----------)------",
                    "                      -> [3----|3----|2----)",
                    "  [0----|1----|2----)"
                ),
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
            ), (
                format!("{}\n{}\n{}\n",
                    "  [3-------)---------",
                    "                      -> [3----|3-|1-|2----)",
                    "  [0----|1----|2----)"
                ),
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
            ), (
                format!("{}\n{}\n{}\n",
                    "  [3----)------------",
                    "                      -> [3----|1----|2----)",
                    "  [0----|1----|2----)"
                ),
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
            ), (
                format!("{}\n{}\n{}\n",
                    "  [3-)---------------",
                    "                      -> [3-|0-|1----|2----)",
                    "  [0----|1----|2----)"
                ),
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
            ), (
                format!("{}\n{}\n{}\n",
                    "  |------------------",
                    "                      -> [0----|1----|2----)",
                    "  [0----|1----|2----)"
                ),
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
            ), (
                format!("{}\n{}\n{}\n",
                    "  ---[3-------------)",
                    "                      -> [0-|3-|3----|3----)",
                    "  [0----|1----|2----)"
                ),
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
            ), (
                format!("{}\n{}\n{}\n",
                    "  ---[3----------)---",
                    "                      -> [0-|3-|3----|3-|2-)",
                    "  [0----|1----|2----)"
                ),
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
            ), (
                format!("{}\n{}\n{}\n",
                    "  ---[3-------)------",
                    "                      -> [0-|3-|3----|2----)",
                    "  [0----|1----|2----)"
                ),
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
            ), (
                format!("{}\n{}\n{}\n",
                    "  ---[3----)---------",
                    "                      -> [0-|3-|3-|1-|2----)",
                    "  [0----|1----|2----)"
                ),
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
            ), (
                format!("{}\n{}\n{}\n",
                    "  ---[3-)------------",
                    "                      -> [0-|3-|1----|2----)",
                    "  [0----|1----|2----)"
                ),
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
            ), (
                format!("{}\n{}\n{}\n",
                    "  --[3)--------------",
                    "                      -> [0|3|0|1----|2----)",
                    "  [0----|1----|2----)"
                ),
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
            ), (
                format!("{}\n{}\n{}\n",
                    "  ---|---------------",
                    "                      -> [0----|1----|2----)",
                    "  [0----|1----|2----)"
                ),
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
            ), (
                format!("{}\n{}\n{}\n",
                    "  ------[3----------)",
                    "                      -> [0----|3----|3----)",
                    "  [0----|1----|2----)"
                ),
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
            ), (
                format!("{}\n{}\n{}\n",
                    "  ------[3-------)---",
                    "                      -> [0----|3----|3-|2-)",
                    "  [0----|1----|2----)"
                ),
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
            ), (
                format!("{}\n{}\n{}\n",
                    "  ------[3----)------",
                    "                      -> [0----|3----|2----)",
                    "  [0----|1----|2----)"
                ),
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
            ), (
                format!("{}\n{}\n{}\n",
                    "  ------[3-)---------",
                    "                      -> [0----|3-|1-|2----)",
                    "  [0----|1----|2----)"
                ),
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
            ), (
                format!("{}\n{}\n{}\n",
                    "  ------|------------",
                    "                      -> [0----|1----|2----)",
                    "  [0----|1----|2----)"
                ),
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
            ), (
                format!("{}\n{}\n{}\n",
                    "  [3----------------)",
                    "                      -> [3----|3----|3----)",
                    "  ------[1----|2----)"
                ),
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
            ), (
                format!("{}\n{}\n{}\n",
                    "  [3-------------)---",
                    "                      -> [3----|3----|3-|2-)",
                    "  ------[1----|2----)"
                ),
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
            ), (
                format!("{}\n{}\n{}\n",
                    "  [3----------)------",
                    "                      -> [3----|3----|2----)",
                    "  ------[1----|2----)"
                ),
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
            ), (
                format!("{}\n{}\n{}\n",
                    "  [3-------)---------",
                    "                      -> [3----|3-|1-|2----)",
                    "  ------[1----|2----)"
                ),
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
            ), (
                format!("{}\n{}\n{}\n",
                    "  [3----)------------",
                    "                      -> [3----|1----|2----)",
                    "  ------[1----|2----)"
                ),
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
            ), (
                format!("{}\n{}\n{}\n",
                    "  [3-)---------------",
                    "                      -> [3-)--[1----|2----)",
                    "  ------[1----|2----)"
                ),
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
            ), (
                format!("{}\n{}\n{}\n",
                    "  |------------------",
                    "                      -> ------[1----|2----)",
                    "  ------[1----|2----)"
                ),
                (Interval::new(0, 0), 3),
                vec![
                    (Interval::new(6, 12), 1),
                    (Interval::new(12, 18), 2)
                ],
                vec![
                    (Interval::new(6, 12), 1),
                    (Interval::new(12, 18), 2)
                ],
            ), (
                format!("{}\n{}\n{}\n",
                    "  [3----------------)",
                    "                      -> [3----|3----|3----)",
                    "  [0----)-----[2----)"
                ),
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
            ), (
                format!("{}\n{}\n{}\n",
                    "  [3-------------)---",
                    "                      -> [3----|3----|3-|2-)",
                    "  [0----)-----[2----)"
                ),
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
            ), (
                format!("{}\n{}\n{}\n",
                    "  [3----------)------",
                    "                      -> [3----|3----|2----)",
                    "  [0----)-----[2----)"
                ),
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
            ), (
                format!("{}\n{}\n{}\n",
                    "  [3-------)---------",
                    "                      -> [3----|3-)--[2----)",
                    "  [0----)-----[2----)"
                ),
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
            ), (
                format!("{}\n{}\n{}\n",
                    "  ---[3-------------)",
                    "                      -> [0-|3-|3----|3----)",
                    "  [0----)-----[2----)"
                ),
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
            ), (
                format!("{}\n{}\n{}\n",
                    "  ---[3----------)---",
                    "                      -> [0-|3-|3----|3-|2-)",
                    "  [0----)-----[2----)"
                ),
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
            ), (
                format!("{}\n{}\n{}\n",
                    "  ---[3-------)------",
                    "                      -> [0-|3-|3----|2----)",
                    "  [0----)-----[2----)"
                ),
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
            ), (
                format!("{}\n{}\n{}\n",
                    "  ---[3----)---------",
                    "                      -> [0-|3-|3-)--[2----)",
                    "  [0----)-----[2----)"
                ),
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
            ), (
                format!("{}\n{}\n{}\n",
                    "  [3----------------)",
                    "                      -> [3----|3----|3----)",
                    "  [0----|1----)------"
                ),
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
            ), (
                format!("{}\n{}\n{}\n",
                    "  [3-------------)---",
                    "                      -> [3----|3----|3-)---",
                    "  [0----|1----)------"
                ),
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
            ), (
                format!("{}\n{}\n{}\n",
                    "  ---[3-------------)",
                    "                      -> [0-|3-|3----|3----)",
                    "  [0----|1----)------"
                ),
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
            ), (
                format!("{}\n{}\n{}\n",
                    "  [3----------------)",
                    "                      -> [3----|3----|3----)",
                    "  ------[1----)------"
                ),
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
        for (case_description, update_interval, insert_intervals, expected_intervals) in cases {
            for (permutation_description, indices) in &permutations[insert_intervals.len()] {
                let mut interval_map = IntervalMap::new();
                for &index in indices {
                    let (insert_interval, insert_value) = insert_intervals[index];
                    interval_map.insert(insert_interval, insert_value);
                }
                let (update_interval, update_value) = update_interval;
                interval_map.update(&update_interval, |_| Some(update_value));
                assert_eq!(expected_intervals, interval_map.into_iter().collect::<Vec<_>>(), "\npermutation:\n\n{}\ncase:\n\n{}\n", permutation_description, case_description);
            }
        }
    }

    #[test]
    fn test_update_remove() {
        let permutations = vec![
            vec![
            ], vec![(
                    format!("{}\n",
                        "  [0----)"
                    ),
                    vec![0]
                )
            ], vec![(
                    format!("{}\n{}\n{}\n",
                        "  [0----)",
                        "       \\",
                        "      [1----)"
                    ),
                    vec![0, 1]
                ), (
                    format!("{}\n{}\n{}\n",
                        "      [1----)",
                        "       /",
                        "  [0----)"
                    ),
                    vec![1, 0]
                )
            ], vec![(
                    format!("{}\n{}\n{}\n{}\n{}\n",
                        "  [0----)",
                        "       \\",
                        "      [1----)",
                        "           \\",
                        "          [2----)"
                    ),
                    vec![0, 1, 2]
                ), (
                    format!("{}\n{}\n{}\n{}\n{}\n",
                        "  [0----)",
                        "       \\",
                        "      [2----)",
                        "       /",
                        "  [1----)"
                    ),
                    vec![0, 2, 1]
                ), (
                    format!("{}\n{}\n{}\n",
                        "      [1----)",
                        "       /   \\",
                        "  [0----) [2----)",
                    ),
                    vec![1, 0, 2]
                ), (
                    format!("{}\n{}\n{}\n{}\n{}\n",
                        "      [2----)",
                        "       /",
                        "  [0----)",
                        "       \\",
                        "      [1----)"
                    ),
                    vec![2, 0, 1]
                ), (
                    format!("{}\n{}\n{}\n{}\n{}\n",
                        "          [2----)",
                        "           /",
                        "      [1----)",
                        "       /",
                        "  [0----)"
                    ),
                    vec![2, 1, 0]
                )
            ]
        ];
        let cases = vec![(
                format!("{}\n{}\n{}\n",
                    "  [-----------------)",
                    "                      -> -------------------",
                    "  [0----|1----|2----)"
                ),
                Interval::new(0, 18),
                vec![
                    (Interval::new(0, 6), 0),
                    (Interval::new(6, 12), 1),
                    (Interval::new(12, 18), 2)
                ],
                vec![],
            ), (
                format!("{}\n{}\n{}\n",
                    "  [--------------)---",
                    "                      -> ---------------[2-)",
                    "  [0----|1----|2----)"
                ),
                Interval::new(0, 15),
                vec![
                    (Interval::new(0, 6), 0),
                    (Interval::new(6, 12), 1),
                    (Interval::new(12, 18), 2)
                ],
                vec![
                    (Interval::new(15, 18), 2)
                ],
            ), (
                format!("{}\n{}\n{}\n",
                    "  [-----------)------",
                    "                      -> ------------[2----)",
                    "  [0----|1----|2----)"
                ),
                Interval::new(0, 12),
                vec![
                    (Interval::new(0, 6), 0),
                    (Interval::new(6, 12), 1),
                    (Interval::new(12, 18), 2)
                ],
                vec![
                    (Interval::new(12, 18), 2)
                ],
            ), (
                format!("{}\n{}\n{}\n",
                    "  [--------)---------",
                    "                      -> ---------[1-|2----)",
                    "  [0----|1----|2----)"
                ),
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
            ), (
                format!("{}\n{}\n{}\n",
                    "  [-----)------------",
                    "                      -> ------[1----|2----)",
                    "  [0----|1----|2----)"
                ),
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
            ), (
                format!("{}\n{}\n{}\n",
                    "  [--)---------------",
                    "                      -> ---[0-|1----|2----)",
                    "  [0----|1----|2----)"
                ),
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
            ), (
                format!("{}\n{}\n{}\n",
                    "  |------------------",
                    "                      -> [0----|1----|2----)",
                    "  [0----|1----|2----)"
                ),
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
            ), (
                format!("{}\n{}\n{}\n",
                    "  ---[--------------)",
                    "                      -> [0-)---------------",
                    "  [0----|1----|2----)"
                ),
                Interval::new(3, 18),
                vec![
                    (Interval::new(0, 6), 0),
                    (Interval::new(6, 12), 1),
                    (Interval::new(12, 18), 2)
                ],
                vec![
                    (Interval::new(0, 3), 0)
                ],
            ), (
                format!("{}\n{}\n{}\n",
                    "  ---[-----------)---",
                    "                      -> [0-)-----------[2-)",
                    "  [0----|1----|2----)"
                ),
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
            ), (
                format!("{}\n{}\n{}\n",
                    "  ---[--------)------",
                    "                      -> [0-)--------[2----)",
                    "  [0----|1----|2----)"
                ),
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
            ), (
                format!("{}\n{}\n{}\n",
                    "  ---[-----)---------",
                    "                      -> [0-)-----[1-|2----)",
                    "  [0----|1----|2----)"
                ),
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
            ), (
                format!("{}\n{}\n{}\n",
                    "  ---[--)------------",
                    "                      -> [0-)--[1----|2----)",
                    "  [0----|1----|2----)"
                ),
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
            ), (
                format!("{}\n{}\n{}\n",
                    "  --[-)--------------",
                    "                      -> [0)-[0|1----|2----)",
                    "  [0----|1----|2----)"
                ),
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
            ), (
                format!("{}\n{}\n{}\n",
                    "  ---|---------------",
                    "                      -> [0----|1----|2----)",
                    "  [0----|1----|2----)"
                ),
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
            ), (
                format!("{}\n{}\n{}\n",
                    "  ------[-----------)",
                    "                      -> [0----)------------",
                    "  [0----|1----|2----)"
                ),
                Interval::new(6, 18),
                vec![
                    (Interval::new(0, 6), 0),
                    (Interval::new(6, 12), 1),
                    (Interval::new(12, 18), 2)
                ],
                vec![
                    (Interval::new(0, 6), 0)
                ],
            ), (
                format!("{}\n{}\n{}\n",
                    "  ------[--------)---",
                    "                      -> [0----)--------[2-)",
                    "  [0----|1----|2----)"
                ),
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
            ), (
                format!("{}\n{}\n{}\n",
                    "  ------[-----)------",
                    "                      -> [0----)-----[2----)",
                    "  [0----|1----|2----)"
                ),
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
            ), (
                format!("{}\n{}\n{}\n",
                    "  ------[--)---------",
                    "                      -> [0----)--[1-|2----)",
                    "  [0----|1----|2----)"
                ),
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
            ), (
                format!("{}\n{}\n{}\n",
                    "  ------|------------",
                    "                      -> [0----|1----|2----)",
                    "  [0----|1----|2----)"
                ),
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
            ), (
                format!("{}\n{}\n{}\n",
                    "  [-----------------)",
                    "                      -> -------------------",
                    "  ------[1----|2----)"
                ),
                Interval::new(0, 18),
                vec![
                    (Interval::new(6, 12), 1),
                    (Interval::new(12, 18), 2)
                ],
                vec![],
            ), (
                format!("{}\n{}\n{}\n",
                    "  [--------------)---",
                    "                      -> ---------------[2-)",
                    "  ------[1----|2----)"
                ),
                Interval::new(0, 15),
                vec![
                    (Interval::new(6, 12), 1),
                    (Interval::new(12, 18), 2)
                ],
                vec![
                    (Interval::new(15, 18), 2)
                ],
            ), (
                format!("{}\n{}\n{}\n",
                    "  [-----------)------",
                    "                      -> ------------[2----)",
                    "  ------[1----|2----)"
                ),
                Interval::new(0, 12),
                vec![
                    (Interval::new(6, 12), 1),
                    (Interval::new(12, 18), 2)
                ],
                vec![
                    (Interval::new(12, 18), 2)
                ],
            ), (
                format!("{}\n{}\n{}\n",
                    "  [--------)---------",
                    "                      -> ---------[1-|2----)",
                    "  ------[1----|2----)"
                ),
                Interval::new(0, 9),
                vec![
                    (Interval::new(6, 12), 1),
                    (Interval::new(12, 18), 2)
                ],
                vec![
                    (Interval::new(9, 12), 1),
                    (Interval::new(12, 18), 2)
                ],
            ), (
                format!("{}\n{}\n{}\n",
                    "  [-----)------------",
                    "                      -> ------[1----|2----)",
                    "  ------[1----|2----)"
                ),
                Interval::new(0, 6),
                vec![
                    (Interval::new(6, 12), 1),
                    (Interval::new(12, 18), 2)
                ],
                vec![
                    (Interval::new(6, 12), 1),
                    (Interval::new(12, 18), 2)
                ],
            ), (
                format!("{}\n{}\n{}\n",
                    "  [--)---------------",
                    "                      -> ------[1----|2----)",
                    "  ------[1----|2----)"
                ),
                Interval::new(0, 3),
                vec![
                    (Interval::new(6, 12), 1),
                    (Interval::new(12, 18), 2)
                ],
                vec![
                    (Interval::new(6, 12), 1),
                    (Interval::new(12, 18), 2)
                ],
            ), (
                format!("{}\n{}\n{}\n",
                    "  |------------------",
                    "                      -> ------[1----|2----)",
                    "  ------[1----|2----)"
                ),
                Interval::new(0, 0),
                vec![
                    (Interval::new(6, 12), 1),
                    (Interval::new(12, 18), 2)
                ],
                vec![
                    (Interval::new(6, 12), 1),
                    (Interval::new(12, 18), 2)
                ],
            ), (
                format!("{}\n{}\n{}\n",
                    "  [-----------------)",
                    "                      -> -------------------",
                    "  [0----)-----[2----)"
                ),
                Interval::new(0, 18),
                vec![
                    (Interval::new(0, 6), 0),
                    (Interval::new(12, 18), 2)
                ],
                vec![],
            ), (
                format!("{}\n{}\n{}\n",
                    "  [--------------)---",
                    "                      -> ---------------[2-)",
                    "  [0----)-----[2----)"
                ),
                Interval::new(0, 15),
                vec![
                    (Interval::new(0, 6), 0),
                    (Interval::new(12, 18), 2)
                ],
                vec![
                    (Interval::new(15, 18), 2)
                ],
            ), (
                format!("{}\n{}\n{}\n",
                    "  [-----------)------",
                    "                      -> ------------[2----)",
                    "  [0----)-----[2----)"
                ),
                Interval::new(0, 12),
                vec![
                    (Interval::new(0, 6), 0),
                    (Interval::new(12, 18), 2)
                ],
                vec![
                    (Interval::new(12, 18), 2)
                ],
            ), (
                format!("{}\n{}\n{}\n",
                    "  [--------)---------",
                    "                      -> ------------[2----)",
                    "  [0----)-----[2----)"
                ),
                Interval::new(0, 9),
                vec![
                    (Interval::new(0, 6), 0),
                    (Interval::new(12, 18), 2)
                ],
                vec![
                    (Interval::new(12, 18), 2)
                ],
            ), (
                format!("{}\n{}\n{}\n",
                    "  ---[--------------)",
                    "                      -> [0-)---------------",
                    "  [0----)-----[2----)"
                ),
                Interval::new(3, 18),
                vec![
                    (Interval::new(0, 6), 0),
                    (Interval::new(12, 18), 2)
                ],
                vec![
                    (Interval::new(0, 3), 0)
                ],
            ), (
                format!("{}\n{}\n{}\n",
                    "  ---[-----------)---",
                    "                      -> [0-)-----------[2-)",
                    "  [0----)-----[2----)"
                ),
                Interval::new(3, 15),
                vec![
                    (Interval::new(0, 6), 0),
                    (Interval::new(12, 18), 2)
                ],
                vec![
                    (Interval::new(0, 3), 0),
                    (Interval::new(15, 18), 2)
                ],
            ), (
                format!("{}\n{}\n{}\n",
                    "  ---[--------)------",
                    "                      -> [0-)--------[2----)",
                    "  [0----)-----[2----)"
                ),
                Interval::new(3, 12),
                vec![
                    (Interval::new(0, 6), 0),
                    (Interval::new(12, 18), 2)
                ],
                vec![
                    (Interval::new(0, 3), 0),
                    (Interval::new(12, 18), 2)
                ],
            ), (
                format!("{}\n{}\n{}\n",
                    "  ---[-----)---------",
                    "                      -> [0-)--------[2----)",
                    "  [0----)-----[2----)"
                ),
                Interval::new(3, 9),
                vec![
                    (Interval::new(0, 6), 0),
                    (Interval::new(12, 18), 2)
                ],
                vec![
                    (Interval::new(0, 3), 0),
                    (Interval::new(12, 18), 2)
                ],
            ), (
                format!("{}\n{}\n{}\n",
                    "  [-----------------)",
                    "                      -> -------------------",
                    "  [0----|1----)------"
                ),
                Interval::new(0, 18),
                vec![
                    (Interval::new(0, 6), 0),
                    (Interval::new(6, 12), 1)
                ],
                vec![],
            ), (
                format!("{}\n{}\n{}\n",
                    "  [--------------)---",
                    "                      -> -------------------",
                    "  [0----|1----)------"
                ),
                Interval::new(0, 15),
                vec![
                    (Interval::new(0, 6), 0),
                    (Interval::new(6, 12), 1)
                ],
                vec![],
            ), (
                format!("{}\n{}\n{}\n",
                    "  ---[--------------)",
                    "                      -> [0-)---------------",
                    "  [0----|1----)------"
                ),
                Interval::new(3, 18),
                vec![
                    (Interval::new(0, 6), 0),
                    (Interval::new(6, 12), 1)
                ],
                vec![
                    (Interval::new(0, 3), 0)
                ],
            ), (
                format!("{}\n{}\n{}\n",
                    "  [-----------------)",
                    "                      -> -------------------",
                    "  ------[1----)------"
                ),
                Interval::new(0, 18),
                vec![
                    (Interval::new(6, 12), 1)
                ],
                vec![],
            ),
        ];
        for (case_description, update_interval, insert_intervals, expected_intervals) in cases {
            for (permutation_description, indices) in &permutations[insert_intervals.len()] {
                let mut interval_map = IntervalMap::new();
                for &index in indices {
                    let (insert_interval, insert_value) = insert_intervals[index];
                    interval_map.insert(insert_interval, insert_value);
                }
                interval_map.update(&update_interval, |_| None);
                assert_eq!(expected_intervals, interval_map.into_iter().collect::<Vec<_>>(), "\npermutation:\n\n{}\ncase:\n\n{}\n", permutation_description, case_description);
            }
        }
    }
}
