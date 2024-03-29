// Copyright (C) 2019 Robin Krahl <robin.krahl@ireas.org>
// SPDX-License-Identifier: MIT

use dialog::backends;
use dialog::DialogBox;

fn main() -> dialog::Result<()> {
    let backend = backends::Stdio::new();

    dialog::Message::new("This is a message.")
        .title("And this is a title:")
        .show_with(&backend)
}
