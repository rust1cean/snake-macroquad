use core::App;
use snake::{cfg::APP_CONFIG, window_config, Game};

#[macroquad::main(window_config)]
async fn main() {
    App::new(APP_CONFIG).add_plugin::<Game>().run().await;
}
