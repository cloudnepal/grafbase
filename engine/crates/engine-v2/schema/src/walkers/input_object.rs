use super::SchemaWalker;
use crate::{InputObjectId, InputValueWalker};

pub type InputObjectWalker<'a> = SchemaWalker<'a, InputObjectId>;

impl<'a> InputObjectWalker<'a> {
    pub fn name(&self) -> &'a str {
        self.names.input_object(self.id)
    }

    pub fn description(&self) -> Option<&'a str> {
        self.description.map(|id| self.schema[id].as_str())
    }

    pub fn input_fields(&self) -> impl Iterator<Item = InputValueWalker<'a>> + 'a {
        let walker = *self;
        self.schema[self.id].input_fields.iter().map(move |id| walker.walk(*id))
    }
}

impl<'a> std::fmt::Debug for InputObjectWalker<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("InputObjectWalker")
            .field("id", &usize::from(self.id))
            .field("name", &self.name())
            .field("description", &self.description())
            .field(
                "input_fields",
                &self.input_fields().map(|f| (f.name(), f.ty())).collect::<Vec<_>>(),
            )
            .finish()
    }
}