#[macro_export]
macro_rules! getter {
    ($name:ident, $type:ty) => {
        #[inline]
        pub fn $name(&self) -> &$type {
            &self.$name
        }
    };
}
