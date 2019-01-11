# dialog-rs

A Rust library for displaying dialog boxes using various backends.

[Documentation][]

Currently `dialog-rs` supports input, message, password and question dialogs.
It can use the `dialog` or `zenity` tools to display the dialog boxes.  If none
of these tools is available, the dialogs are printed to the standard output.

## Example

```rust
use dialog::DialogBox;

let choice = dialog::Question::new("Would you like to install Rust?")
    .title("Rust Installation")
    .show()
    .expect("Could not display dialog box");
if choice == dialog::Choice::Yes {
    dialog::Message::new("You made the right choice!")
        .title("Rust Installation")
        .show()
        .expect("Could not display dialog box");
}
```

## Contact

For bug reports, patches, feature requests or other messages, please send a
mail to [dialog-rs-dev@ireas.org][].

## License

This project is licensed under the [MIT License][].

[Documentation]: https://docs.rs/dialog
[dialog-rs-dev@ireas.org]: mailto:dialog-rs-dev@ireas.org
[MIT license]: https://opensource.org/licenses/MIT
