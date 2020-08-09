use std::ops::Add;
use num::{
    Bounded,
    Zero,
    One,
};

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Segment<K> {
    lower: K,
    upper: K,
}

impl<K: PartialOrd> Segment<K> {
    pub fn new(lower: K, upper: K) -> Segment<K> {
        Segment { lower, upper }
    }

    pub fn contains(&self, value: &K) -> bool {
        (&self.lower <= value) && (value < &self.upper)
    }

    pub fn encloses(&self, other: &Segment<K>) -> bool {
        (self.lower <= other.lower) && (other.upper <= self.upper)
    }

    pub fn is_connected(&self, other: &Segment<K>) -> bool {
        (self.lower <= other.upper) && (other.lower <= self.upper)
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

impl<K: Clone + PartialOrd> Segment<K> {
    pub fn intersection(&self, other: &Segment<K>) -> Option<Segment<K>> {
        if self.is_connected(other) {
            Some(Segment {
                lower: if self.lower < other.lower { other.lower.clone() } else { self.lower.clone() },
                upper: if other.upper < self.upper { other.upper.clone() } else { self.upper.clone() },
            })
        } else { None }
    }

    pub fn span(&self, other: &Segment<K>) -> Segment<K> {
        Segment {
            lower: if self.lower < other.lower { self.lower.clone() } else { other.lower.clone() },
            upper: if other.upper < self.upper { self.upper.clone() } else { other.upper.clone() },
        }
    }
}

impl<K: PartialOrd + Zero> Segment<K> {
    pub fn empty() -> Segment<K> {
        Segment { lower: K::zero(), upper: K::zero() }
    }
}

impl<K: Add<Output = K> + Clone + One + PartialOrd> Segment<K> {
    pub fn singleton(value: K) -> Segment<K> {
        Segment { lower: value.clone(), upper: value + K::one() }
    }
}

impl<K: Add<Output = K> + One + PartialOrd> Segment<K> {
    pub fn open(lower: K, upper: K) -> Segment<K> {
        Segment { lower: lower + K::one(), upper }
    }

    pub fn closed(lower: K, upper: K) -> Segment<K> {
        Segment { lower, upper: upper + K::one() }
    }

    pub fn open_closed(lower: K, upper: K) -> Segment<K> {
        Segment { lower: lower + K::one(), upper: upper + K::one() }
    }
}

impl<K: PartialOrd> Segment<K> {
    pub fn closed_open(lower: K, upper: K) -> Segment<K> {
        Segment { lower, upper }
    }
}

impl<K: Add<Output = K> + Bounded + One + PartialOrd> Segment<K> {
    pub fn at_most(value: K) -> Segment<K> {
        Segment { lower: K::min_value(), upper: value + K::one() }
    }
}

impl<K: Bounded + PartialOrd> Segment<K> {
    pub fn at_least(value: K) -> Segment<K> {
        Segment { lower: value, upper: K::max_value() }
    }

    pub fn less_than(value: K) -> Segment<K> {
        Segment { lower: K::min_value(), upper: value }
    }
}

impl<K: Add<Output = K> + Bounded + One + PartialOrd> Segment<K> {
    pub fn greater_than(value: K) -> Segment<K> {
        Segment { lower: value + K::one(), upper: K::max_value() }
    }
}

impl<K: Bounded + PartialOrd> Segment<K> {
    pub fn all() -> Segment<K> {
        Segment { lower: K::min_value(), upper: K::max_value() }
    }
}

#[cfg(test)]
mod tests {
    use crate::Segment;

    #[test]
    fn test_contains() {
        //
        // -----[-----)----- -> false
        //   ^
        assert!(!Segment::new(5, 11).contains(&2));

        //
        // -----[-----)----- -> true
        //      ^
        assert!(Segment::new(5, 11).contains(&5));

        //
        // -----[-----)----- -> true
        //         ^
        assert!(Segment::new(5, 11).contains(&8));

        //
        // -----[-----)----- -> false
        //            ^
        assert!(!Segment::new(5, 11).contains(&11));

        //
        // -----[-----)----- -> false
        //               ^
        assert!(!Segment::new(5, 11).contains(&14));
    }

    #[test]
    fn test_encloses() {
        // -----[-----)-----
        //                   -> true
        // -----[-----)-----
        assert!(Segment::new(5, 11).encloses(&Segment::new(5, 11)));

        // -------[-)-------
        //                   -> false
        // -----[-----)-----
        assert!(!Segment::new(7, 9).encloses(&Segment::new(5, 11)));

        // -----[-----)-----
        //                   -> true
        // -------[-)-------
        assert!(Segment::new(5, 11).encloses(&Segment::new(7, 9)));

        // -----[---)-------
        //                   -> false
        // -----[-----)-----
        assert!(!Segment::new(5, 9).encloses(&Segment::new(5, 11)));

        // -----[-----)-----
        //                   -> true
        // -----[---)-------
        assert!(Segment::new(5, 11).encloses(&Segment::new(5, 9)));

        // -------[---)-----
        //                   -> false
        // -----[-----)-----
        assert!(!Segment::new(7, 11).encloses(&Segment::new(5, 11)));

        // -----[-----)-----
        //                   -> true
        // -------[---)----- 
        assert!(Segment::new(5, 11).encloses(&Segment::new(7, 11)));

        // -----[-----)-----
        //                   -> false
        // -------[-----)---
        assert!(!Segment::new(5, 11).encloses(&Segment::new(7, 13)));

        // -------[-----)---
        //                   -> false
        // -----[-----)-----
        assert!(!Segment::new(7, 13).encloses(&Segment::new(5, 11)));

        // --------[-----)--
        //                   -> false
        // --[-----)--------
        assert!(!Segment::new(8, 14).encloses(&Segment::new(2, 8)));

        // --[-----)--------
        //                   -> false
        // --------[-----)--
        assert!(!Segment::new(2, 8).encloses(&Segment::new(8, 14)));

        // ----------[-----)
        //                   -> false
        // [-----)----------
        assert!(!Segment::new(10, 16).encloses(&Segment::new(0, 6)));

        // [-----)----------
        //                   -> false
        // ----------[-----)
        assert!(!Segment::new(0, 6).encloses(&Segment::new(10, 16)));
    }

    #[test]
    fn test_intersection() {
        // -----[-----)-----
        //                   -> -----[-----)-----
        // -----[-----)-----
        assert_eq!(Some(Segment::new(5, 11)), Segment::new(5, 11).intersection(&Segment::new(5, 11)));

        // -------[-)-------
        //                   -> -------[-)-------
        // -----[-----)-----
        assert_eq!(Some(Segment::new(7, 9)), Segment::new(7, 9).intersection(&Segment::new(5, 11)));

        // -----[-----)-----
        //                   -> -------[-)-------
        // -------[-)-------
        assert_eq!(Some(Segment::new(7, 9)), Segment::new(5, 11).intersection(&Segment::new(7, 9)));

        // -----[---)-------
        //                   -> -----[---)-------
        // -----[-----)-----
        assert_eq!(Some(Segment::new(5, 9)), Segment::new(5, 9).intersection(&Segment::new(5, 11)));

        // -----[-----)-----
        //                   -> -----[---)-------
        // -----[---)-------
        assert_eq!(Some(Segment::new(5, 9)), Segment::new(5, 11).intersection(&Segment::new(5, 9)));

        // -------[---)-----
        //                   -> -------[---)-----
        // -----[-----)-----
        assert_eq!(Some(Segment::new(7, 11)), Segment::new(7, 11).intersection(&Segment::new(5, 11)));

        // -----[-----)-----
        //                   -> -------[---)-----
        // -------[---)----- 
        assert_eq!(Some(Segment::new(7, 11)), Segment::new(5, 11).intersection(&Segment::new(7, 11)));

        // -----[-----)-----
        //                   -> -------[---)-----
        // -------[-----)---
        assert_eq!(Some(Segment::new(7, 11)), Segment::new(5, 11).intersection(&Segment::new(7, 13)));

        // -------[-----)---
        //                   -> -------[---)-----
        // -----[-----)-----
        assert_eq!(Some(Segment::new(7, 11)), Segment::new(7, 13).intersection(&Segment::new(5, 11)));

        // --------[-----)--
        //                   -> -----------------
        // --[-----)--------
        assert_eq!(Some(Segment::new(8, 8)), Segment::new(8, 14).intersection(&Segment::new(2, 8)));

        // --[-----)--------
        //                   -> -----------------
        // --------[-----)--
        assert_eq!(Some(Segment::new(8, 8)), Segment::new(2, 8).intersection(&Segment::new(8, 14)));

        // ----------[-----)
        //                   -> None
        // [-----)----------
        assert_eq!(None, Segment::new(10, 16).intersection(&Segment::new(0, 6)));

        // [-----)----------
        //                   -> None
        // ----------[-----)
        assert_eq!(None, Segment::new(0, 6).intersection(&Segment::new(10, 16)));
    }

    #[test]
    fn test_is_connected() {
        // -----[-----)-----
        //                   -> true
        // -----[-----)-----
        assert!(Segment::new(5, 11).is_connected(&Segment::new(5, 11)));

        // -------[-)-------
        //                   -> true
        // -----[-----)-----
        assert!(Segment::new(7, 9).is_connected(&Segment::new(5, 11)));

        // -----[-----)-----
        //                   -> true
        // -------[-)-------
        assert!(Segment::new(5, 11).is_connected(&Segment::new(7, 9)));

        // -----[---)-------
        //                   -> true
        // -----[-----)-----
        assert!(Segment::new(5, 9).is_connected(&Segment::new(5, 11)));

        // -----[-----)-----
        //                   -> true
        // -----[---)-------
        assert!(Segment::new(5, 11).is_connected(&Segment::new(5, 9)));

        // -------[---)-----
        //                   -> true
        // -----[-----)-----
        assert!(Segment::new(7, 11).is_connected(&Segment::new(5, 11)));

        // -----[-----)-----
        //                   -> true
        // -------[---)----- 
        assert!(Segment::new(5, 11).is_connected(&Segment::new(7, 11)));

        // -----[-----)-----
        //                   -> true
        // -------[-----)---
        assert!(Segment::new(5, 11).is_connected(&Segment::new(7, 13)));

        // -------[-----)---
        //                   -> true
        // -----[-----)-----
        assert!(Segment::new(7, 13).is_connected(&Segment::new(5, 11)));

        // --------[-----)--
        //                   -> true
        // --[-----)--------
        assert!(Segment::new(8, 14).is_connected(&Segment::new(2, 8)));

        // --[-----)--------
        //                   -> true
        // --------[-----)--
        assert!(Segment::new(2, 8).is_connected(&Segment::new(8, 14)));

        // ----------[-----)
        //                   -> false
        // [-----)----------
        assert!(!Segment::new(10, 16).is_connected(&Segment::new(0, 6)));

        // [-----)----------
        //                   -> false
        // ----------[-----)
        assert!(!Segment::new(0, 6).is_connected(&Segment::new(10, 16)));
    }
    
    #[test]
    fn test_span() {
        // -----[-----)-----
        //                   -> -----[-----)-----
        // -----[-----)-----
        assert_eq!(Segment::new(5, 11), Segment::new(5, 11).span(&Segment::new(5, 11)));

        // -------[-)-------
        //                   -> -----[-----)-----
        // -----[-----)-----
        assert_eq!(Segment::new(5, 11), Segment::new(7, 9).span(&Segment::new(5, 11)));

        // -----[-----)-----
        //                   -> -----[-----)-----
        // -------[-)-------
        assert_eq!(Segment::new(5, 11), Segment::new(5, 11).span(&Segment::new(7, 9)));

        // -----[---)-------
        //                   -> -----[-----)-----
        // -----[-----)-----
        assert_eq!(Segment::new(5, 11), Segment::new(5, 9).span(&Segment::new(5, 11)));

        // -----[-----)-----
        //                   -> -----[-----)-----
        // -----[---)-------
        assert_eq!(Segment::new(5, 11), Segment::new(5, 11).span(&Segment::new(5, 9)));

        // -------[---)-----
        //                   -> -----[-----)-----
        // -----[-----)-----
        assert_eq!(Segment::new(5, 11), Segment::new(7, 11).span(&Segment::new(5, 11)));

        // -----[-----)-----
        //                   -> -----[-----)-----
        // -------[---)----- 
        assert_eq!(Segment::new(5, 11), Segment::new(5, 11).span(&Segment::new(7, 11)));

        // -----[-----)-----
        //                   -> -----[-------)---
        // -------[-----)---
        assert_eq!(Segment::new(5, 13), Segment::new(5, 11).span(&Segment::new(7, 13)));

        // -------[-----)---
        //                   -> -----[-------)---
        // -----[-----)-----
        assert_eq!(Segment::new(5, 13), Segment::new(7, 13).span(&Segment::new(5, 11)));

        // --------[-----)--
        //                   -> --[-----------)--
        // --[-----)--------
        assert_eq!(Segment::new(2, 14), Segment::new(8, 14).span(&Segment::new(2, 8)));

        // --[-----)--------
        //                   -> --[-----------)--
        // --------[-----)--
        assert_eq!(Segment::new(2, 14), Segment::new(2, 8).span(&Segment::new(8, 14)));

        // ----------[-----)
        //                   -> [---------------)
        // [-----)----------
        assert_eq!(Segment::new(0, 16), Segment::new(10, 16).span(&Segment::new(0, 6)));

        // [-----)----------
        //                   -> [---------------)
        // ----------[-----)
        assert_eq!(Segment::new(0, 16), Segment::new(0, 6).span(&Segment::new(10, 16)));
    }
}
