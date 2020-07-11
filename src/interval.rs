#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Interval<K> {
    lower: K,
    upper: K,
}

impl<K: PartialOrd> Interval<K> {
    pub fn new(lower: K, upper: K) -> Interval<K> {
        Interval { lower, upper }
    }

    pub fn contains(&self, value: &K) -> bool {
        (&self.lower <= value) && (value < &self.upper)
    }

    pub fn encloses(&self, other: &Interval<K>) -> bool {
        (self.lower <= other.lower) && (other.upper <= self.upper)
    }

    pub fn is_connected(&self, other: &Interval<K>) -> bool {
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

impl<K: Clone + PartialOrd> Interval<K> {
    pub fn intersection(&self, other: &Interval<K>) -> Option<Interval<K>> {
        if self.is_connected(other) {
            Some(Interval {
                lower: if self.lower < other.lower { other.lower.clone() } else { self.lower.clone() },
                upper: if other.upper < self.upper { other.upper.clone() } else { self.upper.clone() },
            })
        } else { None }
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
        //
        // -----[-----)----- -> false
        //   ^
        assert!(!Interval::new(5, 11).contains(&2));

        //
        // -----[-----)----- -> true
        //      ^
        assert!(Interval::new(5, 11).contains(&5));

        //
        // -----[-----)----- -> true
        //         ^
        assert!(Interval::new(5, 11).contains(&8));

        //
        // -----[-----)----- -> false
        //            ^
        assert!(!Interval::new(5, 11).contains(&11));

        //
        // -----[-----)----- -> false
        //               ^
        assert!(!Interval::new(5, 11).contains(&14));
    }

    #[test]
    fn test_encloses() {
        // -----[-----)-----
        //                   -> true
        // -----[-----)-----
        assert!(Interval::new(5, 11).encloses(&Interval::new(5, 11)));

        // -------[-)-------
        //                   -> false
        // -----[-----)-----
        assert!(!Interval::new(7, 9).encloses(&Interval::new(5, 11)));

        // -----[-----)-----
        //                   -> true
        // -------[-)-------
        assert!(Interval::new(5, 11).encloses(&Interval::new(7, 9)));

        // -----[---)-------
        //                   -> false
        // -----[-----)-----
        assert!(!Interval::new(5, 9).encloses(&Interval::new(5, 11)));

        // -----[-----)-----
        //                   -> true
        // -----[---)-------
        assert!(Interval::new(5, 11).encloses(&Interval::new(5, 9)));

        // -------[---)-----
        //                   -> false
        // -----[-----)-----
        assert!(!Interval::new(7, 11).encloses(&Interval::new(5, 11)));

        // -----[-----)-----
        //                   -> true
        // -------[---)----- 
        assert!(Interval::new(5, 11).encloses(&Interval::new(7, 11)));

        // -----[-----)-----
        //                   -> false
        // -------[-----)---
        assert!(!Interval::new(5, 11).encloses(&Interval::new(7, 13)));

        // -------[-----)---
        //                   -> false
        // -----[-----)-----
        assert!(!Interval::new(7, 13).encloses(&Interval::new(5, 11)));

        // --------[-----)--
        //                   -> false
        // --[-----)--------
        assert!(!Interval::new(8, 14).encloses(&Interval::new(2, 8)));

        // --[-----)--------
        //                   -> false
        // --------[-----)--
        assert!(!Interval::new(2, 8).encloses(&Interval::new(8, 14)));

        // ----------[-----)
        //                   -> false
        // [-----)----------
        assert!(!Interval::new(10, 16).encloses(&Interval::new(0, 6)));

        // [-----)----------
        //                   -> false
        // ----------[-----)
        assert!(!Interval::new(0, 6).encloses(&Interval::new(10, 16)));
    }

    #[test]
    fn test_intersection() {
        // -----[-----)-----
        //                   -> -----[-----)-----
        // -----[-----)-----
        assert_eq!(Some(Interval::new(5, 11)), Interval::new(5, 11).intersection(&Interval::new(5, 11)));

        // -------[-)-------
        //                   -> -------[-)-------
        // -----[-----)-----
        assert_eq!(Some(Interval::new(7, 9)), Interval::new(7, 9).intersection(&Interval::new(5, 11)));

        // -----[-----)-----
        //                   -> -------[-)-------
        // -------[-)-------
        assert_eq!(Some(Interval::new(7, 9)), Interval::new(5, 11).intersection(&Interval::new(7, 9)));

        // -----[---)-------
        //                   -> -----[---)-------
        // -----[-----)-----
        assert_eq!(Some(Interval::new(5, 9)), Interval::new(5, 9).intersection(&Interval::new(5, 11)));

        // -----[-----)-----
        //                   -> -----[---)-------
        // -----[---)-------
        assert_eq!(Some(Interval::new(5, 9)), Interval::new(5, 11).intersection(&Interval::new(5, 9)));

        // -------[---)-----
        //                   -> -------[---)-----
        // -----[-----)-----
        assert_eq!(Some(Interval::new(7, 11)), Interval::new(7, 11).intersection(&Interval::new(5, 11)));

        // -----[-----)-----
        //                   -> -------[---)-----
        // -------[---)----- 
        assert_eq!(Some(Interval::new(7, 11)), Interval::new(5, 11).intersection(&Interval::new(7, 11)));

        // -----[-----)-----
        //                   -> -------[---)-----
        // -------[-----)---
        assert_eq!(Some(Interval::new(7, 11)), Interval::new(5, 11).intersection(&Interval::new(7, 13)));

        // -------[-----)---
        //                   -> -------[---)-----
        // -----[-----)-----
        assert_eq!(Some(Interval::new(7, 11)), Interval::new(7, 13).intersection(&Interval::new(5, 11)));

        // --------[-----)--
        //                   -> -----------------
        // --[-----)--------
        assert_eq!(Some(Interval::new(8, 8)), Interval::new(8, 14).intersection(&Interval::new(2, 8)));

        // --[-----)--------
        //                   -> -----------------
        // --------[-----)--
        assert_eq!(Some(Interval::new(8, 8)), Interval::new(2, 8).intersection(&Interval::new(8, 14)));

        // ----------[-----)
        //                   -> None
        // [-----)----------
        assert_eq!(None, Interval::new(10, 16).intersection(&Interval::new(0, 6)));

        // [-----)----------
        //                   -> None
        // ----------[-----)
        assert_eq!(None, Interval::new(0, 6).intersection(&Interval::new(10, 16)));
    }

    #[test]
    fn test_is_connected() {
        // -----[-----)-----
        //                   -> true
        // -----[-----)-----
        assert!(Interval::new(5, 11).is_connected(&Interval::new(5, 11)));

        // -------[-)-------
        //                   -> true
        // -----[-----)-----
        assert!(Interval::new(7, 9).is_connected(&Interval::new(5, 11)));

        // -----[-----)-----
        //                   -> true
        // -------[-)-------
        assert!(Interval::new(5, 11).is_connected(&Interval::new(7, 9)));

        // -----[---)-------
        //                   -> true
        // -----[-----)-----
        assert!(Interval::new(5, 9).is_connected(&Interval::new(5, 11)));

        // -----[-----)-----
        //                   -> true
        // -----[---)-------
        assert!(Interval::new(5, 11).is_connected(&Interval::new(5, 9)));

        // -------[---)-----
        //                   -> true
        // -----[-----)-----
        assert!(Interval::new(7, 11).is_connected(&Interval::new(5, 11)));

        // -----[-----)-----
        //                   -> true
        // -------[---)----- 
        assert!(Interval::new(5, 11).is_connected(&Interval::new(7, 11)));

        // -----[-----)-----
        //                   -> true
        // -------[-----)---
        assert!(Interval::new(5, 11).is_connected(&Interval::new(7, 13)));

        // -------[-----)---
        //                   -> true
        // -----[-----)-----
        assert!(Interval::new(7, 13).is_connected(&Interval::new(5, 11)));

        // --------[-----)--
        //                   -> true
        // --[-----)--------
        assert!(Interval::new(8, 14).is_connected(&Interval::new(2, 8)));

        // --[-----)--------
        //                   -> true
        // --------[-----)--
        assert!(Interval::new(2, 8).is_connected(&Interval::new(8, 14)));

        // ----------[-----)
        //                   -> false
        // [-----)----------
        assert!(!Interval::new(10, 16).is_connected(&Interval::new(0, 6)));

        // [-----)----------
        //                   -> false
        // ----------[-----)
        assert!(!Interval::new(0, 6).is_connected(&Interval::new(10, 16)));
    }
    
    #[test]
    fn test_span() {
        // -----[-----)-----
        //                   -> -----[-----)-----
        // -----[-----)-----
        assert_eq!(Interval::new(5, 11), Interval::new(5, 11).span(&Interval::new(5, 11)));

        // -------[-)-------
        //                   -> -----[-----)-----
        // -----[-----)-----
        assert_eq!(Interval::new(5, 11), Interval::new(7, 9).span(&Interval::new(5, 11)));

        // -----[-----)-----
        //                   -> -----[-----)-----
        // -------[-)-------
        assert_eq!(Interval::new(5, 11), Interval::new(5, 11).span(&Interval::new(7, 9)));

        // -----[---)-------
        //                   -> -----[-----)-----
        // -----[-----)-----
        assert_eq!(Interval::new(5, 11), Interval::new(5, 9).span(&Interval::new(5, 11)));

        // -----[-----)-----
        //                   -> -----[-----)-----
        // -----[---)-------
        assert_eq!(Interval::new(5, 11), Interval::new(5, 11).span(&Interval::new(5, 9)));

        // -------[---)-----
        //                   -> -----[-----)-----
        // -----[-----)-----
        assert_eq!(Interval::new(5, 11), Interval::new(7, 11).span(&Interval::new(5, 11)));

        // -----[-----)-----
        //                   -> -----[-----)-----
        // -------[---)----- 
        assert_eq!(Interval::new(5, 11), Interval::new(5, 11).span(&Interval::new(7, 11)));

        // -----[-----)-----
        //                   -> -----[-------)---
        // -------[-----)---
        assert_eq!(Interval::new(5, 13), Interval::new(5, 11).span(&Interval::new(7, 13)));

        // -------[-----)---
        //                   -> -----[-------)---
        // -----[-----)-----
        assert_eq!(Interval::new(5, 13), Interval::new(7, 13).span(&Interval::new(5, 11)));

        // --------[-----)--
        //                   -> --[-----------)--
        // --[-----)--------
        assert_eq!(Interval::new(2, 14), Interval::new(8, 14).span(&Interval::new(2, 8)));

        // --[-----)--------
        //                   -> --[-----------)--
        // --------[-----)--
        assert_eq!(Interval::new(2, 14), Interval::new(2, 8).span(&Interval::new(8, 14)));

        // ----------[-----)
        //                   -> [---------------)
        // [-----)----------
        assert_eq!(Interval::new(0, 16), Interval::new(10, 16).span(&Interval::new(0, 6)));

        // [-----)----------
        //                   -> [---------------)
        // ----------[-----)
        assert_eq!(Interval::new(0, 16), Interval::new(0, 6).span(&Interval::new(10, 16)));
    }
}
