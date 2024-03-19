use crate::settings::SETTINGS;

pub fn logger(event: bool, thread_id: std::thread::ThreadId, message: &str) {
    let _colors = &SETTINGS.colors;
    if event {
        println!("{}[CodePulse]{} {}[EVENT]{} {}[{:?}]{} {}", _colors.cyan_green, _colors.endc, _colors.cyan, _colors.endc, _colors.blue, thread_id, _colors.endc, message);
    } else {
        println!("{}[CodePulse]{} {}[ERROR]{} {}[{:?}]{} {}", _colors.fail, _colors.endc, _colors.cyan, _colors.endc, _colors.blue, thread_id, _colors.endc, message);
    }
}