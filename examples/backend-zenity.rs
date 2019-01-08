// Copyright (C) 2019 Robin Krahl <robin.krahl@ireas.org>
// SPDX-License-Identifier: MIT

use dialog::backends;
use dialog::DialogBox;

fn main() -> dialog::Result<()> {
    let mut backend = backends::Zenity::new();

    dialog::Message::new("This is a message.")
        .title("And this is a title:")
        .show_with(&backend)?;

    backend.set_width(500);
    backend.set_height(200);
    dialog::Message::new("This is a message with a fixed size.")
        .title("And this is a title:")
        .show_with(&backend)?;

    let mut backend = backends::Zenity::new();
    backend.set_timeout(5);
    dialog::Message::new("This box should disappear after five seconds.")
        .title("And this is a title:")
        .show_with(&backend)
}
