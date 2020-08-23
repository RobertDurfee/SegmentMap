pub trait Next: Clone + PartialOrd {
    fn next_checked(&self) -> Option<Self>;
    fn next_unchecked(&self) -> Self { self.next_checked().expect("overflow") }
}

impl Next for usize {
    fn next_checked(&self) -> Option<usize> { self.checked_add(1) }
}

impl Next for u8 {
    fn next_checked(&self) -> Option<u8> { self.checked_add(1) }
}

impl Next for u16 {
    fn next_checked(&self) -> Option<u16> { self.checked_add(1) }
}

impl Next for u32 {
    fn next_checked(&self) -> Option<u32> { self.checked_add(1) }
}

impl Next for u64 {
    fn next_checked(&self) -> Option<u64> { self.checked_add(1) }
}

impl Next for u128 {
    fn next_checked(&self) -> Option<u128> { self.checked_add(1) }
}

impl Next for isize {
    fn next_checked(&self) -> Option<isize> { self.checked_add(1) }
}

impl Next for i8 {
    fn next_checked(&self) -> Option<i8> { self.checked_add(1) }
}

impl Next for i16 {
    fn next_checked(&self) -> Option<i16> { self.checked_add(1) }
}

impl Next for i32 {
    fn next_checked(&self) -> Option<i32> { self.checked_add(1) }
}

impl Next for i64 {
    fn next_checked(&self) -> Option<i64> { self.checked_add(1) }
}

impl Next for i128 {
    fn next_checked(&self) -> Option<i128> { self.checked_add(1) }
}
