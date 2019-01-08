// Copyright (C) 2019 Robin Krahl <robin.krahl@ireas.org>
// SPDX-License-Identifier: MIT

use std::io;
use std::io::Result;
use std::process;

use crate::{Input, Message};

/// The `dialog` backend.
///
/// This backend uses the external `dialog` program (not to be confused with this crate also called
/// `dialog`) to display text-based dialog boxes in the terminal.
#[derive(Debug)]
pub struct Dialog {
    backtitle: Option<String>,
    width: String,
    height: String,
}

impl Dialog {
    /// Creates a new `Dialog` instance without configuration.
    pub fn new() -> Dialog {
        Dialog {
            backtitle: None,
            height: "0".to_string(),
            width: "0".to_string(),
        }
    }

    /// Sets the backtitle for the dialog boxes.
    ///
    /// The backtitle is displayed on the backdrop, at the top of the screen.
    pub fn set_backtitle(&mut self, backtitle: impl Into<String>) {
        self.backtitle = Some(backtitle.into());
    }

    /// Sets the height of the dialog boxes.
    ///
    /// The height is given in characters.  The actual height of the dialog box might be higher
    /// than the given height if the content would not fit otherwise.  The default height is zero.
    pub fn set_height(&mut self, height: u32) {
        self.height = height.to_string();
    }

    /// Sets the width of the dialog boxes.
    ///
    /// The width is given in characters.  The actual width of the dialog box might be higher than
    /// the given width if the content would not fit otherwise.  The default width is zero.
    pub fn set_width(&mut self, width: u32) {
        self.width = width.to_string();
    }

    fn execute(
        &self,
        args: Vec<&str>,
        post_args: Vec<&str>,
        title: &Option<String>,
    ) -> Result<process::Output> {
        let mut command = process::Command::new("dialog");
        command.stdin(process::Stdio::inherit());
        command.stdout(process::Stdio::inherit());

        if let Some(ref backtitle) = self.backtitle {
            command.arg("--backtitle");
            command.arg(backtitle);
        }
        if let Some(ref title) = title {
            command.arg("--title");
            command.arg(title);
        }

        command.args(args);
        command.arg(&self.height);
        command.arg(&self.width);
        command.args(post_args);

        command.output()
    }
}

fn require_success(status: process::ExitStatus) -> Result<()> {
    if status.success() {
        Ok(())
    } else {
        Err(io::Error::new(io::ErrorKind::Other, "dialog failed"))
    }
}

fn get_stderr(output: process::Output) -> Result<Option<String>> {
    if output.status.success() {
        String::from_utf8(output.stderr)
            .map(|s| Some(s))
            .map_err(|_| {
                io::Error::new(io::ErrorKind::Other, "Input contained invalid UTF-8 bytes")
            })
    } else {
        if let Some(code) = output.status.code() {
            match code {
                0 => Ok(None),
                1 => Ok(None),
                -1 => Ok(None),
                _ => Err(io::Error::new(
                    io::ErrorKind::Other,
                    "Could not execute dialog",
                )),
            }
        } else {
            Err(io::Error::new(
                io::ErrorKind::Other,
                "dialog was terminated by a signal",
            ))
        }
    }
}

impl super::Backend for Dialog {
    fn show_input(&self, input: &Input) -> Result<Option<String>> {
        let args = vec!["--inputbox", &input.text];
        let mut post_args: Vec<&str> = Vec::new();
        if let Some(ref default) = input.default {
            post_args.push(default);
        }
        self.execute(args, post_args, &input.title)
            .and_then(get_stderr)
    }

    fn show_message(&self, message: &Message) -> Result<()> {
        let args = vec!["--msgbox", &message.text];
        self.execute(args, vec![], &message.title)
            .and_then(|output| require_success(output.status))
            .map(|_| ())
    }
}
