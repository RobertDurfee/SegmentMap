use std::ops::Add;

use num::One;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Interval<K> {
    lower: K,
    upper: K,
}

impl<K: Default> Interval<K> {
    pub fn empty() -> Interval<K> {
        Interval {
            lower: K::default(),
            upper: K::default(),
        }
    }
}

impl<K: Add<Output = K> + Clone + One> Interval<K> {
    pub fn singleton(value: K) -> Interval<K> {
        Interval {
            lower: value.clone(),
            upper: value + K::one(),
        }
    }
}

impl<K: PartialOrd> Interval<K> {
    pub fn new(lower: K, upper: K) -> Interval<K> {
        Interval {
            lower,
            upper,
        }
    }

    pub fn contains(&self, value: K) -> bool {
        (self.lower <= value) && (value < self.upper)
    }

    pub fn encloses(&self, other: &Interval<K>) -> bool {
        (self.lower <= other.lower) && (other.upper <= self.upper)
    }

    pub fn is_connected(&self, other: &Interval<K>) -> bool {
        (self.lower < other.upper) && (other.lower < self.upper)
    }

    pub fn is_empty(&self) -> bool {
        self.lower == self.upper
    }

    pub fn lower(&self) -> &K {
        &self.lower
    }

    pub fn upper(&self) -> &K {
        &self.upper
    }
}

impl<K: Clone + PartialOrd> Interval<K> {
    pub fn intersection(&self, other: &Interval<K>) -> Option<Interval<K>> {
        if self.is_connected(other) {
            Some(Interval {
                lower: if self.lower < other.lower { other.lower.clone() } else { self.lower.clone() },
                upper: if other.upper < self.upper { other.upper.clone() } else { self.upper.clone() },
            })
        } else {
            None
        }
    }

    pub fn union(&self, other: &Interval<K>) -> Option<Interval<K>> {
        if self.is_connected(other) {
            Some(self.span(other))
        } else {
            None
        }
    }

    pub fn span(&self, other: &Interval<K>) -> Interval<K> {
        Interval {
            lower: if self.lower < other.lower { self.lower.clone() } else { other.lower.clone() },
            upper: if other.upper < self.upper { self.upper.clone() } else { other.upper.clone() },
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Interval;

    #[test]
    fn test_contains() {
        assert!(!Interval::new(0, 5).contains(-1));
        assert!(Interval::new(0, 5).contains(0));
        assert!(Interval::new(0, 5).contains(3));
        assert!(!Interval::new(0, 5).contains(5));
        assert!(!Interval::new(0, 5).contains(6));
    }

    #[test]
    fn test_encloses() {
        assert!(Interval::new(0, 5).encloses(&Interval::new(1, 4)));
        assert!(Interval::new(0, 5).encloses(&Interval::new(0, 5)));
        assert!(!Interval::new(0, 5).encloses(&Interval::new(0, 6)));
        assert!(!Interval::new(0, 5).encloses(&Interval::new(-1, 5)));
        assert!(!Interval::new(0, 5).encloses(&Interval::new(-1, 6)));
    }

    #[test]
    fn test_intersection() {
        assert_eq!(Some(Interval::new(3, 5)), Interval::new(0, 5).intersection(&Interval::new(3, 7)));
        assert_eq!(Some(Interval::new(3, 5)), Interval::new(3, 7).intersection(&Interval::new(0, 5)));
        assert_eq!(None, Interval::new(0, 2).intersection(&Interval::new(4, 5)));
        assert_eq!(None, Interval::new(4, 5).intersection(&Interval::new(0, 2)));
        assert_eq!(None, Interval::new(0, 3).intersection(&Interval::new(3, 5)));
        assert_eq!(None, Interval::new(3, 5).intersection(&Interval::new(0, 3)));
    }

    #[test]
    fn test_is_connected() {
        assert!(Interval::new(0, 5).is_connected(&Interval::new(3, 7)));
        assert!(Interval::new(3, 7).is_connected(&Interval::new(0, 5)));
        assert!(!Interval::new(0, 2).is_connected(&Interval::new(4, 5)));
        assert!(!Interval::new(4, 5).is_connected(&Interval::new(0, 2)));
        assert!(!Interval::new(0, 3).is_connected(&Interval::new(3, 5)));
        assert!(!Interval::new(3, 5).is_connected(&Interval::new(0, 3)));
    }
    
    #[test]
    fn test_union() {
        assert_eq!(Some(Interval::new(0, 5)), Interval::new(0, 5).union(&Interval::new(0, 5)));
        assert_eq!(Some(Interval::new(0, 5)), Interval::new(0, 3).union(&Interval::new(2, 5)));
        assert_eq!(Some(Interval::new(0, 5)), Interval::new(2, 5).union(&Interval::new(0, 3)));
        assert_eq!(None, Interval::new(0, 1).union(&Interval::new(4, 5)));
        assert_eq!(None, Interval::new(4, 5).union(&Interval::new(0, 1)));
    }

    #[test]
    fn test_span() {
        assert_eq!(Interval::new(0, 5), Interval::new(0, 5).span(&Interval::new(0, 5)));
        assert_eq!(Interval::new(0, 5), Interval::new(0, 3).span(&Interval::new(2, 5)));
        assert_eq!(Interval::new(0, 5), Interval::new(2, 5).span(&Interval::new(0, 3)));
        assert_eq!(Interval::new(0, 5), Interval::new(0, 1).span(&Interval::new(4, 5)));
        assert_eq!(Interval::new(0, 5), Interval::new(4, 5).span(&Interval::new(0, 1)));
    }
}
