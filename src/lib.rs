pub mod app;
pub mod audio;
pub mod ui;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::App;

    #[test]
    fn app_toggle_running() {
        let app = App::new();
        app.running = true;
        if app.running {
            app.toggle_running();
            assert_eq!(app.running, false);
        }
    }
}
