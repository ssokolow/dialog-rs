// Copyright (C) 2019 Robin Krahl <robin.krahl@ireas.org>
// SPDX-License-Identifier: MIT

#![warn(missing_docs, rust_2018_compatibility, rust_2018_idioms, unused)]

//! Displays dialog boxes using various backends.
//!
//! The `dialog` crate can be used to display different types of dialog boxes.  The supported types
//! are:
//! - [`Message`][]: a simple message box
//!
//! These dialog boxes can be displayed using various backends:
//! - [`Dialog`][]: uses `dialog` to display ncurses-based dialog boxes (requires the external
//!   `dialog` tool)
//!
//! You can let `dialog` choose the backend by calling the [`show`][] method on a dialog box.  If
//! you want to choose the backend yourself, create a backend instance and pass it to
//! [`show_with`][].  You can also use the [`default_backend`][] function to create a backend.
//!
//! # Examples
//!
//! Show a message box using the default backend:
//!
//! ```no_run
//! use dialog::DialogBox;
//!
//! dialog::Message::new("Did you know that I am using the dialog crate?")
//!     .title("Public Service Announcement")
//!     .show()
//!     .expect("Could not display dialog box");
//! ```
//!
//! Show a message box using the [`Dialog`][] backend with customized settings:
//!
//! ```no_run
//! use dialog::DialogBox;
//!
//! let mut backend = dialog::backends::Dialog::new();
//! backend.set_backtitle("dialog demo");
//! backend.set_width(100);
//! backend.set_height(10);
//! dialog::Message::new("Did you know that I am using the dialog crate?")
//!     .title("Public Service Announcement")
//!     .show_with(&backend)
//!     .expect("Could not display dialog box");
//! ```
//!
//! [`Message`]: struct.Message.html
//! [`Dialog`]: backends/struct.Dialog.html
//! [`default_backend`]: fn.default_backend.html
//! [`show`]: trait.DialogBox.html#method.show
//! [`show_with`]: trait.DialogBox.html#method.show_with

/// Backends that display dialog boxes.
///
/// All backends implement the [`Backend`][] trait.  Some backends might provide additional
/// settings.  For a list of supported backends, see the [top-level crate documentation](./..) or
/// the [list of structs in this module](#structs).
///
/// [`Backend`]: trait.Backend.html
pub mod backends;

use std::io::Result;

/// A dialog box that can be shown using a backend.
///
/// Some dialog boxes might return data of the type `Output`.
pub trait DialogBox {
    /// The type of the data returned by the dialog box.
    type Output;

    /// Shows this dialog box using the default backend.
    ///
    /// `box.show()` is a shorthand for `box.show_with(&default_backend())`.
    fn show(&self) -> Result<Self::Output> {
        self.show_with(&default_backend())
    }

    /// Shows this dialog box using the given backend.
    fn show_with(&self, backend: &impl backends::Backend) -> Result<Self::Output>;
}

/// A message box.
///
/// This dialog box displays a text and an optional title and has a single OK button.  It does not
/// produce any output.
///
/// # Example
///
/// ```no_run
/// use dialog::DialogBox;
///
/// dialog::Message::new("The operation was successful.")
///     .title("Success")
///     .show()
///     .expect("Could not display dialog box");
/// ```
pub struct Message {
    text: String,
    title: Option<String>,
}

impl Message {
    /// Creates a new message box with the given text.
    pub fn new(text: impl Into<String>) -> Message {
        Message {
            text: text.into(),
            title: None,
        }
    }

    /// Sets the title of this message box.
    ///
    /// This method returns a reference to `self` to enable chaining.
    pub fn title(&mut self, title: impl Into<String>) -> &mut Message {
        self.title = Some(title.into());
        self
    }
}

impl DialogBox for Message {
    type Output = ();

    fn show_with(&self, backend: &impl backends::Backend) -> Result<Self::Output> {
        backend.show_message(self)
    }
}

/// Creates a new instance of the default backend.
///
/// The current implementation always returns a [`Dialog`][] instance.
///
/// [`Dialog`]: backends/struct.Dialog.html
pub fn default_backend() -> impl backends::Backend {
    backends::Dialog::new()
}
