// Copyright (C) 2019 Robin Krahl <robin.krahl@ireas.org>
// Copyright (C) 2019 Stephan Sokolow <http://www.ssokolow.com/ContactMe>
// SPDX-License-Identifier: MIT

use std::process;

use crate::{Choice, Error, Input, Message, Password, Question, Result};

/// Subprocess exit codes
///
/// (Defined once here to avoid typo-related bugs or confusion)
///
/// NOTE: `kdialog` doesn't have a fixed correspondence between button labels and status codes.
/// The following mappings occur:
///
/// - Yes/No = `0`/`1`
/// - Yes/No/Cancel = `0`/`1`/`2`
/// - OK/Cancel = `0`/`1`
const OK:     i32 = 0;
const CANCEL: i32 = 1;

/// The `kdialog` backend.
///
/// This backend uses the external `kdialog` program to display KDE dialog boxes.
#[derive(Debug)]
pub struct KDialog {
    icon: Option<String>,
    // TODO: --dontagain
}

impl KDialog {
    /// Creates a new `KDialog` instance without configuration.
    pub fn new() -> KDialog {
        KDialog {
            icon: None,
        }
    }

    /// Sets the icon in the dialog box's titlebar and taskbar button.
    ///
    /// The icon can be either a name from the user's configured icon theme, such as `error` or
    /// `info` or the path to an image to use.
    ///
    /// The default image depends on the dialog type.
    pub fn set_icon(&mut self, icon: impl Into<String>) {
        self.icon = Some(icon.into());
    }

    pub(crate) fn is_available() -> bool {
        super::is_available("kdialog")
    }

    fn execute(&self, args: Vec<&str>, title: &Option<String>) -> Result<process::Output> {
        let mut command = process::Command::new("kdialog");

        if let Some(ref icon) = self.icon {
            command.arg("--icon");
            command.arg(icon);
        }
        if let Some(ref title) = title {
            command.arg("--title");
            command.arg(title);
        }

        command.args(args);
        command.output().map_err(Error::IoError)
    }
}

impl AsRef<KDialog> for KDialog {
    fn as_ref(&self) -> &Self {
        self
    }
}

fn require_success(status: process::ExitStatus) -> Result<()> {
    if status.success() {
        Ok(())
    } else if let Some(code) = status.code() {
        match code {
            CANCEL => Ok(()),
            _ => Err(Error::from(("kdialog", status))),
        }
    } else {
        Err(Error::from(("kdialog", status)))
    }
}

fn get_choice(status: process::ExitStatus) -> Result<Choice> {
    if let Some(code) = status.code() {
        match code {
            OK => Ok(Choice::Yes),
            CANCEL => Ok(Choice::No),
            _ => Err(Error::from(("kdialog", status))),
        }
    } else {
        Err(Error::from(("kdialog", status)))
    }
}

fn get_stdout(output: process::Output) -> Result<Option<String>> {
    if output.status.success() {
        String::from_utf8(output.stdout)
            .map(|s| Some(s.trim_end_matches('\n').to_string()))
            .map_err(Error::from)
    } else if let Some(code) = output.status.code() {
        match code {
            OK => Ok(None),
            CANCEL => Ok(None),
            _ => Err(Error::from(("kdialog", output.status))),
        }
    } else {
        Err(Error::from(("kdialog", output.status)))
    }
}

impl super::Backend for KDialog {
    fn show_input(&self, input: &Input) -> Result<Option<String>> {
        let mut args = vec!["--inputbox", &input.text];
        if let Some(ref default) = input.default {
            args.push(default);
        }
        self.execute(args, &input.title).and_then(get_stdout)
    }

    fn show_message(&self, message: &Message) -> Result<()> {
        let args = vec!["--msgbox", &message.text];
        self.execute(args, &message.title)
            .and_then(|output| require_success(output.status))
            .map(|_| ())
    }

    fn show_password(&self, password: &Password) -> Result<Option<String>> {
        let args = vec!["--password", &password.text];
        self.execute(args, &password.title).and_then(get_stdout)
    }

    fn show_question(&self, question: &Question) -> Result<Choice> {
        let args = vec!["--yesno", &question.text];
        self.execute(args, &question.title)
            .and_then(|output| get_choice(output.status))
    }
}
