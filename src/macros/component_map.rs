macro_rules! component_map {
    ($name:ident, $enum:ty) => {
        #[derive(Debug)]
        pub struct $name<ID, T> {
            map: [IdMap<ID, T>; <$enum>::LEN],
        }

        impl<ID, T> Default for $name<ID, T> {
            fn default() -> Self {
                Self {
                    map: Default::default(),
                }
            }
        }

        impl<ID, T> $name<ID, T> {
            pub fn get(&self, value: $enum) -> &IdMap<ID, T> {
                &self.map[value.index()]
            }

            pub fn get_mut(&mut self, value: $enum) -> &mut IdMap<ID, T> {
                &mut self.map[value.index()]
            }

            pub fn iter(&self) -> iter_context::Iter<$enum, IdMap<ID, T>> {
                iter_context::Iter::new(self.map.iter())
            }

            pub fn iter_mut(&mut self) -> iter_context::IterMut<$enum, IdMap<ID, T>> {
                iter_context::IterMut::new(self.map.iter_mut())
            }

            pub fn iter_enum(
                &self,
            ) -> iter_context::Zip<
                $enum,
                iter_context::Iter<$enum, IdMap<ID, T>>,
                iter_context::Iter<$enum, $enum>,
            > {
                self.iter().zip(<$enum>::iter())
            }

            pub fn iter_enum_mut(
                &mut self,
            ) -> iter_context::Zip<
                $enum,
                iter_context::IterMut<$enum, IdMap<ID, T>>,
                iter_context::Iter<$enum, $enum>,
            > {
                self.iter_mut().zip(<$enum>::iter())
            }
        }
    };
}
