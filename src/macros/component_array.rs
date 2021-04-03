macro_rules! component_array {
    ($name:ident, $enum:ty, $array:ty) => {
        #[derive(Debug)]
        pub struct $name<ID, T> {
            components: [Component<ID, T>; <$enum>::LEN],
        }

        impl<ID, T> Default for $name<ID, T> {
            fn default() -> Self {
                Self {
                    components: Default::default(),
                }
            }
        }

        impl<ID, T: Clone> Clone for $name<ID, T> {
            fn clone(&self) -> Self {
                Self {
                    components: self.components.clone(),
                }
            }
        }

        impl<ID, T: Clone> $name<ID, T> {
            pub fn insert<I: ValidId<ID>>(&mut self, id: I, value: T) {
                self.components
                    .iter_mut()
                    .for_each(|comp| comp.insert(id, value.clone()));
            }
        }

        impl<ID, T> $name<ID, T> {
            pub fn get(&self, index: $enum) -> &Component<ID, T> {
                &self.components[index.index()]
            }

            pub fn get_mut(&mut self, index: Resource) -> &mut Component<ID, T> {
                &mut self.components[index.index()]
            }

            pub fn iter(&self) -> iter_context::Iter<$enum, Component<ID, T>> {
                iter_context::Iter::new(self.components.iter())
            }

            pub fn iter_mut(&mut self) -> iter_context::IterMut<$enum, Component<ID, T>> {
                iter_context::IterMut::new(self.components.iter_mut())
            }

            pub fn iter_enum(
                &self,
            ) -> iter_context::Zip<
                $enum,
                iter_context::Iter<$enum, Component<ID, T>>,
                iter_context::Iter<$enum, $enum>,
            > {
                self.iter().zip(<$enum>::iter())
            }

            pub fn iter_enum_mut(
                &mut self,
            ) -> iter_context::Zip<
                $enum,
                iter_context::IterMut<$enum, Component<ID, T>>,
                iter_context::Iter<$enum, $enum>,
            > {
                self.iter_mut().zip(<$enum>::iter())
            }

            pub fn fill_with<F: Fn() -> T + Copy>(&mut self, f: F) {
                self.components
                    .iter_mut()
                    .for_each(|comp| comp.fill_with(f));
            }
        }
    };
}
