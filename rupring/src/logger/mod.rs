pub fn print_system_log(level: log::Level, message: &str) {
    let pid = std::process::id();
    let current_time = chrono::Local::now();
    let current_time = current_time.format("%Y-%m-%d %H:%M:%S.3f").to_string();

    println!("{current_time}  {level} {pid}: {message}");
}
