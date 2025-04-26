# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to
[Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.2] - 2025.03.11

- Implement `std::io::Error` for `BodyReaderError` manually, because `thiserror`
  crate does implement `std::error::Error::source` method, which breaks the
  error boxing.

## [0.1.1] - 2025.03.10

- Improvements in documentation and readme.

## [0.1.0] - 2025.03.09

- Initial release
