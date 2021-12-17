# Changelog

This project follows [semantic versioning](https://semver.org/).

## [Unreleased]

 * changed: Depend on lin-bus crate 0.4
 * breaking: Minimal required Rust version changed to 1.57.0
 * fixed: Always write data completly
   ([#14](https://github.com/Sensirion/lin-bus-driver-serial-rs/pull/14))

## [0.3.0] (2019-07-16)

 * changed: Depend on lin-bus crate 0.3

## [0.2.0] (2019-05-03)

 * changed: Use Rust 2018 edition syntax
   ([#7](https://github.com/Sensirion/lin-bus-driver-serial-rs/pull/7))
 * fixed: Allow sending a frame with the maximal permissible payload (8b)
 * changed: Depend on lin-bus 0.2

## [0.1.1] (2018-12-19)

 * changed: Properly translate serial errors to lin bus errors
   ([#6](https://github.com/Sensirion/lin-bus-driver-serial-rs/pull/6))

## 0.1.0 (2018-06-25)

 * First crates.io release

[Unreleased]: https://github.com/Sensirion/lin-bus-driver-serial-rs/compare/v0.3.0...HEAD
[0.3.0]: https://github.com/Sensirion/lin-bus-driver-serial-rs/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/Sensirion/lin-bus-driver-serial-rs/compare/v0.1.1...v0.2.0
[0.1.1]: https://github.com/Sensirion/lin-bus-driver-serial-rs/compare/v0.1.0...v0.1.1
