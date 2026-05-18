# cryptotokenkit-rs

Safe Rust bindings for Apple's [CryptoTokenKit](https://developer.apple.com/documentation/cryptotokenkit) framework on macOS.

> **Status:** v0.2.3 keeps the audited macOS-public CryptoTokenKit surface at 100% verified wrapper coverage and now documents the public safe Rust API against the corresponding framework types, methods, and delegate entry points. Only iOS-only/deprecated SDK exemptions remain in [`COVERAGE.md`](COVERAGE.md).

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

- `Token` / `TokenDriver` creation, configuration snapshots, and token-driver configuration add/remove helpers.
- `TokenKeychainContents` round-trips for token keys and certificates.
- `TokenSession` helpers for base/password/smart-card PIN auth operations, underlying token access, and session-delegate callbacks.
- `Token`, `TokenDriver`, and `SmartCardTokenDriver` delegate bridges, including `TokenKeyAlgorithm` and `TokenKeyExchangeParameters` helpers.
- `TokenWatcher` enumeration plus insertion/removal callbacks.
- `SmartCard` / `SCardSlotManager` reader enumeration, ATR access, session control, APDU transmit, slot round-tripping, secure PIN interactions, and one-shot `send_ins`.
- `SmartCardATR` parsing from bytes or a source callback, plus TLV helper constructors and pure-Rust parse helpers.
- Framework error constants and codes via `TK_ERROR_DOMAIN`, `TKErrorCode`, and `CryptoTokenKitError::framework_code()`.

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
cargo run --example 09_token_delegate_bridges
cargo run --example 10_smart_card_user_interactions
```

The entitlement-dependent smart-card examples degrade gracefully when the Smart Card entitlement is unavailable or no reader/card is present. `10_smart_card_user_interactions` uses a mock smart card so it remains headless-friendly.

## Entitlements

On macOS, `TKSmartCardSlotManager.default` is only available to processes with the `com.apple.security.smartcard` entitlement. The slot-manager and smart-card examples treat an unavailable manager as a non-fatal skip so command-line verification still succeeds on development machines without that entitlement.

## Coverage audit

- [`COVERAGE.md`](COVERAGE.md) lists every public type/function audited from the macOS 26.2 `CryptoTokenKit` headers.
- [`COVERAGE_AUDIT.md`](COVERAGE_AUDIT.md) records the audit worklist and now reports 113/113 verified non-exempt macOS-public symbols (7 additional SDK rows are exempt).
- `tests/` contains one smoke test per logical area, including secure PIN interaction and delegate coverage.
- `examples/` contains ten numbered examples covering each logical area.

## License

Licensed under either of [Apache-2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT) at your option.
