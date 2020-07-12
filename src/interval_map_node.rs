use crate::Interval;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) struct IntervalMapNode<K, V> {
    pub(crate) interval: Interval<K>,
    pub(crate) value: V,
    pub(crate) left: Box<Option<IntervalMapNode<K, V>>>,
    pub(crate) right: Box<Option<IntervalMapNode<K, V>>>
}

impl<K: PartialOrd, V> IntervalMapNode<K, V> {
    pub(crate) fn new(interval: Interval<K>, value: V, left: Option<IntervalMapNode<K, V>>, right: Option<IntervalMapNode<K, V>>) -> IntervalMapNode<K, V> {
        IntervalMapNode {
            interval,
            value,
            left: Box::new(left),
            right: Box::new(right),
        }
    }

    pub(crate) fn min_key(&self) -> &K {
        self.min_node().interval.lower()
    }

    pub(crate) fn min_node(&self) -> &IntervalMapNode<K, V> {
        if let Some(left) = self.left.as_ref() {
            left.min_node()
        } else { self }
    }

    pub(crate) fn remove_min_node(mut self) -> (Option<IntervalMapNode<K, V>>, IntervalMapNode<K, V>) {
        if let Some(left) = self.left.take() {
            let (left, min_node) = left.remove_max_node();
            self.left = Box::new(left);
            (Some(self), min_node)
        } else { (None, self) }
    }

    pub(crate) fn max_key(&self) -> &K {
        self.max_node().interval.upper()
    }

    pub(crate) fn max_node(&self) -> &IntervalMapNode<K, V> {
        if let Some(right) = self.right.as_ref() {
            right.max_node()
        } else { self }
    }

    pub(crate) fn remove_max_node(mut self) -> (Option<IntervalMapNode<K, V>>, IntervalMapNode<K, V>) {
        if let Some(right) = self.right.take() {
            let (right, max_node) = right.remove_max_node();
            self.right = Box::new(right);
            (Some(self), max_node)
        } else { (None, self) }
    }

    pub(crate) fn span(&self) -> Interval<&K> {
        Interval::new(self.min_key(), self.max_key())
    }

    pub(crate) fn get(&self, key: &K) -> Option<&V> {
        self.get_entry(key).map(|(_, v)| v)
    }

    pub(crate) fn get_entry(&self, key: &K) -> Option<(&Interval<K>, &V)> {
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

    pub(crate) fn insert(&mut self, interval: Interval<K>, value: V) {
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
}

impl<K: Clone + PartialOrd, V: Clone> IntervalMapNode<K, V> {
    pub(crate) fn remove(mut self, interval: &Interval<K>) -> Option<IntervalMapNode<K, V>> {
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

    pub(crate) fn update<F>(self, interval: &Interval<K>, value: F) -> Option<IntervalMapNode<K, V>>
    where
        F: Fn(Option<V>) -> Option<V> + Clone
    {
        self.update_entry(interval, |_, v| value(v))
    }

    pub(crate) fn update_entry<F>(mut self, interval: &Interval<K>, value: F) -> Option<IntervalMapNode<K, V>>
    where
        F: Fn(&Interval<K>, Option<V>) -> Option<V> + Clone
    {
        if interval.is_empty() {
            Some(self)
        } else if let Some(intersection) = interval.intersection(&self.interval) {
            if intersection.is_empty() {
                if interval.lower() >= self.interval.upper() {
                    if let Some(right) = self.right.take() {
                        self.right = Box::new(right.update_entry(interval, value));
                    } else if let Some(value) = value(interval, None) {
                        self.right = Box::new(Some(IntervalMapNode::new(interval.clone(), value, None, None)));
                    }
                } else {
                    if let Some(left) = self.left.take() {
                        self.left = Box::new(left.update_entry(interval, value));
                    } else if let Some(value) = value(interval, None) {
                        self.left = Box::new(Some(IntervalMapNode::new(interval.clone(), value, None, None)));
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
                    (Some(left), None) => { Some(left) },
                    (None, Some(right)) => { Some(right) },
                    (None, None) => { None },
                };
                if let Some(value) = value(&intersection, Some(self.value.clone())) {
                    if let Some(result) = result.as_mut() {
                        result.insert(intersection.clone(), value);
                    } else {
                        result = Some(IntervalMapNode::new(intersection.clone(), value, None, None));
                    }
                }
                if interval.lower() < intersection.lower() {
                    let interval = Interval::new(interval.lower().clone(), intersection.lower().clone());
                    result = if let Some(result) = result {
                        result.update_entry(&interval, value.clone())
                    } else if let Some(value) = value(&interval, None) {
                        Some(IntervalMapNode::new(interval, value, None, None))
                    } else { None }
                } else if self.interval.lower() < intersection.lower() {
                    let interval = Interval::new(self.interval.lower().clone(), intersection.lower().clone());
                    if let Some(result) = result.as_mut() {
                        result.insert(interval, self.value.clone());
                    } else {
                        result = Some(IntervalMapNode::new(interval, self.value.clone(), None, None));
                    }
                }
                if interval.upper() > intersection.upper() {
                    let interval = Interval::new(intersection.upper().clone(), interval.upper().clone());
                    result = if let Some(result) = result {
                        result.update_entry(&interval, value)
                    } else if let Some(value) = value(&interval, None) {
                        Some(IntervalMapNode::new(interval, value, None, None))
                    } else { None }
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
                    self.right = Box::new(right.update_entry(interval, value));
                } else if let Some(value) = value(interval, None) {
                    self.right = Box::new(Some(IntervalMapNode::new(interval.clone(), value, None, None)));
                }
            } else {
                if let Some(left) = self.left.take() {
                    self.left = Box::new(left.update_entry(interval, value));
                } else if let Some(value) = value(interval, None) {
                    self.left = Box::new(Some(IntervalMapNode::new(interval.clone(), value, None, None)));
                }
            }
            Some(self)
        }
    }
}
