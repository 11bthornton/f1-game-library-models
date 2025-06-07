# Contributing

## Code Review
- The `main` branch is the release branch.
- Make pull requests into the `develop` branch.

## Guidelines

All `enums` **must**:

- Derive `deserialize_repr` and `serialize_repr` as opposed to `deserialize` and `serialize`.
- Be annotated with `#[repr(T)]` where `T` is the type of the enum as written in the UDP specification - e.g. `u8`.