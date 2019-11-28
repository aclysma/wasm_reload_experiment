
use skulpin::AppHandler;
use skulpin::AppControl;
use skulpin::InputState;
use skulpin::TimeState;
use skulpin::VirtualKeyCode;
use skulpin::LogicalSize;
use std::ffi::CString;

use skulpin::skia_safe;

use notify::{RecommendedWatcher, Watcher, RecursiveMode, DebouncedEvent};
use std::sync::mpsc::{Receiver, channel};
use std::time::Duration;

#[wasmtime_rust::wasmtime]
trait WasmModule {
    fn render(&mut self, input: &str) -> String;
}

const FILE_PATH : &str = "../wasm_module/pkg/wasm_module.wasm";

pub fn run() -> Result<(), anyhow::Error> {
    // Setup logging
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Debug)
        .filter_module("cranelift_codegen", log::LevelFilter::Info)
        .init();

    // Load for first time
    let wasm_module = WasmModule::load_file(FILE_PATH)?;

    // Create a channel to receive the events.
    let (tx, rx) = channel();

    // Automatically select the best implementation for your platform.
    // You can also access each implementation directly e.g. INotifyWatcher.
    let mut watcher: RecommendedWatcher = Watcher::new(tx, Duration::from_secs(2))?;

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    watcher.watch(FILE_PATH, RecursiveMode::Recursive)?;

    let example_app = ExampleApp::new(
        rx,
        watcher,
        wasm_module
    );

    skulpin::AppBuilder::new()
        .app_name(CString::new("Skulpin Example App").unwrap())
        .use_vulkan_debug_layer(true)
        .logical_size(LogicalSize::new(900.0, 600.0))
        .run(example_app)
        .expect("The app failed with an error");

    Ok(())
}

struct ExampleApp {
    watcher_rx: Receiver<DebouncedEvent>,
    watcher: RecommendedWatcher,
    wasm_module: WasmModule
}

impl ExampleApp {
    pub fn new(
        watcher_rx: Receiver<DebouncedEvent>,
        watcher: RecommendedWatcher,
        wasm_module: WasmModule
    ) -> Self {
        ExampleApp {
            watcher_rx,
            watcher,
            wasm_module
        }
    }
}

impl AppHandler for ExampleApp {
    fn update(
        &mut self,
        app_control: &mut AppControl,
        input_state: &InputState,
        _time_state: &TimeState
    ) {
        if input_state.is_key_down(VirtualKeyCode::Escape) {
            app_control.enqueue_terminate_process();
        }

        match self.watcher_rx.recv_timeout(Duration::from_millis(0)) {
            Ok(event) => {
                println!("{:?}", event);
                println!("Reloading the wasm module");
                self.wasm_module = WasmModule::load_file(FILE_PATH).unwrap();
            },
            Err(e) => println!("watch error: {:?}", e),
        }
    }

    fn draw(
        &mut self,
        _app_control: &AppControl,
        _input_state: &InputState,
        time_state: &TimeState,
        canvas: &mut skia_safe::Canvas
    ) {
        // Generally would want to clear data every time we draw
        canvas.clear(skia_safe::Color::from_argb(0, 0, 0, 255));

        // Floating point value constantly moving between 0..1 to generate some movement
        let f = ((time_state.system().frame_count as f32 / 30.0).sin() + 1.0) / 2.0;

        // Make a color to draw with
        let mut paint = skia_safe::Paint::new(skia_safe::Color4f::new(1.0 - f, 0.0, f, 1.0), None);
        paint.set_anti_alias(true);
        paint.set_style(skia_safe::paint::Style::Stroke);
        paint.set_stroke_width(2.0);

        // Draw a line
        canvas.draw_line(
            skia_safe::Point::new(100.0, 500.0),
            skia_safe::Point::new(800.0, 500.0),
            &paint
        );

        // Draw a circle
        canvas.draw_circle(
            skia_safe::Point::new(
                200.0 + (f * 500.0),
                420.0
            ),
            50.0,
            &paint
        );

        // Draw a rectangle
        canvas.draw_rect(
            skia_safe::Rect {
                left: 10.0,
                top: 10.0,
                right: 890.0,
                bottom: 590.0
            },
            &paint
        );

        let mut font = skia_safe::Font::default();
        font.set_size(50.0);

        let string_to_draw = self.wasm_module.render("TEST STRING");

        canvas.draw_str(&string_to_draw, (65, 200), &font, &paint);
    }
}
