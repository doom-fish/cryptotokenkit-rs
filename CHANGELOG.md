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
- The Swift bridge unboxed every handle without checking its class, so a
  handle of the wrong type was reinterpreted as the expected one. Safe code
  could trigger this: `DerefMut` let `mem::swap` move a secure-PIN
  verification into a `SmartCardUserInteractionForSecurePinChange`, and
  `set_pin_confirmation` then aborted the process. Every bridge function now
  checks the class (`as?`) and a mismatch is
  `CryptoTokenKitError::InvalidArgument`. Objects returned by Rust delegates
  are checked too, and a mismatch fails the callback instead of being replaced
  with a newly made default session, token or auth operation.
- Data a Rust `TokenSessionDelegate` returned from `sign_data`,
  `decrypt_data` or `perform_key_exchange` (decrypted plaintext and shared
  secrets among it) crossed the bridge as a JSON array of numbers, leaving
  unwiped copies in Rust, in the C string and in Swift. It now travels in a
  byte buffer that is wiped once Swift has made the `Data` it hands to
  CryptoTokenKit, and the Rust copies are `Zeroizing`.

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
- The PIN-interaction getters (`pin_completion`, `pin_message_indices`,
  `locale_identifier`, `result_status_word`, `result_data`,
  `pin_confirmation`) preferred a process-wide table that every setter wrote
  to. Once a setter had run, `result_status_word` and `result_data` returned
  stale values instead of the card's reply, and the table, keyed by object
  address, grew with every interaction. The getters now read the interaction.
- Token and smart-card-token constructors, token configuration, key and
  certificate lookups, `keychain_contents_items`, `get_smart_card`, the
  token-watcher queries, `slot_names` and the secure-PIN interaction
  constructors report their failures with a status. A macOS version gate is
  `Unsupported` and a `TKError` (for example `ObjectNotFound`) keeps its code
  in `framework_code()`; before, all of these were `FrameworkError`.

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
- **Breaking:** Accessors that could not report a bridge failure return
  `Result`: `SmartCard::valid`, `allowed_protocols`, `set_allowed_protocols`,
  `current_protocol`, `sensitive`, `set_sensitive`, `cla`, `set_cla`,
  `use_extended_length`, `set_use_extended_length`, `use_command_chaining`,
  `set_use_command_chaining` and `end_session`; `SmartCardSlot::max_input_length`,
  `max_output_length`, `state` and `make_smart_card`;
  `SmartCardUserInteraction::has_delegate`, `clear_delegate`,
  `simulate_delegate_event`, `initial_timeout`, `set_initial_timeout`,
  `interaction_timeout`, `set_interaction_timeout` and `cancel`;
  `SmartCardUserInteractionForPinOperation::pin_completion`,
  `set_pin_completion` and `result_status_word`;
  `SmartCardUserInteractionForSecurePinChange::pin_confirmation` and
  `set_pin_confirmation`; `SmartCardTokenSession::smart_card`;
  `TokenKeyExchangeParameters::requested_size`; `has_delegate` and
  `clear_delegate` on `TokenSession`, `Token`, `TokenDriver` and
  `SmartCardTokenDriver`; `Token::invoke_delegate_terminate_session`; and
  `invoke_delegate_terminate_token` on both drivers.
- **Breaking:** `TokenSession::new` and `SmartCardTokenSession::new` return
  `Result<Self, CryptoTokenKitError>` instead of panicking.
- **Breaking:** `SmartCardUserInteractionForPinOperation`,
  `SmartCardUserInteractionForSecurePinVerification` and
  `SmartCardUserInteractionForSecurePinChange` no longer implement `DerefMut`;
  `Deref` remains, and every method takes `&self`.
- **Breaking:** `TokenSession::invoke_delegate_decrypt_data` and
  `invoke_delegate_perform_key_exchange` return
  `Result<Zeroizing<Vec<u8>>, CryptoTokenKitError>`.
- **Breaking:** The failures listed under Fixed that used to be
  `FrameworkError` are now `InvalidArgument`, `Unsupported` or
  `Unknown { code }` carrying the `TKError` code.
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
