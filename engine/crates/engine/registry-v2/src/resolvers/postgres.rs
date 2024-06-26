#[serde_with::minify_field_names(serialize = "minified", deserialize = "minified")]
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, Hash, PartialEq, Eq)]
pub struct PostgresResolver {
    pub operation: Operation,
    pub directive_name: String,
}

impl PostgresResolver {
    pub fn new(operation: Operation, directive_name: &str) -> Self {
        Self {
            operation,
            directive_name: directive_name.to_string(),
        }
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub enum Operation {
    FindOne,
    FindMany,
    DeleteOne,
    DeleteMany,
    CreateOne,
    CreateMany,
    UpdateOne,
    UpdateMany,
}

impl AsRef<str> for Operation {
    fn as_ref(&self) -> &str {
        match self {
            Self::FindOne => "findOne",
            Self::FindMany => "findMany",
            Self::DeleteOne => "deleteOne",
            Self::DeleteMany => "deleteMany",
            Self::CreateOne => "createOne",
            Self::CreateMany => "createMany",
            Self::UpdateOne => "updateOne",
            Self::UpdateMany => "updateMany",
        }
    }
}
