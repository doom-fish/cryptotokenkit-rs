use cryptotokenkit::TokenWatcher;
use std::sync::{Arc, Mutex};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut watcher = TokenWatcher::new();
    let seen = Arc::new(Mutex::new(Vec::new()));
    let seen_clone = Arc::clone(&seen);
    watcher.set_insertion_handler(move |token_id| {
        seen_clone.lock().unwrap().push(token_id);
    })?;

    let removed = Arc::new(Mutex::new(Vec::new()));
    let removed_clone = Arc::clone(&removed);
    watcher.add_removal_handler("missing-token", move |token_id| {
        removed_clone.lock().unwrap().push(token_id);
    })?;

    let token_ids = watcher.token_ids()?;
    println!("visible tokens: {}", token_ids.len());
    println!(
        "insertion callbacks observed: {}",
        seen.lock().unwrap().len()
    );
    println!(
        "removal callbacks observed: {}",
        removed.lock().unwrap().len()
    );
    println!("✅ token watcher snapshot OK");
    Ok(())
}
