// Copyright (C) 2019 Robin Krahl <robin.krahl@ireas.org>
// SPDX-License-Identifier: MIT

use std::io::Result;

use dialog::backends;
use dialog::DialogBox;

#[test]
fn message() -> Result<()> {
    dialog::Message::new("This is a message.")
        .title("And this is a title:")
        .show_with(&backends::Dialog::new())
}

#[test]
fn backtitle() -> Result<()> {
    let mut backend = backends::Dialog::new();
    backend.set_backtitle("Backtitle");
    dialog::Message::new("This is a message.")
        .title("And this is a title:")
        .show_with(&backend)
}

#[test]
fn size() -> Result<()> {
    let mut backend = backends::Dialog::new();
    backend.set_width(100);
    backend.set_height(10);
    dialog::Message::new("This is a message.")
        .title("And this is a title:")
        .show_with(&backend)
}
