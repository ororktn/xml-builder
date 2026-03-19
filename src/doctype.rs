//! Doctype models

/// XML <!DOCTYPE ...>
pub struct Doctype {
    /// The name of the document type
    name: String,
}

impl Doctype {
    /// Create a new <DOCTYPE>
    pub fn new(name: String) -> Self {
        Self {
            name,
        }
    }

    /// Get the doctype name
    pub fn get_name(&self) -> &str {
        &self.name
    }
}
