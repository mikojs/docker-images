<!-- next-header -->

## [Unreleased] - ReleaseDate

### Added
- Use the release command with `cargo-release`.
- Add `CHANGELOG.md`.
- Use resuable `build-docker` workflow from [@mikojs/actions](https://github.com/mikojs/actions).
- Add `entrypoint` command.
- Use `entrypoint` command in the all Docker images.

## Changed
- Should exit process with `sub_process::exec`.

### Fixed
- Should ignore `release` command in `.dockerignore`.

<!-- next-url -->
[Unreleased]: https://github.com/mikojs/docker-images/compare/v1.0.0...HEAD
