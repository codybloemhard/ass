/// trait that does `x as y`
/// but for generics.
pub trait Ass<T>{
    fn ass(self) -> T;
}
/// Implements the Ass trait like for this: `$from as $to`
macro_rules! impl_ass{
    ($from:ty,$to:ty) => {
        impl Ass<$to> for $from{
            fn ass(self) -> $to{
                self as $to
            }
        }
    }
}
/// Implements Ass for $from to common types.
macro_rules! impl_ass_list{
    ($from:ty) => {
        impl_ass!($from, u8);
        impl_ass!($from, u16);
        impl_ass!($from, u32);
        impl_ass!($from, u64);
        impl_ass!($from, u128);
        impl_ass!($from, i8);
        impl_ass!($from, i16);
        impl_ass!($from, i32);
        impl_ass!($from, i64);
        impl_ass!($from, i128);
        impl_ass!($from, f32);
        impl_ass!($from, f64);
    }
}

impl_ass_list!(u8);
impl_ass_list!(u16);
impl_ass_list!(u32);
impl_ass_list!(u64);
impl_ass_list!(u128);
impl_ass_list!(i8);
impl_ass_list!(i16);
impl_ass_list!(i32);
impl_ass_list!(i64);
impl_ass_list!(i128);
impl_ass_list!(f32);
impl_ass_list!(f64);

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
