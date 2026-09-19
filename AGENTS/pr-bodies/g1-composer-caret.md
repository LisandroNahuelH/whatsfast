## Summary

- Mirror the viewport focus flag into `RawInput.focused` in `Shell::raw_input_hook` so the composer caret paints under native eframe backends.
- Add a demo tour unit test that asserts the caret appears with focus and hides without it.

## Test plan

- [x] `cargo test --locked the_composer_shows_its_caret_while_the_window_has_focus --features demo` (Windows)
- [ ] Full CI on GitHub

## Notes

- Independent of other pending contributions from the same fork.
