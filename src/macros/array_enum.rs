macro_rules! array_enum {
    (
        enum $name:ident {
            $( $enum_type:ident ),+ $(,)?
        }
    ) => {
        #[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
        pub enum $name {
            $(
                $enum_type,
            )*
        }

        impl $name {
            pub const ARRAY: [Self; Self::LEN] = [
                $(
                    Self::$enum_type,
                )*
            ];

            pub const LEN: usize = [
                    $(
                        Self::$enum_type,
                    )*
                ]
                    .len();

            pub fn index(&self) -> usize {
                *self as usize
            }

            #[allow(dead_code)]
            pub fn iter<'a>() -> iter_context::Iter<'a, Self, Self> {
                iter_context::Iter::new(Self::ARRAY.iter())
            }
        }
    };
    (
        enum $name:ident {
            type Array = struct $array:ident;
            $( $enum_type:ident, )+
        }
    ) => {
        array_enum! {
            enum $name {
                $( $enum_type, )*
            }
        }

        #[derive(Debug, Default, Copy, Clone)]
        pub struct $array <T> {
            values: [T; $name::LEN],
        }

        impl<T> $array <T> {
            pub const fn new(values: [T; <$name>::LEN]) -> Self {
                Self { values }
            }

            pub fn iter(&self) -> iter_context::Iter<$name, T> {
                iter_context::Iter::new(self.values.iter())
            }

            pub fn iter_mut(&mut self) -> iter_context::IterMut<$name, T> {
                iter_context::IterMut::new(self.values.iter_mut())
            }
        }

        impl<T> std::ops::Index<$name> for $array <T> {
            type Output = T;

            fn index(&self, index: $name) -> &Self::Output {
                &self.values[index.index()]
            }
        }

        impl<T> std::ops::IndexMut<$name> for $array <T> {
            fn index_mut(&mut self, index: $name) -> &mut Self::Output {
                &mut self.values[index.index()]
            }
        }
    };
}
