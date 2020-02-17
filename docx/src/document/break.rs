use docx_codegen::{IntoOwned, XmlRead, XmlWrite};

use crate::{
    __string_enum,
    error::{Error, Result},
};

/// Break
///
/// ```rust
/// use docx::document::*;
///
/// let br = Break::from(BreakType::Page);
/// ```
#[derive(Debug, Default, XmlRead, XmlWrite, IntoOwned)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(leaf, tag = "w:br")]
pub struct Break {
    /// Specifies the break type of this break.
    #[xml(attr = "type")]
    pub ty: Option<BreakType>,
}

impl<T: Into<Option<BreakType>>> From<T> for Break {
    fn from(val: T) -> Self {
        Break { ty: val.into() }
    }
}

/// Specifies the break type of a break
///
/// The default value is TextWrapping.
#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub enum BreakType {
    /// Text restarts on the next column.
    Column,
    /// Text restarts on the next page.
    Page,
    /// Text restarts on the next line.
    TextWrapping,
}

__string_enum! {
    BreakType {
        Column = "column",
        Page = "page",
        TextWrapping = "textWrapping",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::__test_read_write;

    __test_read_write!(
        Break,
        Break::default(),
        r#"<w:br/>"#,
        Break::from(BreakType::Page),
        r#"<w:br type="page"/>"#,
    );
}
