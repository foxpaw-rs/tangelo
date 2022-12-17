//! Todo(Paul): Crate Documentation once 0.1 complete.
//! * Include a note about the crate is target OS specific
//!
//! # Usage
//!
//! # Examples
//!
//! For further detailed examples, refer to the documentation.

#![deny(missing_docs)]
#![deny(clippy::missing_docs_in_private_items)]

/// Tangelo.
///
/// The Tangelo struct that is used to interact with the terminal.
#[derive(Debug, Eq, PartialEq)]
pub struct Tangelo {}

impl Tangelo {
    /// Get the current Tangelo.
    ///
    /// Create a new Tangelo object for the current terminal. Will return None if
    /// the currently active window is not a terminal.
    ///
    /// # Examples
    /// ```rust
    /// use tangelo::Tangelo;
    ///
    /// let tangelo = Tangelo::current().unwrap();
    /// ```
    pub fn current() -> Option<Self> {
        Some(Tangelo {})
    }

    /// Determine the terminal height.
    ///
    /// Retrieve the termnal height.
    ///
    /// # Examples
    /// ```rust
    /// use tangelo::Tangelo;
    ///
    ///
    /// let tangelo = Tangelo::current().unwrap();
    /// let height = tangelo.get_height();
    /// ```
    pub fn get_height(&self) -> u16 {
        0
    }

    /// Determine the terminal size.
    ///
    /// Retrieve the termnal size.
    ///
    /// # Examples
    /// ```rust
    /// use tangelo::Tangelo;
    ///
    /// let tangelo = Tangelo::current().unwrap();
    /// let (height, width) = tangelo.get_size();
    /// ```
    pub fn get_size(&self) -> (u16, u16) {
        (0, 0)
    }

    /// Determine the terminal width.
    ///
    /// Retrieve the termnal width.
    ///
    /// # Examples
    /// ```rust
    /// use tangelo::Tangelo;
    ///
    /// let tangelo = Tangelo::current().unwrap();
    /// let width = tangelo.get_width();
    /// ```
    pub fn get_width(&self) -> u16 {
        0
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    /// Tangelo::current must create as per struct initialisation.
    ///
    /// The current method on Tangelo must create an object as per the struct
    /// initialiser syntax.
    #[test]
    fn tangelo_current() {
        let expected = Tangelo {};
        let actual = Tangelo::current().unwrap();
        assert_eq!(expected, actual);
    }
}
