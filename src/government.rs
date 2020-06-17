use super::*;

#[derive(Debug, Default)]
pub struct Government {
    pub alloc: Allocator<Self>,

    pub name: Component<Self, String>,

    pub population: Component<Self, Population>,
}

dynamic_arena!(Government, u16);

impl Government {
    pub fn create(&mut self, row: GovernmentRow, links: ()) -> Id<Self> {
        let id = self.alloc.create();

        self.name.insert(id, row.name);

        self.population.insert(id, Population::default());

        id.id
    }
}

#[derive(Debug, Clone)]
pub struct GovernmentRow {
    pub name: String,
}