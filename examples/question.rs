// Copyright (C) 2019 Robin Krahl <robin.krahl@ireas.org>
// SPDX-License-Identifier: MIT

use std::io::Result;

use dialog::DialogBox;

fn main() -> Result<()> {
    let choice = dialog::Question::new("Do you want to continue?").show()?;
    println!("The user chose: {:?}", choice);
    Ok(())
}
