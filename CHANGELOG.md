# Git HEAD
- Add the `KDialog` backend. (contributed by Stephan Sokolow)

# v0.2.1 (2019-06-30)
- Fix the input and password dialogs for the `zenity` backend (thanks Silvano
  Cortesi for the bug report):
  - Read from `stdout` instead of `stderr`.
  - Remove trailing newlines from the result.

# v0.2.0 (2019-01-11)
- Refactor `default_backend` to return a `Box<dyn Backend>`.
- Check the `DIALOG` and `DISPLAY` environment variables in `default_backend`.
- Add the `Stdio` backend.

# v0.1.1 (2019-01-11)
- Add the `Password` dialog box.
- Add the `Zenity` backend.
- Implement `std::fmt::Display` for the `Error` enum.

# v0.1.0 (2019-01-08)
- Initial release with the `Input`, `Message` and `Question` dialog boxes and
  the `Dialog` backend.
