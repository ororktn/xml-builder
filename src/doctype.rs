//! Doctype models

use crate::XMLError;

pub(crate) enum ExternalId {
    System(String),
    Public { id: String, uri: String },
}

impl std::fmt::Display for ExternalId {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match self {
            Self::System(s) => write!(f, r#"SYSTEM "{s}""#),
            Self::Public { id, uri } => write!(f, r#"PUBLIC "{id}" "{uri}""#),
        }
    }
}

/// XML <!DOCTYPE ...>
pub struct Doctype {
    /// The name of the document type
    name: String,

    /// External ID
    external_id: Option<ExternalId>,
}

impl Doctype {
    /// Create a new <DOCTYPE>
    pub fn new(name: String) -> Self {
        Self {
            name,
            external_id: None,
        }
    }

    // FIXME: Injectable
    #[must_use]
    /// Add a SYSTEM external id to the DOCTYPE
    pub fn add_system_id(&mut self, id: String) -> Result<(), XMLError> {
        if let Some(other) = &self.external_id {
            Err(XMLError::InsertError(format!(
                "DOCTYPE already has an external id: {other}"
            )))
        } else {
            self.external_id = Some(ExternalId::System(id));

            Ok(())
        }
    }

    // FIXME: Injectable
    #[must_use]
    /// Add a PUBLIC external id to the DOCTYPE
    pub fn add_public_id(&mut self, id: String, uri: String) -> Result<(), XMLError> {
        if let Some(other) = &self.external_id {
            Err(XMLError::InsertError(format!(
                "DOCTYPE already has an external id: {other}"
            )))
        } else {
            self.external_id = Some(ExternalId::Public { id, uri });

            Ok(())
        }
    }
    /// Get the doctype name
    pub fn get_name(&self) -> &str {
        &self.name
    }
}

impl std::fmt::Display for Doctype {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "<!DOCTYPE {}{}>",
            self.name,
            if let Some(id) = &self.external_id {
                " ".to_string() + &id.to_string()
            } else {
                String::new()
            }
        )
    }
}
