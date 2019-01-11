// Copyright (C) 2019 Robin Krahl <robin.krahl@ireas.org>
// SPDX-License-Identifier: MIT

use std::io::{self, Write};

use crate::{Choice, Input, Message, Password, Question, Result};

/// The fallback backend using standard input and output.
///
/// This backend is intended as a fallback backend to use if no other backend is available.  The
/// dialogs are printed to the standard output and user input is read from the standard input.
#[derive(Debug)]
pub struct Stdio {}

impl Stdio {
    /// Creates a new `Stdio` instance.
    pub fn new() -> Stdio {
        Stdio {}
    }
}

impl AsRef<Stdio> for Stdio {
    fn as_ref(&self) -> &Self {
        self
    }
}

fn print_title(title: &Option<String>) {
    if let Some(ref title) = title {
        println!("{}", title);
        println!("{}", "=".repeat(title.len()));
    }
}

fn read_input() -> Result<String> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim_end_matches("\n").to_string())
}

fn parse_choice(input: &str) -> Choice {
    match input.to_lowercase().as_ref() {
        "y" => Choice::Yes,
        "yes" => Choice::Yes,
        "n" => Choice::No,
        "no" => Choice::No,
        _ => Choice::Cancel,
    }
}

impl super::Backend for Stdio {
    fn show_input(&self, input: &Input) -> Result<Option<String>> {
        print_title(&input.title);
        if let Some(ref default) = input.default {
            print!("{} [default: {}]: ", input.text, default);
        } else {
            print!("{}: ", input.text);
        }
        io::stdout().flush()?;

        let user_input = read_input()?;
        if user_input.is_empty() {
            if let Some(ref default) = input.default {
                return Ok(Some(default.to_string()));
            }
        }
        Ok(Some(user_input))
    }

    fn show_message(&self, message: &Message) -> Result<()> {
        print_title(&message.title);
        println!("{}", message.text);
        Ok(())
    }

    fn show_password(&self, password: &Password) -> Result<Option<String>> {
        print_title(&password.title);
        print!("{}: ", password.text);
        io::stdout().flush()?;
        Ok(Some(rpassword::read_password()?))
    }

    fn show_question(&self, question: &Question) -> Result<Choice> {
        print_title(&question.title);
        print!("{} [y/n]: ", question.text);
        io::stdout().flush()?;
        Ok(parse_choice(&read_input()?))
    }
}
