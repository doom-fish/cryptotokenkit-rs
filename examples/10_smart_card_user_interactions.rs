use std::sync::{Arc, Mutex};

use cryptotokenkit::{
    SmartCard, SmartCardPinCompletion, SmartCardPinConfirmation, SmartCardPinFormat,
    SmartCardUserInteractionDelegate, SmartCardUserInteractionEvent,
};

struct InteractionDelegate {
    events: Arc<Mutex<Vec<SmartCardUserInteractionEvent>>>,
}

impl SmartCardUserInteractionDelegate for InteractionDelegate {
    fn character_entered(&mut self, _interaction: &cryptotokenkit::SmartCardUserInteraction) {
        self.events
            .lock()
            .unwrap()
            .push(SmartCardUserInteractionEvent::CharacterEntered);
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let card = SmartCard::mock("Example Mock Reader")?;
    let slot = card.slot()?;
    let pin_format = SmartCardPinFormat::default();
    let verification = card
        .user_interaction_for_secure_pin_verification(&pin_format, &[0x00, 0x20, 0x00, 0x00], 0)?
        .expect("mock smart card should create a verification interaction");
    verification.set_pin_completion(SmartCardPinCompletion::KEY);
    verification.set_pin_message_indices(Some(&[1]))?;
    verification.set_locale_identifier(Some("en-US"))?;

    let events = Arc::new(Mutex::new(Vec::new()));
    let _delegate_handle = verification.set_delegate(InteractionDelegate {
        events: Arc::clone(&events),
    })?;
    verification.simulate_delegate_event(SmartCardUserInteractionEvent::CharacterEntered);
    verification.run()?;

    let change = card
        .user_interaction_for_secure_pin_change(&pin_format, &[0x00, 0x24, 0x00, 0x00], 0, 8)?
        .expect("mock smart card should create a change interaction");
    change.set_pin_confirmation(SmartCardPinConfirmation::CURRENT);
    change.run()?;

    println!("slot: {}", slot.name()?);
    println!("verification-locale: {}", verification.locale_identifier()?);
    println!("verification-events: {:?}", events.lock().unwrap().clone());
    println!("change-confirmation: {}", change.pin_confirmation().bits());
    Ok(())
}
