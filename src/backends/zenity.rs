// Copyright (C) 2019 Robin Krahl <robin.krahl@ireas.org>
// SPDX-License-Identifier: MIT

use std::process;

use crate::{Choice, Error, Input, Message, Password, Question, Result};

/// The `zenity` backend.
///
/// This backend uses the external `zenity` program to display GTK+ dialog boxes.
#[derive(Debug)]
pub struct Zenity {
    icon: Option<String>,
    width: Option<String>,
    height: Option<String>,
    timeout: Option<String>,
}

impl Zenity {
    /// Creates a new `Zenity` instance without configuration.
    pub fn new() -> Zenity {
        Zenity {
            icon: None,
            width: None,
            height: None,
            timeout: None,
        }
    }

    /// Sets the icon of the dialog box.
    ///
    /// The icon can either be one of `error`, `info`, `question` or `warning, or the path to an
    /// image to use.  The default image depends on the dialog type.
    pub fn set_icon(&mut self, icon: impl Into<String>) {
        self.icon = Some(icon.into());
    }

    /// Sets the height of the dialog boxes.
    ///
    /// The height is given in pixels.  The actual height of the dialog box might be higher than
    /// the given height if the content would not fit otherwise.
    pub fn set_height(&mut self, height: u32) {
        self.height = Some(height.to_string());
    }

    /// Sets the width of the dialog boxes.
    ///
    /// The width is given in pixels.  The actual width of the dialog box might be higher than the
    /// given width if the content would not fit otherwise.
    pub fn set_width(&mut self, width: u32) {
        self.width = Some(width.to_string());
    }

    /// Sets the timout of the dialog boxes (in seconds).
    ///
    /// After the timeout, the dialog box is closed.  The timeout is handled like a cancel event.
    /// Per default, there is no timeout.
    pub fn set_timeout(&mut self, timeout: u32) {
        self.timeout = Some(timeout.to_string());
    }

    fn execute(&self, args: Vec<&str>, title: &Option<String>) -> Result<process::Output> {
        let mut command = process::Command::new("zenity");

        if let Some(ref icon) = self.icon {
            command.arg("--window-icon");
            command.arg(icon);
        }
        if let Some(ref width) = self.width {
            command.arg("--width");
            command.arg(width);
        }
        if let Some(ref height) = self.height {
            command.arg("--height");
            command.arg(height);
        }
        if let Some(ref timeout) = self.timeout {
            command.arg("--timeout");
            command.arg(timeout);
        }
        if let Some(ref title) = title {
            command.arg("--title");
            command.arg(title);
        }

        command.args(args);
        command.output().map_err(Error::IoError)
    }
}

fn require_success(status: process::ExitStatus) -> Result<()> {
    if status.success() {
        Ok(())
    } else {
        if let Some(code) = status.code() {
            match code {
                5 => Ok(()),
                _ => Err(Error::from(("zenity", status))),
            }
        } else {
            Err(Error::from(("zenity", status)))
        }
    }
}

fn get_choice(status: process::ExitStatus) -> Result<Choice> {
    if let Some(code) = status.code() {
        match code {
            0 => Ok(Choice::Yes),
            1 => Ok(Choice::No),
            5 => Ok(Choice::Cancel),
            _ => Err(Error::from(("zenity", status))),
        }
    } else {
        Err(Error::from(("zenity", status)))
    }
}

fn get_stdout(output: process::Output) -> Result<Option<String>> {
    if output.status.success() {
        String::from_utf8(output.stderr)
            .map(|s| Some(s))
            .map_err(|err| Error::from(err))
    } else {
        if let Some(code) = output.status.code() {
            match code {
                0 => Ok(None),
                1 => Ok(None),
                5 => Ok(None),
                _ => Err(Error::from(("zenity", output.status))),
            }
        } else {
            Err(Error::from(("zenity", output.status)))
        }
    }
}

impl super::Backend for Zenity {
    fn show_input(&self, input: &Input) -> Result<Option<String>> {
        let mut args = vec!["--entry", "--text", &input.text];
        if let Some(ref default) = input.default {
            args.push("--entry-text");
            args.push(default);
        }
        self.execute(args, &input.title).and_then(get_stdout)
    }

    fn show_message(&self, message: &Message) -> Result<()> {
        let args = vec!["--info", "--text", &message.text];
        self.execute(args, &message.title)
            .and_then(|output| require_success(output.status))
            .map(|_| ())
    }

    fn show_password(&self, password: &Password) -> Result<Option<String>> {
        let args = vec!["--password"];
        self.execute(args, &password.title).and_then(get_stdout)
    }

    fn show_question(&self, question: &Question) -> Result<Choice> {
        let args = vec!["--question", "--text", &question.text];
        self.execute(args, &question.title)
            .and_then(|output| get_choice(output.status))
    }
}
