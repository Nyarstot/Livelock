pub struct Engine {
    pub is_running: bool,
    pub is_shutdown_required: bool,
    pub target_fps: i32
}

impl Engine {
    pub fn run(&mut self, target_fps: i32) {
        self.target_fps = target_fps;
        self.is_running = true;

        while self.is_running {

            println!("Game loop runnging");

            if self.is_shutdown_required == true {
                self.shutdown();
            }
        }
    }

    pub fn require_shutdown(&mut self) {

    }

    pub fn shutdown(&mut self) {

    }
}