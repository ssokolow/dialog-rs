// Copyright (C) 2019 Robin Krahl <robin.krahl@ireas.org>
// SPDX-License-Identifier: MIT

mod dialog;

pub use crate::backends::dialog::Dialog;

use crate::Result;

/// A dialog backend.
///
/// A dialog backend is a program that can be used to display dialog boxes.  Use the
/// [`default_backend`][] function to create a new instance of the default backend, or choose a
/// backend and create an instance manually.  To use a backend, pass it to the [`show_with`][]
/// method of a dialog box.
///
/// [`default_backend`]: ../function.default_backend.html
/// [`show_with`]: ../trait.DialogBox.html#method.show_with
pub trait Backend {
    /// Shows the given input dialog and returns the input.
    fn show_input(&self, input: &super::Input) -> Result<Option<String>>;

    /// Shows the given message dialog.
    fn show_message(&self, message: &super::Message) -> Result<()>;

    /// Shows the given question dialog and returns the choice.
    fn show_question(&self, question: &super::Question) -> Result<super::Choice>;
}
