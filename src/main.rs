use futures::{
    channel::mpsc::{channel, Receiver},
    SinkExt, StreamExt
};
use notify::{Config, Event, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::Path;

fn main() {
    let path = std::env::args()
        .nth(1)
        .expect("No path arg found");
    println!("Watching: {}", path);
    
    let path = Path::new(".").join(path);

    futures::executor::block_on(async {
        if let Err(e) = async_watch(path.as_path()).await{
            println!("Error: {:?}", e)
        }
    });
}

fn async_watcher() -> notify::Result<(RecommendedWatcher, Receiver<notify::Result<Event>>)> {
    let (mut tx, rx) = channel(1);

    let watcher = RecommendedWatcher::new(
        move |res| {
            futures::executor::block_on(async {
                tx.send(res).await.unwrap();
            })
        },
        Config::default(),
    )?;

    Ok((watcher, rx))
}

async fn async_watch(path: &Path)-> notify::Result<()> {
    let (mut watcher, mut rx) = async_watcher()?;

    watcher.watch(path, RecursiveMode::Recursive)?;

    while let Some(res) = rx.next().await {
        match res {
            Ok(event) => println!("Changed: {:?}", event),
            Err(e) => println!("watch error: {:?}", e),
        }
    }

    Ok(())
}
