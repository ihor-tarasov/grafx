use crate::data;

pub unsafe trait Uniform : data::Pod + data::Zeroable { }

pub fn check_data_for_pod<T: data::Pod + data::Zeroable>(_: &T) {}

#[macro_export]
macro_rules! impl_uniform {
    {$visibility:vis struct $name:ident { $($field:ident: $field_type:ty),* $(,)?} } => {
        #[repr(C)]
        #[derive(Clone, Copy, Debug)]
        $visibility struct $name {
            $(
                $field: $field_type,
            )*
        }

        impl $name {
            fn _check_data_for_pod(&self) {
                $(
                    $crate::uniform::check_data_for_pod(&self.$field);
                )*
            }
        }

        unsafe impl $crate::uniform::Uniform for $name {}
        unsafe impl $crate::data::Pod for $name {}
        unsafe impl $crate::data::Zeroable for $name {}
    };
}
