pub trait Next: Clone + PartialOrd {
    fn next(&self) -> Self;
}

impl Next for usize {
    fn next(&self) -> usize { self.checked_add(1).expect("integer overflow") }
}

impl Next for u8 {
    fn next(&self) -> u8 { self.checked_add(1).expect("integer overflow") }
}

impl Next for u16 {
    fn next(&self) -> u16 { self.checked_add(1).expect("integer overflow") }
}

impl Next for u32 {
    fn next(&self) -> u32 { self.checked_add(1).expect("integer overflow") }
}

impl Next for u64 {
    fn next(&self) -> u64 { self.checked_add(1).expect("integer overflow") }
}

impl Next for u128 {
    fn next(&self) -> u128 { self.checked_add(1).expect("integer overflow") }
}

impl Next for isize {
    fn next(&self) -> isize { self.checked_add(1).expect("integer overflow") }
}

impl Next for i8 {
    fn next(&self) -> i8 { self.checked_add(1).expect("integer overflow") }
}

impl Next for i16 {
    fn next(&self) -> i16 { self.checked_add(1).expect("integer overflow") }
}

impl Next for i32 {
    fn next(&self) -> i32 { self.checked_add(1).expect("integer overflow") }
}

impl Next for i64 {
    fn next(&self) -> i64 { self.checked_add(1).expect("integer overflow") }
}

impl Next for i128 {
    fn next(&self) -> i128 { self.checked_add(1).expect("integer overflow") }
}
