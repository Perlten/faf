mod cli_arguments;

use clap::Parser;
use cli_arguments::FafArgs;
use notify::{RecursiveMode, Result, Watcher};
use std::{path::Path, process::Command};

fn main() -> Result<()> {
    let args = FafArgs::parse();

    println!("Start watching current directory");
    let mut watcher = notify::recommended_watcher(move |res| match res {
        Ok(event) => file_change(event, &args.command),
        Err(e) => println!("watch error: {:?}", e),
    })?;

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    watcher.watch(Path::new(&args.location), RecursiveMode::Recursive)?;
    loop {}

    // Ok(())
}

fn file_change(event: notify::Event, command: &str) {
    let ignore_vec = vec![".git", ".gitignore"];
    let path = event.paths[0].to_str().unwrap_or("").to_owned();
    for ignore in ignore_vec {
        if path.contains(ignore) {
            println!("ignore {}", ignore);
            return;
        }
    }
    match Command::new("sh").arg("-c").arg(command).spawn() {
        Ok(_) => (),
        Err(e) => println!("error: {:?}", e.to_string()),
    };
}
