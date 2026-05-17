use std::sync::{Arc, Mutex};

use cryptotokenkit::{
    SmartCard, SmartCardPinCompletion, SmartCardPinConfirmation, SmartCardPinFormat,
    SmartCardUserInteractionDelegate, SmartCardUserInteractionEvent,
};

struct RecordingInteractionDelegate {
    events: Arc<Mutex<Vec<SmartCardUserInteractionEvent>>>,
}

impl SmartCardUserInteractionDelegate for RecordingInteractionDelegate {
    fn character_entered(&mut self, _interaction: &cryptotokenkit::SmartCardUserInteraction) {
        self.events
            .lock()
            .unwrap()
            .push(SmartCardUserInteractionEvent::CharacterEntered);
    }

    fn correction_key_pressed(&mut self, _interaction: &cryptotokenkit::SmartCardUserInteraction) {
        self.events
            .lock()
            .unwrap()
            .push(SmartCardUserInteractionEvent::CorrectionKeyPressed);
    }

    fn validation_key_pressed(&mut self, _interaction: &cryptotokenkit::SmartCardUserInteraction) {
        self.events
            .lock()
            .unwrap()
            .push(SmartCardUserInteractionEvent::ValidationKeyPressed);
    }

    fn invalid_character_entered(
        &mut self,
        _interaction: &cryptotokenkit::SmartCardUserInteraction,
    ) {
        self.events
            .lock()
            .unwrap()
            .push(SmartCardUserInteractionEvent::InvalidCharacterEntered);
    }

    fn old_pin_requested(&mut self, _interaction: &cryptotokenkit::SmartCardUserInteraction) {
        self.events
            .lock()
            .unwrap()
            .push(SmartCardUserInteractionEvent::OldPinRequested);
    }

    fn new_pin_requested(&mut self, _interaction: &cryptotokenkit::SmartCardUserInteraction) {
        self.events
            .lock()
            .unwrap()
            .push(SmartCardUserInteractionEvent::NewPinRequested);
    }

    fn new_pin_confirmation_requested(
        &mut self,
        _interaction: &cryptotokenkit::SmartCardUserInteraction,
    ) {
        self.events
            .lock()
            .unwrap()
            .push(SmartCardUserInteractionEvent::NewPinConfirmationRequested);
    }
}

#[test]
fn smart_card_slot_and_user_interactions_work() -> Result<(), Box<dyn std::error::Error>> {
    let card = SmartCard::mock("Mock Reader")?;
    let slot = card.slot()?;
    assert_eq!(slot.name()?, "Mock Reader");
    assert_eq!(card.slot_name()?, "Mock Reader");

    let pin_format = SmartCardPinFormat::default();
    let verification = card
        .user_interaction_for_secure_pin_verification(&pin_format, &[0x00, 0x20, 0x00, 0x00], 0)?
        .expect("mock smart card should return a PIN verification interaction");
    verification.set_initial_timeout(1.5);
    verification.set_interaction_timeout(2.5);
    verification.set_pin_completion(SmartCardPinCompletion::MAX_LENGTH);
    verification.set_pin_message_indices(Some(&[1, 2, 3]))?;
    verification.set_locale_identifier(Some("en-US"))?;
    assert!((verification.initial_timeout() - 1.5).abs() < f64::EPSILON);
    assert!((verification.interaction_timeout() - 2.5).abs() < f64::EPSILON);
    assert_eq!(
        verification.pin_completion().bits(),
        SmartCardPinCompletion::MAX_LENGTH.bits()
    );
    assert_eq!(verification.pin_message_indices()?, Some(vec![1, 2, 3]));
    assert_eq!(verification.locale_identifier()?, "en-US");
    assert_eq!(verification.result_status_word(), 0);
    assert_eq!(verification.result_data()?, None);

    let events = Arc::new(Mutex::new(Vec::new()));
    let handle = verification.set_delegate(RecordingInteractionDelegate {
        events: Arc::clone(&events),
    })?;
    assert!(verification.has_delegate());
    for event in [
        SmartCardUserInteractionEvent::CharacterEntered,
        SmartCardUserInteractionEvent::CorrectionKeyPressed,
        SmartCardUserInteractionEvent::ValidationKeyPressed,
        SmartCardUserInteractionEvent::InvalidCharacterEntered,
        SmartCardUserInteractionEvent::OldPinRequested,
        SmartCardUserInteractionEvent::NewPinRequested,
        SmartCardUserInteractionEvent::NewPinConfirmationRequested,
    ] {
        verification.simulate_delegate_event(event);
    }
    verification.run()?;
    assert!(!verification.cancel());
    drop(handle);
    assert!(!verification.has_delegate());
    assert_eq!(
        *events.lock().unwrap(),
        vec![
            SmartCardUserInteractionEvent::CharacterEntered,
            SmartCardUserInteractionEvent::CorrectionKeyPressed,
            SmartCardUserInteractionEvent::ValidationKeyPressed,
            SmartCardUserInteractionEvent::InvalidCharacterEntered,
            SmartCardUserInteractionEvent::OldPinRequested,
            SmartCardUserInteractionEvent::NewPinRequested,
            SmartCardUserInteractionEvent::NewPinConfirmationRequested,
        ]
    );

    let change = card
        .user_interaction_for_secure_pin_change(&pin_format, &[0x00, 0x24, 0x00, 0x00], 0, 8)?
        .expect("mock smart card should return a PIN change interaction");
    change.set_pin_confirmation(SmartCardPinConfirmation::CURRENT);
    assert_eq!(
        change.pin_confirmation().bits(),
        SmartCardPinConfirmation::CURRENT.bits()
    );
    change.run()?;

    Ok(())
}
