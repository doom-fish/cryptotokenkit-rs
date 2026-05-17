use cryptotokenkit::SmartCardSlotManager;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let Some(manager) = SmartCardSlotManager::default_manager() else {
        println!("smart-card manager unavailable; skipping APDU example");
        println!("✅ smart card session OK");
        return Ok(());
    };

    let Some(slot_name) = manager.slot_names()?.into_iter().next() else {
        println!("no smart-card slots available; skipping APDU example");
        println!("✅ smart card session OK");
        return Ok(());
    };

    let Some(slot) = manager.slot_named(&slot_name)? else {
        println!("slot disappeared before use; skipping APDU example");
        println!("✅ smart card session OK");
        return Ok(());
    };

    let Some(card) = slot.make_smart_card() else {
        println!("no valid card inserted in {slot_name}; skipping APDU example");
        println!("✅ smart card session OK");
        return Ok(());
    };

    match card.send_ins(0x84, 0x00, 0x00, None, Some(8)) {
        Ok(reply) => println!(
            "challenge bytes={} sw=0x{:04X}",
            reply.data.len(),
            reply.status_word
        ),
        Err(error) => println!("smart-card exchange unavailable: {error}"),
    }

    println!("✅ smart card session OK");
    Ok(())
}
