pub fn detect() {
    if cfg!(target_os = "macos") {
        println!("macOS");
    } else {
        println!("Not macOS");
    }

    match sys_info::hostname() {
        Ok(hostname) => println!("Hostname: {}", hostname),
        Err(e) => eprintln!("Error getting hostname: {}", e),
    }
}
