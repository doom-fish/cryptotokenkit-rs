# cryptotokenkit-rs

Safe Rust bindings for Apple's [CryptoTokenKit](https://developer.apple.com/documentation/cryptotokenkit) framework on macOS.

## Installation

```toml
[dependencies]
cryptotokenkit-rs = "0.4"
```

The library is imported as `cryptotokenkit`. The optional `async` feature adds executor-agnostic streams for `TKTokenWatcher` events.

## Requirements

- macOS 10.13 or newer (the Swift bridge's deployment target).
- Token-driver configurations (`TokenDriver::driver_configurations`, `add_token_configuration`, `remove_token_configuration`, `Token::configuration`, `Token::set_configuration_data`) need macOS 10.15, `TokenWatcher::token_info` needs macOS 12, and `SmartCardTokenSession::get_smart_card` needs macOS 26. On older systems they return an error.

## Quick start

```rust,no_run
use cryptotokenkit::{Token, TokenDriver};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let driver = TokenDriver::new();
    let token = Token::new(&driver, "com.example.cryptotokenkit.token")?;

    println!("instance-id: {}", token.instance_id()?);
    Ok(())
}
```

## Covered areas

- `Token` / `TokenDriver` creation, configuration snapshots, and token-driver configuration add/remove helpers. `CryptoTokenKit` only exposes token-driver configurations to the app that contains the token extension; everywhere else `driver_configurations()` is empty and add/remove/`set_configuration_data` return `CryptoTokenKitError::Unsupported`.
- `TokenKeychainContents` round-trips for token keys and certificates.
- `TokenSession` helpers for base/password/smart-card PIN auth operations, underlying token access, and session-delegate callbacks.
- `Token`, `TokenDriver`, and `SmartCardTokenDriver` delegate bridges, including `TokenKeyAlgorithm` and `TokenKeyExchangeParameters` helpers.
- `TokenWatcher` enumeration plus insertion/removal callbacks.
- `SmartCard` / `SCardSlotManager` reader enumeration, ATR access, session control, APDU transmit, slot round-tripping, secure PIN interactions, and one-shot `send_ins`.
- `SmartCardATR` parsing from bytes or a source callback, plus TLV helper constructors and pure-Rust parse helpers. The parsers treat card data as untrusted and return `None` for malformed input; the constructors return `None` for values `CryptoTokenKit` would reject with an exception (BER tag 0, compact tags above 15 or values over 15 bytes, simple values over 65535 bytes).
- Framework error constants and codes via `TK_ERROR_DOMAIN`, `TKErrorCode` (including `InvalidatedDeviceKey` from SDK 27), and `CryptoTokenKitError::framework_code()`. Bridge failures (`InvalidArgument`, `FrameworkError`, `TimedOut`, `Unsupported`) never alias a `TKErrorCode`.

## PINs and passwords

`TokenPasswordAuthOperation::password` and `TokenSmartCardPinAuthOperation::pin` return `zeroize::Zeroizing<String>`, and the buffer the Swift bridge hands to Rust is wiped before it is freed. PINs and passwords never pass through the bridge's JSON snapshots. The copies `CryptoTokenKit` keeps (its own `NSString` properties) cannot be wiped from Rust.

## Callbacks and threads

Delegates, slot observers and token-watcher handlers are kept alive by the Swift objects that `CryptoTokenKit` calls. Dropping a handle stops delivery to your Rust code; a callback that is already running finishes first, and one that arrives later gets an error or is ignored. The synchronous smart-card calls (`begin_session`, which `send_ins` and `with_session` also use, `transmit_request`, `SmartCardSlotManager::get_slot_with_name`, `SmartCardUserInteraction::run`) wait at most 30 seconds for the framework's reply (`run` waits for the interaction's own timeouts, up to an hour) and report `TimedOut` instead of a result that arrives later; a session that begins after the timeout is ended, and a timed-out interaction is cancelled.

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

- [`COVERAGE.md`](COVERAGE.md) lists the audited `CryptoTokenKit` API rows from the macOS 26.2 headers; the SDK 27.0 headers only add `TKErrorCodeInvalidatedDeviceKey`, which is mapped.
- [`COVERAGE_AUDIT.md`](COVERAGE_AUDIT.md) counts rows, not individual symbols: related selectors and properties share a row. Some rows are covered by Rust-side equivalents rather than one-to-one wrappers, and token-configuration rows only take effect in the app that contains a token extension.
- `tests/` contains one smoke test per logical area, including secure PIN interaction and delegate coverage, plus malformed-input tests for the TLV parsers.
- `examples/` contains eleven numbered examples covering each logical area.

## License

Licensed under either of [Apache-2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT) at your option.
