use super::*;

#[derive(Debug, Default)]
pub struct Government {
    pub alloc: Allocator<Self>,

    pub name: Component<Self, String>,
}

dynamic_arena!(Government, u16);
