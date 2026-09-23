# Changelog

All notable changes to `cryptotokenkit-rs` are documented here.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.4.0] - Unreleased

### Security

- `TlvRecord::parse`, `parse_with_encoding` and the sequence parsers no longer
  panic on card-supplied lengths. An overflowing BER length field, a BER tag
  longer than 8 bytes and truncated input now return `None`.
- Card data starting with a `0x00` byte could abort the process: the parser
  passed BER tag 0 to CryptoTokenKit, which raises an Objective-C exception.
  `TlvRecord::ber`, `ber_tag_data`, `ber_constructed`, `simple` and `compact`
  now return `None` for inputs the framework rejects with an exception (BER
  tag 0, compact tags above 15 or values over 15 bytes, simple values over
  65535 bytes, and such records as constructed children).
- Dropping a token-session, token, token-driver, smart-card token-driver or
  smart-card user-interaction delegate handle, a `SlotStateObserver` or a
  `TokenWatcher`, or replacing the watcher's insertion handler, freed Rust
  state that a callback in flight could still read. The Swift side now owns a
  `CallbackContext` reference, and late callbacks no longer reach the dropped
  delegate.
- `SmartCardSlotManager::get_slot_with_name` wrote its reply into the caller's
  stack even after the 30 s timeout had returned to Rust.
- PINs and passwords no longer pass through the bridge's JSON snapshots or
  NUL-terminated copies. `pin()` and `password()` return `Zeroizing<String>`,
  and the buffer the Swift bridge hands to Rust is wiped before it is freed.
- A framework call that failed without an `NSError` (for example
  `TokenSmartCardPinAuthOperation::finish` without a smart card) was reported
  as success. It is now `CryptoTokenKitError::FrameworkError`.

### Fixed

- Bridge statuses no longer share values with `TKError` codes. A card's
  `CorruptedData` error was reported as `TimedOut`, `NotImplemented` as
  `InvalidArgument` and `CommunicationError` as `FrameworkError`, and
  `framework_code()` returned an SDK code for bridge failures. Only
  `TKErrorDomain` codes are passed through, and errors returned by Rust
  delegates reach CryptoTokenKit as `TKError` codes.
- The Swift bridge trapped on `NSError` codes outside the `Int32` range.
- `SmartCard::send_ins` rejects `le` above 65536 instead of passing a negative
  value to the framework.
- `SmartCard::with_session` ends the session when the callback panics.
- A smart-card session that begins after the 30 s timeout is ended again, a
  timed-out `SmartCardUserInteraction::run` cancels the interaction, and `run`
  waits for the interaction's configured timeouts (up to an hour).
- Sign, decrypt, key-exchange and create-token delegate callbacks accept empty
  data instead of rejecting it as a missing argument.
- Retained framework objects passed to a delegate trampoline are released when
  the trampoline returns early.
- `TokenDriver::add_token_configuration`, `remove_token_configuration` and
  `Token::set_configuration_data` stored their values in process-local
  dictionaries (the configuration data keyed by object address, so a later
  token could read another token's data). They now use the framework.

### Changed

- **Breaking:** `TokenPasswordAuthOperation::password` and
  `TokenSmartCardPinAuthOperation::pin` return
  `Result<Option<Zeroizing<String>>, CryptoTokenKitError>`.
- **Breaking:** `TokenDriver::add_token_configuration`,
  `remove_token_configuration` and `Token::set_configuration_data` return
  `CryptoTokenKitError::Unsupported` outside the app that contains the token
  extension, and `TokenDriver::driver_configurations` only lists the
  configurations CryptoTokenKit reports.
- **Breaking:** `CryptoTokenKitError::code()` returns -1001 to -1004 for the
  bridge variants (`InvalidArgument`, `FrameworkError`, `TimedOut`,
  `Unsupported`) instead of values that collided with `TKErrorCode`.
- **Breaking:** `TKErrorCode` is `#[non_exhaustive]`.
- `doom-fish-utils` is now a regular dependency (`>=0.4.1, <0.5`); the `async`
  feature only enables its `futures-stream` support.
- New dependency `zeroize` (`>=1.6, <1.9`; 1.9 needs Rust 1.85).
- `rust-version` is now 1.82.

### Added

- `TKErrorCode::InvalidatedDeviceKey` (`-10`, SDK 27.0).
- `CryptoTokenKitError::Unsupported`.

## [0.3.1] - 2026-06-06

### Fixed

- Guarded the CryptoTokenKit trampolines against panics crossing the FFI
  boundary, validated the AID slice passed to smart-card token-driver
  delegates, and removed the vestigial Swift bridge C header.

## [0.3.0] - 2026-05-20

### Added

- `async_api` module behind the `async` feature, providing executor-agnostic async wrappers for `TKTokenWatcher` insertion and removal callbacks. Uses `doom-fish-utils::stream`.

## [0.2.3] - 2026-05-18

### Changed

- Added Rustdoc coverage across the public safe API, with `///` comments that point back to the corresponding CryptoTokenKit framework types and delegate entry points. The safe surface now reports 100.0% Rustdoc coverage in `cargo +nightly rustdoc --lib --all-features -- -Z unstable-options --show-coverage`.

## [0.2.2] - 2025-01-10

### Changed

- Added SAFETY comments to all critical unsafe blocks in callback implementations and memory management to improve code clarity and auditability. This includes clarifications for C string conversion, pointer dereferencing in delegate callbacks, and object lifetime management in Drop implementations.

## [0.2.1] - 2026-05-16

### Added

- `TK_ERROR_DOMAIN`, `TKErrorCode`, and `CryptoTokenKitError::framework_code()` for first-class framework error inspection.
- BER TLV tag/constructed-record helpers plus pure-Rust `TlvRecord::parse*` fallbacks for the framework parsers that throw on macOS 26.2.
- Token/session/driver delegate bridges, `TokenSession::token`, `Token::token_driver`, `TokenKeyAlgorithm`, `TokenKeyExchangeParameters`, and token-driver configuration add/remove helpers.
- Secure smart-card user-interaction wrappers, delegate callbacks, `SmartCard::slot`, and `SmartCard::mock` for hardware-free testing.
- Examples `09_token_delegate_bridges` and `10_smart_card_user_interactions`, plus integration tests for delegates, TLV/error helpers, and secure PIN interactions.

### Changed

- Updated the coverage audit to 100% verified coverage for the audited macOS-public CryptoTokenKit surface (113 non-exempt SDK symbols, 7 exempt rows).

## [0.2.0] - 2026-05-16

### Added

- Multi-file Swift bridge coverage for `Token`, `TokenDriver`, `TokenKeychainContents`, `TokenSession`, `TokenWatcher`, `SmartCard`, `SCardSlotManager`, and `SmartCardATR`.
- Safe Rust wrappers for token configuration snapshots, token watcher callbacks, auth-operation helpers, ATR parsing, TLV construction, and expanded smart-card session control.
- Eight numbered examples (`01_` through `08_`) covering each logical area in a headless-friendly way.
- Eight integration test files, one per logical area, plus a DER certificate fixture for token keychain round-trips.
- `COVERAGE.md` documenting implemented, partial, and skipped CryptoTokenKit surface area against the macOS 26.2 SDK headers.

## [0.1.0] - 2026-05-16

### Added

- `SmartCardSlotManager`, `SmartCardSlot`, and `SmartCard` wrappers for enumerating readers, selecting slots, observing slot-state changes, and issuing APDU commands.
- Delegate-to-Rust KVO observer bridging for `TKSmartCardSlot.state`.
- One-shot `send_ins` APDU exchange that wraps the async `beginSession()` / sync `send(ins:...)` / `endSession()` lifecycle.
- Rust data models for `TokenObjectId`, `TokenOperation`, `TokenKeychainItem`, `TokenKeychainCertificate`, and `TokenKeychainKey` to describe token keychain metadata.
- Initial smoke example for smart-card slot enumeration (superseded in v0.2.0 by `examples/07_scard_slot_manager_slots.rs`).
