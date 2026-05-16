# Changelog

## [0.1.0] - 2026-05-16

### Added

- `SmartCardSlotManager`, `SmartCardSlot`, and `SmartCard` wrappers for enumerating readers, selecting slots, observing slot-state changes, and issuing APDU commands.
- Delegate-to-Rust KVO observer bridging for `TKSmartCardSlot.state`.
- One-shot `send_ins` APDU exchange that wraps the async `beginSession()` / sync `send(ins:...)` / `endSession()` lifecycle.
- Rust data models for `TokenObjectId`, `TokenOperation`, `TokenKeychainItem`, `TokenKeychainCertificate`, and `TokenKeychainKey` to describe token keychain metadata.
- Smoke example `examples/01_smoke.rs` that lists visible smart-card slots (or `0` when the Smart Card entitlement is unavailable).
