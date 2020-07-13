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
        // if left exists, recurse
        if let Some(left) = self.left.as_ref() {
            left.min_node()
        // otherwise, self is minimum
        } else { self }
    }

    pub(crate) fn remove_min_node(mut self) -> (Option<IntervalMapNode<K, V>>, IntervalMapNode<K, V>) {
        // if left exists, recurse
        if let Some(left) = self.left.take() {
            let (left, min_node) = left.remove_min_node();
            self.left = Box::new(left);
            (Some(self), min_node)
        // otherwise, self is minimum
        } else { (None, self) }
    }

    pub(crate) fn max_key(&self) -> &K {
        self.max_node().interval.upper()
    }

    pub(crate) fn max_node(&self) -> &IntervalMapNode<K, V> {
        // if right exists, recurse
        if let Some(right) = self.right.as_ref() {
            right.max_node()
        // otherwise, self is maximum
        } else { self }
    }

    pub(crate) fn span(&self) -> Interval<&K> {
        Interval::new(self.min_key(), self.max_key())
    }

    pub(crate) fn get(&self, key: &K) -> Option<&V> {
        self.get_entry(key).map(|(_, v)| v)
    }

    pub(crate) fn get_entry(&self, key: &K) -> Option<(&Interval<K>, &V)> {
        // if self interval contains key
        if self.interval.contains(key) {
            Some((&self.interval, &self.value))
        // if key is less than self interval
        } else if key < self.interval.lower() {
            // if left exists, recurse
            if let Some(left) = self.left.as_ref() {
                left.get_entry(key)
            // otherwise, key doesn't exist
            } else { None }
        // otherwise, key is greater than self interval
        } else {
            // if right exists, recurse
            if let Some(right) = self.right.as_ref() {
                right.get_entry(key)
            // otherwise, key doesn't exist
            } else { None }
        }
    }

    pub(crate) fn insert(&mut self, interval: Interval<K>, value: V) {
        // if the intervals perfectly overlap (this prevents inserting duplicate empty intervals)
        if (interval.lower() == self.interval.lower()) && (interval.upper() == self.interval.upper()) {
            panic!("intervals must not overlap");
        // if interval is less than self interval
        } else if interval.upper() <= self.interval.lower() {
            // if left exists, recurse
            if let Some(left) = self.left.as_mut() {
                left.insert(interval, value);
            // otherwise, set new left
            } else {
                self.left = Box::new(Some(IntervalMapNode::new(interval, value, None, None)));
            }
        // if interval is greater than self interval
        } else if interval.lower() >= self.interval.upper() {
            // if right exists, recurse
            if let Some(right) = self.right.as_mut() {
                right.insert(interval, value);
            // otherwise, set new right
            } else {
                self.right = Box::new(Some(IntervalMapNode::new(interval, value, None, None)));
            }
        // otherwise, intervals overlap in some (non-perfect) way
        } else {
            panic!("intervals must not overlap");
        }
    }
}

impl<K: Clone + PartialOrd, V: Clone> IntervalMapNode<K, V> {
    pub(crate) fn remove(mut self, interval: &Interval<K>) -> Option<IntervalMapNode<K, V>> {
        // empty intervals can be removed
        if interval.is_empty() {
            // if empty interval is enclosed by self interval, (potentially) split the interval
            if self.interval.encloses(&interval) {
                // if empty interval exactly equals self interval
                if (interval.lower() == self.interval.lower()) && (interval.upper() == self.interval.upper()) {
                    // remove self
                    match (*self.left, *self.right) {
                        // two children, replace with right minimum
                        (Some(left), Some(right)) => {
                            let (right, mut result) = right.remove_min_node();
                            result.right = Box::new(right);
                            result.left = Box::new(Some(left));
                            Some(result)
                        },
                        // one left child, move up
                        (Some(left), None) => Some(left),
                        // one right child, move up
                        (None, Some(right)) => Some(right),
                        // no children, remove
                        (None, None) => None,
                    }
                // if empty interval is touching left side of nonempty self interval, do not remove self
                } else if interval.lower() == self.interval.lower() {
                    // if left exists, recurse
                    self.left = Box::new(if let Some(left) = self.left.take() {
                        left.remove(interval)
                    // otherwise, nothing to remove
                    } else { None });
                    Some(self)
                // if empty interval is touching right side of nonempty self interval, do not remove self
                } else if interval.upper() == self.interval.upper() {
                    // if right exists, recurse
                    self.right = Box::new(if let Some(right) = self.right.take() {
                        right.remove(interval)
                    // otherwise, nothing to remove
                    } else { None });
                    Some(self)
                // otherwise, empty interval is within self interval
                } else {
                    // remove self, will reinsert each side of split
                    let mut result = match (*self.left, *self.right) {
                        // two children, replace with right minimum
                        (Some(left), Some(right)) => {
                            let (right, mut result) = right.remove_min_node();
                            result.right = Box::new(right);
                            result.left = Box::new(Some(left));
                            Some(result)
                        },
                        // one left child, move up
                        (Some(left), None) => Some(left),
                        // one right child, move up
                        (None, Some(right)) => Some(right),
                        // no children, remove
                        (None, None) => None,
                    };
                    // reinsert left part of interval
                    let left_interval = Interval::new(self.interval.lower().clone(), interval.lower().clone());
                    // if result exists, do plain insert
                    if let Some(result) = result.as_mut() {
                        result.insert(left_interval, self.value.clone());
                    // otherwise, this is the new result
                    } else {
                        result = Some(IntervalMapNode::new(left_interval, self.value.clone(), None, None));
                    }
                    // reinsert right part of interval
                    let right_interval = Interval::new(interval.upper().clone(), self.interval.upper().clone());
                    // if result exists, do plain insert
                    if let Some(result) = result.as_mut() {
                        result.insert(right_interval, self.value.clone());
                    // otherwise, this is the new result
                    } else {
                        result = Some(IntervalMapNode::new(right_interval, self.value.clone(), None, None));
                    }
                    result
                }
            // if empty interval is less than self interval, recurse
            } else if interval.upper() < self.interval.lower() {
                // if left exists, recurse
                if let Some(left) = self.left.take() {
                    self.left = Box::new(left.remove(interval));
                } // otherwise, nothing to remove
                Some(self)
            // otherwise, empty interval is greater than self interval, recurse
            } else {
                // if right exists, recurse
                if let Some(right) = self.right.take() {
                    self.right = Box::new(right.remove(interval));
                } // otherwise, nothing to remove
                Some(self)
            }
        // if the intervals overlap
        } else if let Some(intersection) = interval.intersection(&self.interval) {
            // if the overlap is empty, handle specially to prevent infinite recursion
            if intersection.is_empty() {
                // if interval is touching the right
                if interval.lower() == self.interval.upper() {
                    // if right exists, recurse
                    if let Some(right) = self.right.take() {
                        self.right = Box::new(right.remove(interval));
                    } // otherwise, nothing to remove
                // otherwise, interval is touching the left
                } else {
                    // if left exists, recurse
                    if let Some(left) = self.left.take() {
                        self.left = Box::new(left.remove(interval));
                    } // otherwise, nothing to remove
                }
                Some(self)
            // otherwise, the overlap must be removed
            } else {
                // remove self, will reinsert as needed
                let mut result = match (*self.left, *self.right) {
                    // two children, replace with right minimum
                    (Some(left), Some(right)) => {
                        let (right, mut result) = right.remove_min_node();
                        result.right = Box::new(right);
                        result.left = Box::new(Some(left));
                        Some(result)
                    },
                    // one left child, move up
                    (Some(left), None) => Some(left),
                    // one right child, move up
                    (None, Some(right)) => Some(right),
                    // no children, simply remove
                    (None, None) => None,
                };
                // if left part of interval still needs to be removed
                if interval.lower() < intersection.lower() {
                    // if result exists, do plain remove
                    result = if let Some(result) = result {
                        result.remove(&Interval::new(interval.lower().clone(), intersection.lower().clone()))
                    // otherwise, nothing to remove
                    } else { None };
                // if left part of self still exists, reinsert
                } else if self.interval.lower() < intersection.lower() {
                    let interval = Interval::new(self.interval.lower().clone(), intersection.lower().clone());
                    // if result exists, do plain insert
                    if let Some(result) = result.as_mut() {
                        result.insert(interval, self.value.clone());
                    // otherwise, this is the new result
                    } else {
                        result = Some(IntervalMapNode::new(interval, self.value.clone(), None, None));
                    }
                }
                // if right part of interval still needs to be removed
                if interval.upper() > intersection.upper() {
                    // if result exists, do plain remove
                    result = if let Some(result) = result {
                        result.remove(&Interval::new(intersection.upper().clone(), interval.upper().clone()))
                    // otherwise, nothing to remove
                    } else { None };
                // if right part of self still exists, reinsert
                } else if self.interval.upper() > intersection.upper() {
                    let interval = Interval::new(intersection.upper().clone(), self.interval.upper().clone());
                    // if result exists, do plain insert
                    if let Some(result) = result.as_mut() {
                        result.insert(interval, self.value);
                    // otherwise, this is the new result
                    } else {
                        result = Some(IntervalMapNode::new(interval, self.value, None, None));
                    }
                }
                result
            }
        // otherwise, intervals do not overlap
        } else {
            // if interval is greater than self interval
            if interval.lower() > self.interval.upper() {
                // if right exists, recurse
                if let Some(right) = self.right.take() {
                    self.right = Box::new(right.remove(interval));
                } // otherwise, there is nothing to remove
            // otherwise interval is less than self interval
            } else {
                // if left exists, recurse
                if let Some(left) = self.left.take() {
                    self.left = Box::new(left.remove(interval));
                } // otherwise, there is nothing to remove
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
        // empty intervals can be updated
        if interval.is_empty() {
            // if empty interval is enclosed by self interval, (potentially) split the interval
            if self.interval.encloses(&interval) {
                // if empty interval exactly equals self interval
                if (interval.lower() == self.interval.lower()) && (interval.upper() == self.interval.upper()) {
                    // remove self, will reinsert as needed
                    let mut result = match (*self.left, *self.right) {
                        // two children, replace with right minimum
                        (Some(left), Some(right)) => {
                            let (right, mut result) = right.remove_min_node();
                            result.right = Box::new(right);
                            result.left = Box::new(Some(left));
                            Some(result)
                        },
                        // one left child, move up
                        (Some(left), None) => Some(left),
                        // one right child, move up
                        (None, Some(right)) => Some(right),
                        // no children, remove
                        (None, None) => None,
                    };
                    // if update produces a value, reinsert
                    if let Some(value) = value(&interval, Some(self.value.clone())) {
                        // if result exists, do plain insert
                        if let Some(result) = result.as_mut() {
                            result.insert(interval.clone(), value);
                        // otherwise, this is the new result
                        } else {
                            result = Some(IntervalMapNode::new(interval.clone(), value, None, None));
                        }
                    };
                    result
                // if empty interval is touching left side of nonempty self interval, do not remove self
                } else if interval.lower() == self.interval.lower() {
                    // if left exists, recurse
                    if let Some(left) = self.left.take() {
                        self.left = Box::new(left.update_entry(interval, value));
                    // otherwise, if update produces a value, this is the new result
                    } else if let Some(value) = value(interval, None) {
                        self.left = Box::new(Some(IntervalMapNode::new(interval.clone(), value, None, None)));
                    }
                    Some(self)
                // if empty interval is touching right side of nonempty self interval, do not remove self
                } else if interval.upper() == self.interval.upper() {
                    // if right exists, recurse
                    if let Some(right) = self.right.take() {
                        self.right = Box::new(right.update_entry(interval, value));
                    // otherwise, if update produces a value, this is the new result
                    } else if let Some(value) = value(interval, None) {
                        self.right = Box::new(Some(IntervalMapNode::new(interval.clone(), value, None, None)));
                    }
                    Some(self)
                // otherwise, empty interval is within self interval
                } else {
                    // remove self, will reinsert each side of split
                    let mut result = match (*self.left, *self.right) {
                        // two children, replace with right minimum
                        (Some(left), Some(right)) => {
                            let (right, mut result) = right.remove_min_node();
                            result.right = Box::new(right);
                            result.left = Box::new(Some(left));
                            Some(result)
                        },
                        // one left child, move up
                        (Some(left), None) => Some(left),
                        // one right child, move up
                        (None, Some(right)) => Some(right),
                        // no children, remove
                        (None, None) => None,
                    };
                    // if update produces a value, reinsert
                    if let Some(value) = value(&interval, Some(self.value.clone())) {
                        // if result exists, do plain insert
                        if let Some(result) = result.as_mut() {
                            result.insert(interval.clone(), value);
                        // otherwise, this is the new result
                        } else {
                            result = Some(IntervalMapNode::new(interval.clone(), value, None, None));
                        }
                    };
                    // reinsert left part of interval
                    let left_interval = Interval::new(self.interval.lower().clone(), interval.lower().clone());
                    // if result exists, do plain insert
                    if let Some(result) = result.as_mut() {
                        result.insert(left_interval, self.value.clone());
                    // otherwise, this is the new result
                    } else {
                        result = Some(IntervalMapNode::new(left_interval, self.value.clone(), None, None));
                    }
                    // reinsert right part of interval
                    let right_interval = Interval::new(interval.upper().clone(), self.interval.upper().clone());
                    // if result exists, do plain insert
                    if let Some(result) = result.as_mut() {
                        result.insert(right_interval, self.value.clone());
                    // otherwise, this is the new result
                    } else {
                        result = Some(IntervalMapNode::new(right_interval, self.value.clone(), None, None));
                    }
                    result
                }
            // if empty interval is less than self interval, recurse
            } else if interval.upper() < self.interval.lower() {
                // if left exists, recurse
                if let Some(left) = self.left.take() {
                    self.left = Box::new(left.update_entry(interval, value));
                // otherwise, if update produces a value, this is the new result
                } else if let Some(value) = value(interval, None) {
                    self.left = Box::new(Some(IntervalMapNode::new(interval.clone(), value, None, None)));
                }
                Some(self)
            // otherwise, empty interval is greater than self interval, recurse
            } else {
                // if right exists, recurse
                if let Some(right) = self.right.take() {
                    self.right = Box::new(right.update_entry(interval, value));
                // otherwise, if update produces a value, this is the new result
                } else if let Some(value) = value(interval, None) {
                    self.right = Box::new(Some(IntervalMapNode::new(interval.clone(), value, None, None)));
                }
                Some(self)
            }
        // if the intervals overlap
        } else if let Some(intersection) = interval.intersection(&self.interval) {
            // if the overlap is empty, handle specially to prevent infinite recursion
            if intersection.is_empty() {
                // if interval is touching the right
                if interval.lower() == self.interval.upper() {
                    // if right exists, recurse
                    if let Some(right) = self.right.take() {
                        self.right = Box::new(right.update_entry(interval, value));
                    // otherwise, if update produces a value, this is the new right
                    } else if let Some(value) = value(interval, None) {
                        self.right = Box::new(Some(IntervalMapNode::new(interval.clone(), value, None, None)));
                    }
                // otherwise, interval is touching the left
                } else {
                    // if left exists, recurse
                    if let Some(left) = self.left.take() {
                        self.left = Box::new(left.update_entry(interval, value));
                    // otherwise, if update produces a value, this is the new left
                    } else if let Some(value) = value(interval, None) {
                        self.left = Box::new(Some(IntervalMapNode::new(interval.clone(), value, None, None)));
                    }
                }
                Some(self)
            // otherwise, the overlap must be updated
            } else {
                // remove self, will reinsert as needed
                let mut result = match (*self.left, *self.right) {
                    // two children, replace with right minimum
                    (Some(left), Some(right)) => {
                        let (right, mut result) = right.remove_min_node();
                        result.right = Box::new(right);
                        result.left = Box::new(Some(left));
                        Some(result)
                    },
                    // one left child, move up
                    (Some(left), None) => Some(left),
                    // one right child, move up
                    (None, Some(right)) => Some(right),
                    // no children, simply remove
                    (None, None) => None,
                };
                // if update produces a value, reinsert intersection
                if let Some(value) = value(&intersection, Some(self.value.clone())) {
                    // if result exists, do plain insert
                    if let Some(result) = result.as_mut() {
                        result.insert(intersection.clone(), value);
                    // otherwise, this is the new result
                    } else {
                        result = Some(IntervalMapNode::new(intersection.clone(), value, None, None));
                    }
                }
                // if left part of interval still needs to be updated
                if interval.lower() < intersection.lower() {
                    let interval = Interval::new(interval.lower().clone(), intersection.lower().clone());
                    // if result exists, do plain update
                    result = if let Some(result) = result {
                        result.update_entry(&interval, value.clone())
                    // otherwise, if update produces a value, this is the new result
                    } else if let Some(value) = value(&interval, None) {
                        Some(IntervalMapNode::new(interval, value, None, None))
                    // otherwise, no result
                    } else { None }
                // if left part of self still exists, reinsert
                } else if self.interval.lower() < intersection.lower() {
                    let interval = Interval::new(self.interval.lower().clone(), intersection.lower().clone());
                    // if result exists, do plain insert
                    if let Some(result) = result.as_mut() {
                        result.insert(interval, self.value.clone());
                    // otherwise, this is the new result
                    } else {
                        result = Some(IntervalMapNode::new(interval, self.value.clone(), None, None));
                    }
                }
                // if right part of interval still needs to be updated
                if interval.upper() > intersection.upper() {
                    let interval = Interval::new(intersection.upper().clone(), interval.upper().clone());
                    // if result exists, do plain update
                    result = if let Some(result) = result {
                        result.update_entry(&interval, value)
                    // otherwise, if update produces value, this is the new result
                    } else if let Some(value) = value(&interval, None) {
                        Some(IntervalMapNode::new(interval, value, None, None))
                    // otherwise, no result
                    } else { None }
                // if right part of self still exists, reinsert
                } else if self.interval.upper() > intersection.upper() {
                    let interval = Interval::new(intersection.upper().clone(), self.interval.upper().clone());
                    // if result exists, do plain insert
                    if let Some(result) = result.as_mut() {
                        result.insert(interval, self.value);
                    // otherwise, this is the new result
                    } else {
                        result = Some(IntervalMapNode::new(interval, self.value, None, None));
                    }
                }
                result
            }
        // otherwise, intervals do not overlap
        } else {
            // if interval is greater than self interval
            if interval.lower() > self.interval.upper() {
                // if right exists, recurse
                if let Some(right) = self.right.take() {
                    self.right = Box::new(right.update_entry(interval, value));
                // otherwise, if update produces value, this is the new right
                } else if let Some(value) = value(interval, None) {
                    self.right = Box::new(Some(IntervalMapNode::new(interval.clone(), value, None, None)));
                }
            // otherwise, interval is less than self interval
            } else {
                // if left exists, recurse
                if let Some(left) = self.left.take() {
                    self.left = Box::new(left.update_entry(interval, value));
                // otherwise, if update produces value, this is the new right
                } else if let Some(value) = value(interval, None) {
                    self.left = Box::new(Some(IntervalMapNode::new(interval.clone(), value, None, None)));
                }
            }
            Some(self)
        }
    }
}
