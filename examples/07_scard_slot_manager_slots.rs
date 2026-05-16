use cryptotokenkit::SmartCardSlotManager;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let count = if let Some(manager) = SmartCardSlotManager::default_manager() {
        let slots = manager.slot_names()?;
        println!("smart-card slots: {}", slots.len());
        for slot in &slots {
            println!("slot: {slot}");
        }
        slots.len()
    } else {
        println!(
            "smart-card slots: 0 (default manager unavailable; missing entitlement or no access)"
        );
        0
    };

    println!("visible slot count = {count}");
    println!("✅ cryptotokenkit slots OK");
    Ok(())
}
