# cryptotokenkit-rs

Safe Rust bindings for Apple's [CryptoTokenKit](https://developer.apple.com/documentation/cryptotokenkit) framework on macOS.

> **Status:** v0.1.0 covers smart-card slot enumeration, slot-state observation, one-shot APDU exchange via `TKSmartCard.send(ins:p1:p2:data:le:)`, and Rust data models for token keychain items / certificates / keys. `TKToken` / `TKTokenSession` driver-side bridging is intentionally deferred to v0.2.

## Quick start

```rust,no_run
use cryptotokenkit::SmartCardSlotManager;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let Some(manager) = SmartCardSlotManager::default_manager() else {
        println!("smart-card entitlement unavailable; slot count = 0");
        return Ok(());
    };

    for name in manager.slot_names()? {
        println!("slot: {name}");
    }
    Ok(())
}
```

## Highlights

- `SmartCardSlotManager::default_manager`, `slot_names`, `slot_named`, and callback-backed `get_slot_with_name`
- `SmartCardSlot` accessors for slot name, input/output limits, state, and `make_smart_card`
- `SlotStateObserver` with delegate-to-Rust callbacks when `TKSmartCardSlot.state` changes
- `SmartCard::send_ins` for one-shot APDU exchange (automatically opens and closes the card session)
- Rust models for `TokenObjectId`, `TokenKeychainItem`, `TokenKeychainCertificate`, and `TokenKeychainKey`

## Entitlements

On macOS, `TKSmartCardSlotManager.default` is only available to processes with the `com.apple.security.smartcard` entitlement. The smoke example treats an unavailable manager as `0` visible slots so CLI verification still succeeds on development machines without that entitlement.

## Smoke example

Run the framework smoke test with:

```bash
cargo run --all-features --example 01_smoke
```

It prints the number of visible slot names and exits with `✅ cryptotokenkit slots OK`.

## License

Licensed under either of [Apache-2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT) at your option.
