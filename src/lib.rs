#![crate_type = "lib"]
#![forbid(unsafe_code)]
#![forbid(missing_docs)]
#![doc = include_str!("../README.md")]

mod builder;
mod doctype;
mod utils;
mod xml;
mod xmlcontent;
mod xmlelement;
mod xmlerror;
mod xmlversion;

pub use builder::XMLBuilder;
pub use doctype::Doctype;
pub use xml::XML;
pub use xmlelement::XMLElement;
pub use xmlerror::{Result, XMLError};
pub use xmlversion::XMLVersion;

use utils::escape_str;
use xmlcontent::XMLElementContent;
