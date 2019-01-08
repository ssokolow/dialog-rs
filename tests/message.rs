// Copyright (C) 2019 Robin Krahl <robin.krahl@ireas.org>
// SPDX-License-Identifier: MIT

use std::io::Result;

use dialog::DialogBox;

#[test]
fn text() -> Result<()> {
    dialog::Message::new("This is a message.").show()
}

#[test]
fn text_title() -> Result<()> {
    dialog::Message::new("This is a message.")
        .title("And this is a title:")
        .show()
}
