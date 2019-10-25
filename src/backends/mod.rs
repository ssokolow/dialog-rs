// Copyright (C) 2019 Robin Krahl <robin.krahl@ireas.org>
// SPDX-License-Identifier: MIT

mod dialog;
mod stdio;
mod zenity;
mod kdialog;

pub use crate::backends::dialog::Dialog;
pub use crate::backends::stdio::Stdio;
pub use crate::backends::zenity::Zenity;
pub use crate::backends::kdialog::KDialog;

use std::env;
use std::path;

use crate::Result;

/// A dialog backend.
///
/// A dialog backend is a program that can be used to display dialog boxes.  Use the
/// [`default_backend`][] function to create a new instance of the default backend, or choose a
/// backend and create an instance manually.  To use a backend, pass it to the [`show_with`][]
/// method of a dialog box.
///
/// [`default_backend`]: ../fn.default_backend.html
/// [`show_with`]: ../trait.DialogBox.html#method.show_with
pub trait Backend {
    /// Shows the given input dialog and returns the input.
    fn show_input(&self, input: &super::Input) -> Result<Option<String>>;

    /// Shows the given message dialog.
    fn show_message(&self, message: &super::Message) -> Result<()>;

    /// Shows the given password dialog and returns the password.
    fn show_password(&self, password: &super::Password) -> Result<Option<String>>;

    /// Shows the given question dialog and returns the choice.
    fn show_question(&self, question: &super::Question) -> Result<super::Choice>;
}

pub(crate) fn is_available(name: &str) -> bool {
    if let Ok(path) = env::var("PATH") {
        for part in path.split(':') {
            if path::Path::new(part).join(name).exists() {
                return true;
            }
        }
    }
    false
}

pub(crate) fn from_str(s: &str) -> Option<Box<dyn Backend>> {
    match s.to_lowercase().as_ref() {
        "dialog" => Some(Box::new(Dialog::new())),
        "kdialog" => Some(Box::new(KDialog::new())),
        "stdio" => Some(Box::new(Stdio::new())),
        "zenity" => Some(Box::new(Zenity::new())),
        _ => None,
    }
}
