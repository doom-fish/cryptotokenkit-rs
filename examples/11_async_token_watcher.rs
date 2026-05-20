#[cfg(feature = "async")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    use cryptotokenkit::async_api::AsyncTokenWatcher;

    pollster::block_on(async {
        let watcher = AsyncTokenWatcher::new();
        println!("visible_tokens={}", watcher.token_ids()?.len());

        let insertions = watcher.insertion_stream(8)?;
        println!("Insert a token to receive the next async event...");
        if let Some(token_id) = insertions.next().await {
            println!("inserted={token_id}");
            println!("info={:?}", insertions.token_info(&token_id)?);
        }

        Ok::<(), Box<dyn std::error::Error>>(())
    })
}

#[cfg(not(feature = "async"))]
fn main() {}
