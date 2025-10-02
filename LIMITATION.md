# Limitations

Version: 0.0.2

## Option<String> with empty_str_option_not_none

- Behavior: When this flag is enabled, an empty placeholder for `Option<String>` is parsed as `Some("")`, even if the source was `None`. Round-trips (struct → template → struct) are converted from `None` to `Some("")`.
- Scope: This mainly concerns string-like `Option<T>`; other types aren’t affected in the same way.
- Why: The flag treats an empty string as a valid, intentional value to keep templates user-friendly.
- Workarounds: 
  - Use a small sentinel for `None` (e.g., `<none>`) if you need to preserve the distinction, or
  - Avoid enabling the flag where `None` vs `Some("")` must be retained.
- Status: Known trade-off in 0.0.2. We’ll consider opt-in refinements in future versions.
