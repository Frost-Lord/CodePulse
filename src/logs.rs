use crate::settings::SETTINGS;

pub fn logger(event: bool, thread_id: std::thread::ThreadId, message: &str) {
    let settings = SETTINGS.lock().unwrap();
    let colors = &settings.colors;
    if event {
        println!("{}[CodePulse]{} {}[EVENT]{} {}[{:?}]{} {}", colors.cyan_green, colors.endc, colors.cyan, colors.endc, colors.blue, thread_id, colors.endc, message);
    } else {
        println!("{}[CodePulse]{} {}[ERROR]{} {}[{:?}]{} {}", colors.fail, colors.endc, colors.cyan, colors.endc, colors.blue, thread_id, colors.endc, message);
    }
}

pub fn log_settings(success: bool, message: &str) {
    let settings = SETTINGS.lock().unwrap();
    let colors = &settings.colors;
    if success {
        println!("{}[CodePulse]{} {}[SETTINGS]{} {}", colors.cyan_green, colors.endc, colors.cyan, colors.endc, message);
    } else {
        println!("{}[CodePulse]{} {}[ERROR]{} {}", colors.fail, colors.endc, colors.cyan, colors.endc, message);
    }
}
