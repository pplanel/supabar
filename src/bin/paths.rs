use supabar::{Config, ConfigManager};

fn main() {
    match Config::check_first_run() {
        FirstRun(config) => {},
        NotFirstRun(config) => {}
    }
    let app = supabar::Application::builder()
        .with_
    Config::setup_app_folders();
}
