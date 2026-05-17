use cryptotokenkit::SmartCardSlotManager;

#[test]
fn slot_manager_smoke_test() -> Result<(), Box<dyn std::error::Error>> {
    let Some(manager) = SmartCardSlotManager::default_manager() else {
        return Ok(());
    };

    let slot_names = manager.slot_names()?;
    if let Some(slot_name) = slot_names.first() {
        let slot = manager
            .slot_named(slot_name)?
            .expect("slot must still exist");
        assert_eq!(slot.name()?, *slot_name);
        let _ = slot.atr()?;
        let _ = slot.make_smart_card();
    }

    Ok(())
}
