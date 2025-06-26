pub mod engine;

fn main() {
    let mut eng = engine::Engine { is_running: true, is_shutdown_required: false, target_fps: 60 };
    eng.run(60);

    println!("Hello, world!");
}
