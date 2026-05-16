# Changelog

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
