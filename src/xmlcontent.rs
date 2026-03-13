use crate::XMLElement;

/// An enum value representing the types of XML contents
#[derive(Clone)]
pub(crate) enum XMLElementContent {
    /// The content is an XML element.
    Element(Box<XMLElement>),

    /// The content is a textual string.
    Text(String),
}
