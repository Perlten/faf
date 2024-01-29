use std::path::Path;

use notify::{RecursiveMode, Result, Watcher};

fn main() -> Result<()> {
    let mut watcher = notify::recommended_watcher(|res| match res {
        Ok(event) => file_change(event),
        Err(e) => println!("watch error: {:?}", e),
    })?;

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    watcher.watch(Path::new("."), RecursiveMode::Recursive)?;
    loop {}

    // Ok(())
}

fn file_change(event: notify::Event) {
    if event.paths[0].to_str().unwrap_or("").contains(".git") {
        println!("ignore .git");
        return;
    }
    println!("event: {:?}", event);
}
