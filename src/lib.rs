// Copyright (C) 2019 Robin Krahl <robin.krahl@ireas.org>
// SPDX-License-Identifier: MIT

#![warn(missing_docs, rust_2018_compatibility, rust_2018_idioms, unused)]

//! Displays dialog boxes using various backends.
//!
//! The `dialog` crate can be used to display different types of dialog boxes.  The supported types
//! are:
//! - [`Input`][]: a text input dialog
//! - [`Message`][]: a simple message box
//! - [`Password`][]: a password input dialog
//! - [`Question`][]: a question dialog box
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
//! Query a string from the user:
//!
//! ```no_run
//! use dialog::DialogBox;
//!
//! let name = dialog::Input::new("Please enter your name")
//!     .title("Name")
//!     .show()
//!     .expect("Could not display dialog box");
//! match name {
//!     Some(name) => println!("Hello {}!", name),
//!     None => println!("Hello stranger!"),
//! };
//! ```
//!
//! [`Dialog`]: backends/struct.Dialog.html
//! [`Input`]: struct.Input.html
//! [`Message`]: struct.Message.html
//! [`Password`]: struct.Password.html
//! [`Question`]: struct.Question.html
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

use std::io;
use std::process;
use std::result;
use std::str;
use std::string;

/// A result returned by `dialog`.
pub type Result<T> = result::Result<T, Error>;

/// An error returned by `dialog`.
#[derive(Debug)]
pub enum Error {
    /// A general error with an error message.
    Error(String),
    /// An input or output error.
    IoError(io::Error),
    /// An UTF-8 error.
    Utf8Error(str::Utf8Error),
}

impl From<&str> for Error {
    fn from(string: &str) -> Error {
        Error::Error(string.to_string())
    }
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Error {
        Error::IoError(error)
    }
}

impl From<str::Utf8Error> for Error {
    fn from(error: str::Utf8Error) -> Error {
        Error::Utf8Error(error)
    }
}

impl From<string::FromUtf8Error> for Error {
    fn from(error: string::FromUtf8Error) -> Error {
        Error::Utf8Error(error.utf8_error())
    }
}

impl From<(&str, process::ExitStatus)> for Error {
    fn from(data: (&str, process::ExitStatus)) -> Error {
        let (command, status) = data;
        let msg = match status.code() {
            Some(code) => format!("Command {} failed with exit status {}", command, code),
            None => format!("Command {} was terminated by a signal", command),
        };
        Error::Error(msg)
    }
}

/// A dialog box that can be shown using a backend.
///
/// Some dialog boxes might return data of the type `Output`.
pub trait DialogBox {
    /// The type of the data returned by the dialog box.
    type Output;

    /// Shows this dialog box using the default backend and returns the output.
    ///
    /// `box.show()` is a shorthand for `box.show_with(&default_backend())`.
    fn show(&self) -> Result<Self::Output> {
        self.show_with(&default_backend())
    }

    /// Shows this dialog box using the given backend and returns the output.
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

/// A dialog box with a text input field.
///
/// This dialog box displays a text and an input field.  It returns the text entered by the user or
/// `None` if the user cancelled the dialog.
///
/// # Example
///
/// ```no_run
/// use dialog::DialogBox;
///
/// let name = dialog::Input::new("Please enter your name")
///     .title("Name")
///     .show()
///     .expect("Could not display dialog box");
/// match name {
///     Some(name) => println!("Hello {}!", name),
///     None => println!("Hello stranger!"),
/// };
/// ```
pub struct Input {
    text: String,
    title: Option<String>,
    default: Option<String>,
}

impl Input {
    /// Creates a new input dialog box with the given text.
    pub fn new(text: impl Into<String>) -> Input {
        Input {
            text: text.into(),
            title: None,
            default: None,
        }
    }

    /// Sets the title of this input box.
    ///
    /// This method returns a reference to `self` to enable chaining.
    pub fn title(&mut self, title: impl Into<String>) -> &mut Input {
        self.title = Some(title.into());
        self
    }

    /// Sets the default value of this input box.
    ///
    /// This method returns a reference to `self` to enable chaining.
    pub fn default(&mut self, default: impl Into<String>) -> &mut Input {
        self.default = Some(default.into());
        self
    }
}

impl DialogBox for Input {
    type Output = Option<String>;

    fn show_with(&self, backend: &impl backends::Backend) -> Result<Self::Output> {
        backend.show_input(self)
    }
}

/// A dialog box with a password input field.
///
/// This dialog box displays a text and a password input field.  It returns the password entered by
/// the user or `None` if the user cancelled the dialog.
///
/// # Example
///
/// ```no_run
/// use dialog::DialogBox;
///
/// let password = dialog::Password::new("Please enter a new password")
///     .title("Password")
///     .show()
///     .expect("Could not display dialog box");
/// match password {
///     Some(password) => println!("Your new password is: {}", password),
///     None => println!("You do not want to have a password."),
/// };
/// ```
pub struct Password {
    text: String,
    title: Option<String>,
}

impl Password {
    /// Creates a new password dialog box with the given text.
    pub fn new(text: impl Into<String>) -> Password {
        Password {
            text: text.into(),
            title: None,
        }
    }

    /// Sets the title of this password dialog box.
    ///
    /// This method returns a reference to `self` to enable chaining.
    pub fn title(&mut self, title: impl Into<String>) -> &mut Password {
        self.title = Some(title.into());
        self
    }
}

impl DialogBox for Password {
    type Output = Option<String>;

    fn show_with(&self, backend: &impl backends::Backend) -> Result<Self::Output> {
        backend.show_password(self)
    }
}

/// A user choise in a dialog box.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Choice {
    /// The yes button.
    Yes,
    /// The no button.
    No,
    /// The cancel button or a cancelled dialog.
    Cancel,
}

/// A question dialog box.
///
/// This dialog box displays a text and an optional title and has a yes and a no button.  The
/// output is the button presed by the user, or Cancel if the dialog has been cancelled.
///
/// # Example
///
/// ```no_run
/// use dialog::DialogBox;
///
/// let choice = dialog::Question::new("Do you want to continue?")
///     .title("Question")
///     .show()
///     .expect("Could not display dialog box");
/// println!("The user chose: {:?}", choice);
/// ```
pub struct Question {
    text: String,
    title: Option<String>,
}

impl Question {
    /// Creates a new question dialog with the given text.
    pub fn new(text: impl Into<String>) -> Question {
        Question {
            text: text.into(),
            title: None,
        }
    }

    /// Sets the title of this question dialog box.
    ///
    /// This method returns a reference to `self` to enable chaining.
    pub fn title(&mut self, title: impl Into<String>) -> &mut Question {
        self.title = Some(title.into());
        self
    }
}

impl DialogBox for Question {
    type Output = Choice;

    fn show_with(&self, backend: &impl backends::Backend) -> Result<Self::Output> {
        backend.show_question(self)
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
