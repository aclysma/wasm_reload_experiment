
use notify::{RecommendedWatcher, Watcher, RecursiveMode};
use std::sync::mpsc::channel;
use std::time::Duration;

#[wasmtime_rust::wasmtime]
trait WasmModule {
    fn render(&mut self, input: &str) -> String;
}

pub fn run() -> Result<(), anyhow::Error> {

    // Load for first time
    let file_path = "../wasm_module/pkg/wasm_module.wasm";
    let mut wasm_module = WasmModule::load_file(file_path)?;

    // Create a channel to receive the events.
    let (tx, rx) = channel();

    // Automatically select the best implementation for your platform.
    // You can also access each implementation directly e.g. INotifyWatcher.
    let mut watcher: RecommendedWatcher = Watcher::new(tx, Duration::from_secs(2))?;

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    watcher.watch("../wasm_module/pkg/wasm_module.wasm", RecursiveMode::Recursive)?;

    loop {
        println!("{}", wasm_module.render("--test string--"));

        match rx.recv_timeout(Duration::from_millis(100)) {
            Ok(event) => {
                println!("{:?}", event);
                println!("Reloading the wasm module");
                wasm_module = WasmModule::load_file(file_path)?;
            },
            Err(e) => println!("watch error: {:?}", e),
        }
    }

    Ok(())
}