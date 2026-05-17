use cryptotokenkit::TokenWatcher;
use std::sync::{Arc, Mutex};

#[test]
fn watcher_lists_tokens_and_triggers_callbacks() -> Result<(), Box<dyn std::error::Error>> {
    let mut watcher = TokenWatcher::new();
    let inserted = Arc::new(Mutex::new(Vec::new()));
    let inserted_clone = Arc::clone(&inserted);
    watcher.set_insertion_handler(move |token_id| inserted_clone.lock().unwrap().push(token_id))?;

    let removed = Arc::new(Mutex::new(Vec::new()));
    let removed_clone = Arc::clone(&removed);
    watcher.add_removal_handler("missing-token", move |token_id| {
        removed_clone.lock().unwrap().push(token_id);
    })?;

    let token_ids = watcher.token_ids()?;
    assert!(!token_ids.is_empty());
    assert!(!inserted.lock().unwrap().is_empty());
    assert_eq!(
        removed.lock().unwrap().as_slice(),
        &[String::from("missing-token")]
    );

    if let Some(token_id) = token_ids.first() {
        let _ = watcher.token_info(token_id)?;
    }

    Ok(())
}
