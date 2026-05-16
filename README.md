# cryptotokenkit-rs

Safe Rust bindings for Apple's [CryptoTokenKit](https://developer.apple.com/documentation/cryptotokenkit) framework on macOS.

> **Status:** v0.2.0 expands the Swift bridge and safe Rust API across `Token`, `TokenDriver`, `TokenKeychainContents`, `TokenSession`, `TokenWatcher`, `SmartCard`, `SCardSlotManager`, and `SmartCardATR`. Remaining iOS-only and extension-host-only gaps are called out in [`COVERAGE.md`](COVERAGE.md).

## Quick start

```rust,no_run
use cryptotokenkit::{Token, TokenDriver};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let driver = TokenDriver::new();
    let token = Token::new(&driver, "com.example.cryptotokenkit.token")?;
    token.set_configuration_data(Some(b"demo"))?;

    println!("instance-id: {}", token.instance_id()?);
    Ok(())
}
```

## Covered areas

- `Token` / `TokenDriver` creation plus configuration snapshots.
- `TokenKeychainContents` round-trips for token keys and certificates.
- `TokenSession` helpers for base/password/smart-card PIN auth operations.
- `TokenWatcher` enumeration plus insertion/removal callbacks.
- `SmartCard` / `SCardSlotManager` reader enumeration, ATR access, session control, APDU transmit, and one-shot `send_ins`.
- `SmartCardATR` parsing from bytes or a source callback, plus TLV helper constructors.

## Examples

```bash
cargo run --example 01_token_snapshot
cargo run --example 02_token_driver_snapshot
cargo run --example 03_token_keychain_contents_roundtrip
cargo run --example 04_token_session_auth_ops
cargo run --example 05_token_watcher_snapshot
cargo run --example 06_smart_card_session
cargo run --example 07_scard_slot_manager_slots
cargo run --example 08_smart_card_atr_parse
```

The smart-card examples degrade gracefully when the Smart Card entitlement is unavailable or no reader/card is present.

## Entitlements

On macOS, `TKSmartCardSlotManager.default` is only available to processes with the `com.apple.security.smartcard` entitlement. The slot-manager and smart-card examples treat an unavailable manager as a non-fatal skip so command-line verification still succeeds on development machines without that entitlement.

## Coverage audit

- [`COVERAGE.md`](COVERAGE.md) lists every public type/function audited from the macOS 26.2 `CryptoTokenKit` headers.
- `tests/` contains one smoke test per logical area.
- `examples/` contains one numbered example per logical area.

## License

Licensed under either of [Apache-2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT) at your option.
