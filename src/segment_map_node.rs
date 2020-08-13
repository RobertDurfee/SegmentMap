use crate::Segment;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SegmentMapNode<K, V> {
    pub segment: Segment<K>,
    pub value: V,
    pub left: Box<Option<SegmentMapNode<K, V>>>,
    pub right: Box<Option<SegmentMapNode<K, V>>>
}

impl<K, V> SegmentMapNode<K, V> 
where
    K: PartialOrd
{
    pub fn new(segment: Segment<K>, value: V, left: Option<SegmentMapNode<K, V>>, right: Option<SegmentMapNode<K, V>>) -> SegmentMapNode<K, V> {
        SegmentMapNode {
            segment,
            value,
            left: Box::new(left),
            right: Box::new(right),
        }
    }

    pub fn min_key(&self) -> &K {
        self.min_node().segment.lower()
    }

    pub fn min_node(&self) -> &SegmentMapNode<K, V> {
        // if left exists, recurse
        if let Some(left) = self.left.as_ref() {
            left.min_node()
        // otherwise, self is minimum
        } else { self }
    }

    pub fn remove_min_node(mut self) -> (Option<SegmentMapNode<K, V>>, SegmentMapNode<K, V>) {
        // if left exists, recurse
        if let Some(left) = self.left.take() {
            let (left, min_node) = left.remove_min_node();
            self.left = Box::new(left);
            (Some(self), min_node)
        // otherwise, self is minimum
        } else { (None, self) }
    }

    pub fn max_key(&self) -> &K {
        self.max_node().segment.upper()
    }

    pub fn max_node(&self) -> &SegmentMapNode<K, V> {
        // if right exists, recurse
        if let Some(right) = self.right.as_ref() {
            right.max_node()
        // otherwise, self is maximum
        } else { self }
    }

    pub fn span(&self) -> Segment<&K> {
        Segment::new(self.min_key(), self.max_key())
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        self.get_entry(key).map(|(_, v)| v)
    }

    pub fn get_entry(&self, key: &K) -> Option<(&Segment<K>, &V)> {
        // if self segment contains key
        if self.segment.contains(key) {
            Some((&self.segment, &self.value))
        // if key is less than self segment
        } else if key < self.segment.lower() {
            // if left exists, recurse
            if let Some(left) = self.left.as_ref() {
                left.get_entry(key)
            // otherwise, key doesn't exist
            } else { None }
        // otherwise, key is greater than self segment
        } else {
            // if right exists, recurse
            if let Some(right) = self.right.as_ref() {
                right.get_entry(key)
            // otherwise, key doesn't exist
            } else { None }
        }
    }

    pub fn insert(&mut self, segment: Segment<K>, value: V) {
        // if the segments perfectly overlap (this prevents inserting duplicate empty segments)
        if (segment.lower() == self.segment.lower()) && (segment.upper() == self.segment.upper()) {
            panic!("segments must not overlap");
        // if segment is less than self segment
        } else if segment.upper() <= self.segment.lower() {
            // if left exists, recurse
            if let Some(left) = self.left.as_mut() {
                left.insert(segment, value);
            // otherwise, set new left
            } else {
                self.left = Box::new(Some(SegmentMapNode::new(segment, value, None, None)));
            }
        // if segment is greater than self segment
        } else if segment.lower() >= self.segment.upper() {
            // if right exists, recurse
            if let Some(right) = self.right.as_mut() {
                right.insert(segment, value);
            // otherwise, set new right
            } else {
                self.right = Box::new(Some(SegmentMapNode::new(segment, value, None, None)));
            }
        // otherwise, segments overlap in some (non-perfect) way
        } else {
            panic!("segments must not overlap");
        }
    }
}

impl<K, V> SegmentMapNode<K, V> 
where
    K: Clone + PartialOrd,
    V: Clone,
{
    pub fn remove(mut self, segment: &Segment<K>) -> Option<SegmentMapNode<K, V>> {
        // empty segments can be removed
        if segment.is_empty() {
            // if empty segment is enclosed by self segment, (potentially) split the segment
            if self.segment.encloses(&segment) {
                // if empty segment exactly equals self segment
                if (segment.lower() == self.segment.lower()) && (segment.upper() == self.segment.upper()) {
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
                // if empty segment is touching left side of nonempty self segment, do not remove self
                } else if segment.lower() == self.segment.lower() {
                    // if left exists, recurse
                    self.left = Box::new(if let Some(left) = self.left.take() {
                        left.remove(segment)
                    // otherwise, nothing to remove
                    } else { None });
                    Some(self)
                // if empty segment is touching right side of nonempty self segment, do not remove self
                } else if segment.upper() == self.segment.upper() {
                    // if right exists, recurse
                    self.right = Box::new(if let Some(right) = self.right.take() {
                        right.remove(segment)
                    // otherwise, nothing to remove
                    } else { None });
                    Some(self)
                // otherwise, empty segment is within self segment
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
                    // reinsert left part of segment
                    let left_segment = Segment::new(self.segment.lower().clone(), segment.lower().clone());
                    // if result exists, do plain insert
                    if let Some(result) = result.as_mut() {
                        result.insert(left_segment, self.value.clone());
                    // otherwise, this is the new result
                    } else {
                        result = Some(SegmentMapNode::new(left_segment, self.value.clone(), None, None));
                    }
                    // reinsert right part of segment
                    let right_segment = Segment::new(segment.upper().clone(), self.segment.upper().clone());
                    // if result exists, do plain insert
                    if let Some(result) = result.as_mut() {
                        result.insert(right_segment, self.value.clone());
                    // otherwise, this is the new result
                    } else {
                        result = Some(SegmentMapNode::new(right_segment, self.value.clone(), None, None));
                    }
                    result
                }
            // if empty segment is less than self segment, recurse
            } else if segment.upper() < self.segment.lower() {
                // if left exists, recurse
                if let Some(left) = self.left.take() {
                    self.left = Box::new(left.remove(segment));
                } // otherwise, nothing to remove
                Some(self)
            // otherwise, empty segment is greater than self segment, recurse
            } else {
                // if right exists, recurse
                if let Some(right) = self.right.take() {
                    self.right = Box::new(right.remove(segment));
                } // otherwise, nothing to remove
                Some(self)
            }
        // if the segments overlap
        } else if let Some(intersection) = segment.intersection(&self.segment) {
            // if the overlap is empty, handle specially to prevent infinite recursion
            if intersection.is_empty() {
                // if segment is touching the right
                if segment.lower() == self.segment.upper() {
                    // if right exists, recurse
                    if let Some(right) = self.right.take() {
                        self.right = Box::new(right.remove(segment));
                    } // otherwise, nothing to remove
                // otherwise, segment is touching the left
                } else {
                    // if left exists, recurse
                    if let Some(left) = self.left.take() {
                        self.left = Box::new(left.remove(segment));
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
                // if left part of segment still needs to be removed
                if segment.lower() < intersection.lower() {
                    // if result exists, do plain remove
                    result = if let Some(result) = result {
                        result.remove(&Segment::new(segment.lower().clone(), intersection.lower().clone()))
                    // otherwise, nothing to remove
                    } else { None };
                // if left part of self still exists, reinsert
                } else if self.segment.lower() < intersection.lower() {
                    let segment = Segment::new(self.segment.lower().clone(), intersection.lower().clone());
                    // if result exists, do plain insert
                    if let Some(result) = result.as_mut() {
                        result.insert(segment, self.value.clone());
                    // otherwise, this is the new result
                    } else {
                        result = Some(SegmentMapNode::new(segment, self.value.clone(), None, None));
                    }
                }
                // if right part of segment still needs to be removed
                if segment.upper() > intersection.upper() {
                    // if result exists, do plain remove
                    result = if let Some(result) = result {
                        result.remove(&Segment::new(intersection.upper().clone(), segment.upper().clone()))
                    // otherwise, nothing to remove
                    } else { None };
                // if right part of self still exists, reinsert
                } else if self.segment.upper() > intersection.upper() {
                    let segment = Segment::new(intersection.upper().clone(), self.segment.upper().clone());
                    // if result exists, do plain insert
                    if let Some(result) = result.as_mut() {
                        result.insert(segment, self.value);
                    // otherwise, this is the new result
                    } else {
                        result = Some(SegmentMapNode::new(segment, self.value, None, None));
                    }
                }
                result
            }
        // otherwise, segments do not overlap
        } else {
            // if segment is greater than self segment
            if segment.lower() > self.segment.upper() {
                // if right exists, recurse
                if let Some(right) = self.right.take() {
                    self.right = Box::new(right.remove(segment));
                } // otherwise, there is nothing to remove
            // otherwise segment is less than self segment
            } else {
                // if left exists, recurse
                if let Some(left) = self.left.take() {
                    self.left = Box::new(left.remove(segment));
                } // otherwise, there is nothing to remove
            }
            Some(self)
        }
    }

    pub fn update<F>(self, segment: &Segment<K>, value: F) -> Option<SegmentMapNode<K, V>>
    where
        F: Fn(Option<V>) -> Option<V> + Clone
    {
        self.update_entry(segment, |_, v| value(v))
    }

    pub fn update_entry<F>(mut self, segment: &Segment<K>, value: F) -> Option<SegmentMapNode<K, V>>
    where
        F: Fn(&Segment<K>, Option<V>) -> Option<V> + Clone
    {
        // empty segments can be updated
        if segment.is_empty() {
            // if empty segment is enclosed by self segment, (potentially) split the segment
            if self.segment.encloses(&segment) {
                // if empty segment exactly equals self segment
                if (segment.lower() == self.segment.lower()) && (segment.upper() == self.segment.upper()) {
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
                    if let Some(value) = value(&segment, Some(self.value.clone())) {
                        // if result exists, do plain insert
                        if let Some(result) = result.as_mut() {
                            result.insert(segment.clone(), value);
                        // otherwise, this is the new result
                        } else {
                            result = Some(SegmentMapNode::new(segment.clone(), value, None, None));
                        }
                    };
                    result
                // if empty segment is touching left side of nonempty self segment, do not remove self
                } else if segment.lower() == self.segment.lower() {
                    // if left exists, recurse
                    if let Some(left) = self.left.take() {
                        self.left = Box::new(left.update_entry(segment, value));
                    // otherwise, if update produces a value, this is the new result
                    } else if let Some(value) = value(segment, None) {
                        self.left = Box::new(Some(SegmentMapNode::new(segment.clone(), value, None, None)));
                    }
                    Some(self)
                // if empty segment is touching right side of nonempty self segment, do not remove self
                } else if segment.upper() == self.segment.upper() {
                    // if right exists, recurse
                    if let Some(right) = self.right.take() {
                        self.right = Box::new(right.update_entry(segment, value));
                    // otherwise, if update produces a value, this is the new result
                    } else if let Some(value) = value(segment, None) {
                        self.right = Box::new(Some(SegmentMapNode::new(segment.clone(), value, None, None)));
                    }
                    Some(self)
                // otherwise, empty segment is within self segment
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
                    if let Some(value) = value(&segment, Some(self.value.clone())) {
                        // if result exists, do plain insert
                        if let Some(result) = result.as_mut() {
                            result.insert(segment.clone(), value);
                        // otherwise, this is the new result
                        } else {
                            result = Some(SegmentMapNode::new(segment.clone(), value, None, None));
                        }
                    };
                    // reinsert left part of segment
                    let left_segment = Segment::new(self.segment.lower().clone(), segment.lower().clone());
                    // if result exists, do plain insert
                    if let Some(result) = result.as_mut() {
                        result.insert(left_segment, self.value.clone());
                    // otherwise, this is the new result
                    } else {
                        result = Some(SegmentMapNode::new(left_segment, self.value.clone(), None, None));
                    }
                    // reinsert right part of segment
                    let right_segment = Segment::new(segment.upper().clone(), self.segment.upper().clone());
                    // if result exists, do plain insert
                    if let Some(result) = result.as_mut() {
                        result.insert(right_segment, self.value.clone());
                    // otherwise, this is the new result
                    } else {
                        result = Some(SegmentMapNode::new(right_segment, self.value.clone(), None, None));
                    }
                    result
                }
            // if empty segment is less than self segment, recurse
            } else if segment.upper() < self.segment.lower() {
                // if left exists, recurse
                if let Some(left) = self.left.take() {
                    self.left = Box::new(left.update_entry(segment, value));
                // otherwise, if update produces a value, this is the new result
                } else if let Some(value) = value(segment, None) {
                    self.left = Box::new(Some(SegmentMapNode::new(segment.clone(), value, None, None)));
                }
                Some(self)
            // otherwise, empty segment is greater than self segment, recurse
            } else {
                // if right exists, recurse
                if let Some(right) = self.right.take() {
                    self.right = Box::new(right.update_entry(segment, value));
                // otherwise, if update produces a value, this is the new result
                } else if let Some(value) = value(segment, None) {
                    self.right = Box::new(Some(SegmentMapNode::new(segment.clone(), value, None, None)));
                }
                Some(self)
            }
        // if the segments overlap
        } else if let Some(intersection) = segment.intersection(&self.segment) {
            // if the overlap is empty, handle specially to prevent infinite recursion
            if intersection.is_empty() {
                // if segment is touching the right
                if segment.lower() == self.segment.upper() {
                    // if right exists, recurse
                    if let Some(right) = self.right.take() {
                        self.right = Box::new(right.update_entry(segment, value));
                    // otherwise, if update produces a value, this is the new right
                    } else if let Some(value) = value(segment, None) {
                        self.right = Box::new(Some(SegmentMapNode::new(segment.clone(), value, None, None)));
                    }
                // otherwise, segment is touching the left
                } else {
                    // if left exists, recurse
                    if let Some(left) = self.left.take() {
                        self.left = Box::new(left.update_entry(segment, value));
                    // otherwise, if update produces a value, this is the new left
                    } else if let Some(value) = value(segment, None) {
                        self.left = Box::new(Some(SegmentMapNode::new(segment.clone(), value, None, None)));
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
                        result = Some(SegmentMapNode::new(intersection.clone(), value, None, None));
                    }
                }
                // if left part of segment still needs to be updated
                if segment.lower() < intersection.lower() {
                    let segment = Segment::new(segment.lower().clone(), intersection.lower().clone());
                    // if result exists, do plain update
                    result = if let Some(result) = result {
                        result.update_entry(&segment, value.clone())
                    // otherwise, if update produces a value, this is the new result
                    } else if let Some(value) = value(&segment, None) {
                        Some(SegmentMapNode::new(segment, value, None, None))
                    // otherwise, no result
                    } else { None }
                // if left part of self still exists, reinsert
                } else if self.segment.lower() < intersection.lower() {
                    let segment = Segment::new(self.segment.lower().clone(), intersection.lower().clone());
                    // if result exists, do plain insert
                    if let Some(result) = result.as_mut() {
                        result.insert(segment, self.value.clone());
                    // otherwise, this is the new result
                    } else {
                        result = Some(SegmentMapNode::new(segment, self.value.clone(), None, None));
                    }
                }
                // if right part of segment still needs to be updated
                if segment.upper() > intersection.upper() {
                    let segment = Segment::new(intersection.upper().clone(), segment.upper().clone());
                    // if result exists, do plain update
                    result = if let Some(result) = result {
                        result.update_entry(&segment, value)
                    // otherwise, if update produces value, this is the new result
                    } else if let Some(value) = value(&segment, None) {
                        Some(SegmentMapNode::new(segment, value, None, None))
                    // otherwise, no result
                    } else { None }
                // if right part of self still exists, reinsert
                } else if self.segment.upper() > intersection.upper() {
                    let segment = Segment::new(intersection.upper().clone(), self.segment.upper().clone());
                    // if result exists, do plain insert
                    if let Some(result) = result.as_mut() {
                        result.insert(segment, self.value);
                    // otherwise, this is the new result
                    } else {
                        result = Some(SegmentMapNode::new(segment, self.value, None, None));
                    }
                }
                result
            }
        // otherwise, segments do not overlap
        } else {
            // if segment is greater than self segment
            if segment.lower() > self.segment.upper() {
                // if right exists, recurse
                if let Some(right) = self.right.take() {
                    self.right = Box::new(right.update_entry(segment, value));
                // otherwise, if update produces value, this is the new right
                } else if let Some(value) = value(segment, None) {
                    self.right = Box::new(Some(SegmentMapNode::new(segment.clone(), value, None, None)));
                }
            // otherwise, segment is less than self segment
            } else {
                // if left exists, recurse
                if let Some(left) = self.left.take() {
                    self.left = Box::new(left.update_entry(segment, value));
                // otherwise, if update produces value, this is the new right
                } else if let Some(value) = value(segment, None) {
                    self.left = Box::new(Some(SegmentMapNode::new(segment.clone(), value, None, None)));
                }
            }
            Some(self)
        }
    }
}
