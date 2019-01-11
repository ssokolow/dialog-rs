# Unreleased
- Refactor `default_backend` to return a `Box<dyn Backend>`.
- Check the `DIALOG` and `DISPLAY` environment variables in `default_backend`.

# v0.1.1 (2019-01-11)
- Add the `Password` dialog box.
- Add the `Zenity` backend.
- Implement `std::fmt::Display` for the `Error` enum.

# v0.1.0 (2019-01-08)
- Initial release with the `Input`, `Message` and `Question` dialog boxes and
  the `Dialog` backend.
