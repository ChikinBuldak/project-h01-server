use std::time::Duration;
use bevy::{app::ScheduleRunnerPlugin, prelude::*};
mod plugins;
mod systems;
mod components;
mod network;

fn main() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(Duration::from_secs_f64(1.0/60.0))));
    app.run();
}
