use crate::response::ResponseKey;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct QueryPath(im::Vector<ResponseKey>);

impl IntoIterator for QueryPath {
    type Item = ResponseKey;

    type IntoIter = <im::Vector<ResponseKey> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> IntoIterator for &'a QueryPath {
    type Item = &'a ResponseKey;

    type IntoIter = <&'a im::Vector<ResponseKey> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl QueryPath {
    pub fn empty() -> Self {
        Self::default()
    }

    pub fn child(&self, key: ResponseKey) -> Self {
        let mut child = self.clone();
        child.0.push_back(key);
        child
    }
}
