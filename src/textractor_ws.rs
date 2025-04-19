use std::string::String;
use std::thread::spawn;
use std::time::Duration;

use crossbeam_channel::{bounded, Sender};
use once_cell::sync::OnceCell;

pub const ADDRESS: &str = "0.0.0.0:6677";
const BOUND: usize = 1000;
const SEND_TIMEOUT: Duration = Duration::from_secs(1);

fn run_once() -> Result<Sender<String>, ws::Error> {
    let (sender, receiver) = bounded(BOUND);
    let me = ws::WebSocket::new(|_| move |_| Ok(()))?;
    let broadcaster = me.broadcaster();
    spawn(move || me.listen(ADDRESS).expect("Failed to spawn listener thread"));
    spawn(move || {
        while let Ok(msg) = receiver.recv() {
            broadcaster.broadcast(msg).ok();
        }
    });
    Ok(sender)
}

fn get_sender() -> &'static Sender<String> {
    static INSTANCE: OnceCell<Sender<String>> = OnceCell::new();
    INSTANCE.get_or_init(|| run_once().expect("WebSocket init failed"))
}

pub(crate) fn start_server() {
    get_sender();
}

pub(crate) fn handle(s: String) {
    let sender = get_sender();
    sender.send_timeout(s, SEND_TIMEOUT).ok();
}
