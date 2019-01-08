// Copyright (C) 2019 Robin Krahl <robin.krahl@ireas.org>
// SPDX-License-Identifier: MIT

use std::io;
use std::io::Result;
use std::process;

use crate::Message;

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

    fn execute(&self, args: Vec<&str>) -> Result<process::Output> {
        let mut args = args;
        if let Some(ref backtitle) = self.backtitle {
            args.insert(0, "--backtitle");
            args.insert(1, backtitle);
        }
        println!("{:?}", args);
        process::Command::new("dialog")
            .args(args)
            .stdin(process::Stdio::inherit())
            .stdout(process::Stdio::inherit())
            .output()
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

    fn show_box(&self, args: Vec<&str>, title: &Option<String>) -> Result<process::Output> {
        let mut args = args;
        if let Some(ref title) = title {
            args.insert(0, "--title");
            args.insert(1, title);
        }
        args.push(&self.height);
        args.push(&self.width);
        self.execute(args)
    }
}

fn require_success(status: process::ExitStatus) -> Result<()> {
    if status.success() {
        Ok(())
    } else {
        Err(io::Error::new(io::ErrorKind::Other, "dialog failed"))
    }
}

impl super::Backend for Dialog {
    fn show_message(&self, message: &Message) -> Result<()> {
        let args = vec!["--msgbox", &message.text];
        self.show_box(args, &message.title)
            .and_then(|output| require_success(output.status))
            .map(|_| ())
    }
}
