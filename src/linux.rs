pub fn detect() {
    if cfg!(target_os = "linux") {
        println!("Linux");
    } else {
        println!("Not Linux");
    }

    match sys_info::hostname() {
        Ok(hostname) => println!("Hostname: {}", hostname),
        Err(e) => eprintln!("Error getting hostname: {}", e),
    }
}
