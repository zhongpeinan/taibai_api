//! Path represents a path from some root to a particular field.
//!
//! Ported from k8s.io/apimachinery/pkg/util/validation/field/path.go

use std::fmt;

/// Path represents a path from some root to a particular field.
///
/// This is a linked-list structure where each node contains:
/// - `name`: name of this field or empty if this is an index
/// - `index`: if name is empty, this is a subscript (index or map key) of previous element
/// - `parent`: pointer to parent node, None if this is root
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Path {
    pub(crate) name: String,
    pub(crate) index: String,
    pub(crate) parent: Option<Box<Path>>,
    pub(crate) is_nil: bool,
}

impl Path {
    /// Creates a root Path object with the given name(s).
    ///
    /// # Examples
    /// ```
    /// # use taibai_api::common::validation::Path;
    /// let p = Path::new("metadata");
    /// assert_eq!(p.to_string(), "metadata");
    ///
    /// let p = Path::new_with_segments(&["metadata", "name"]);
    /// assert_eq!(p.to_string(), "metadata.name");
    /// ```
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            index: String::new(),
            parent: None,
            is_nil: false,
        }
    }

    /// Creates a nil Path object.
    pub fn nil() -> Self {
        Self {
            name: String::new(),
            index: String::new(),
            parent: None,
            is_nil: true,
        }
    }

    /// Creates a root Path object with multiple segments.
    ///
    /// # Examples
    /// ```
    /// # use taibai_api::common::validation::Path;
    /// let p = Path::new_with_segments(&["metadata", "name"]);
    /// assert_eq!(p.to_string(), "metadata.name");
    /// ```
    pub fn new_with_segments(segments: &[&str]) -> Self {
        if segments.is_empty() {
            return Self::nil();
        }
        let mut r = Self::new(segments[0]);
        for segment in &segments[1..] {
            r = r.child(segment);
        }
        r
    }

    /// Child creates a new Path that is a child of method receiver.
    ///
    /// # Examples
    /// ```
    /// # use taibai_api::common::validation::Path;
    /// let root = Path::new("metadata");
    /// let child = root.child("name");
    /// assert_eq!(child.to_string(), "metadata.name");
    /// ```
    pub fn child(&self, name: &str) -> Self {
        Self {
            name: name.to_string(),
            index: String::new(),
            parent: Some(Box::new(self.clone())),
            is_nil: false,
        }
    }

    /// Index indicates that the previous Path is to be subscripted by an int.
    ///
    /// # Examples
    /// ```
    /// # use taibai_api::common::validation::Path;
    /// let p = Path::new("items").index(0);
    /// assert_eq!(p.to_string(), "items[0]");
    /// ```
    pub fn index(&self, idx: usize) -> Self {
        Self {
            name: String::new(),
            index: idx.to_string(),
            parent: Some(Box::new(self.clone())),
            is_nil: false,
        }
    }

    /// Key indicates that the previous Path is to be subscripted by a string.
    ///
    /// # Examples
    /// ```
    /// # use taibai_api::common::validation::Path;
    /// let p = Path::new("annotations").key("key");
    /// assert_eq!(p.to_string(), "annotations[key]");
    /// ```
    pub fn key(&self, key: &str) -> Self {
        Self {
            name: String::new(),
            index: key.to_string(),
            parent: Some(Box::new(self.clone())),
            is_nil: false,
        }
    }

    /// Returns the root element of this Path.
    pub fn root(&self) -> &Path {
        let mut p = self;
        while let Some(ref parent) = p.parent {
            p = parent;
        }
        p
    }
}

impl Default for Path {
    fn default() -> Self {
        Self::nil()
    }
}

impl fmt::Display for Path {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.parent.is_none() && self.name.is_empty() && self.index.is_empty() {
            return if self.is_nil {
                write!(f, "<nil>")
            } else {
                Ok(())
            };
        }

        // Collect all path elements into a vec in reverse order
        let mut elems = Vec::new();
        let mut p = Some(self);
        while let Some(curr) = p {
            elems.push(curr);
            p = curr.parent.as_deref();
        }

        // Iterate in reverse order
        for elem in elems.iter().rev() {
            if elem.name.is_empty() && elem.index.is_empty() {
                continue;
            }
            if elem.parent.is_some() && !elem.name.is_empty() {
                write!(f, ".")?;
            }
            if !elem.name.is_empty() {
                write!(f, "{}", elem.name)?;
            } else {
                write!(f, "[{}]", elem.index)?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_path_new() {
        let p = Path::new("metadata");
        assert_eq!(p.to_string(), "metadata");
    }

    #[test]
    fn test_path_new_empty() {
        let p = Path::new("");
        assert_eq!(p.to_string(), "");
    }

    #[test]
    fn test_path_new_with_segments() {
        let p = Path::new_with_segments(&["metadata", "name"]);
        assert_eq!(p.to_string(), "metadata.name");
    }

    #[test]
    fn test_path_child() {
        let root = Path::new("metadata");
        let child = root.child("name");
        assert_eq!(child.to_string(), "metadata.name");
    }

    #[test]
    fn test_path_index() {
        let p = Path::new("items").index(0);
        assert_eq!(p.to_string(), "items[0]");
    }

    #[test]
    fn test_path_key() {
        let p = Path::new("annotations").key("app");
        assert_eq!(p.to_string(), "annotations[app]");
    }

    #[test]
    fn test_path_nested() {
        let p = Path::new("status")
            .child("conditions")
            .index(0)
            .child("message");
        assert_eq!(p.to_string(), "status.conditions[0].message");
    }

    #[test]
    fn test_path_default() {
        let p = Path::default();
        assert_eq!(p.to_string(), "<nil>");
    }
}
