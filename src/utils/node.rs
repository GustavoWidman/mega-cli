use std::{fmt::Display, hash::Hash};

#[derive(Clone)]
pub struct NodeWrapper<'a> {
    inner: Option<&'a mega::Node>,
}

impl<'a> NodeWrapper<'a> {
    pub fn new(node: &'a mega::Node) -> Self {
        NodeWrapper { inner: Some(node) }
    }
    pub fn new_empty() -> Self {
        NodeWrapper { inner: None }
    }

    pub fn name(&self) -> String {
        self.inner
            .map(|node| node.name().to_string())
            .unwrap_or_else(|| "Download all in folder.".to_string())
    }

    pub fn into_inner(self) -> Option<&'a mega::Node> {
        self.inner
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_none()
    }
}

impl Hash for NodeWrapper<'_> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        if let Some(node) = self.inner {
            node.handle().hash(state);
        } else {
            0.hash(state);
        }
    }
}

impl PartialEq for NodeWrapper<'_> {
    fn eq(&self, other: &Self) -> bool {
        if let (Some(node1), Some(node2)) = (self.inner, other.inner) {
            node1 == node2
        } else {
            self.is_empty() && other.is_empty()
        }
    }
}

impl Eq for NodeWrapper<'_> {}

impl Display for NodeWrapper<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}
