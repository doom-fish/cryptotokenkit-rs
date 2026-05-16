# Changelog

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
