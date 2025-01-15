use std::process::Command;

pub fn detect() {
    if cfg!(target_os = "windows") {
        println!("Windows");
    } else {
        println!("Not Windows");
        return;
    }

    match sys_info::hostname() {
        Ok(hostname) => println!("Hostname: {}", hostname),
        Err(e) => eprintln!("Error getting hostname: {}", e),
    }

    let is_vm = is_running_in_vm();
    println!(
        "The system is {}running on a virtual machine.",
        if is_vm { "" } else { "not " }
    );

    check_wsl();
}

fn is_running_in_vm() -> bool {
    // Placeholder for VM detection logic
    // `GetSystemFirmwareTable` is used in GO, need alternative in Rust
    println!("VM detection logic not implemented yet.");
    false
}

fn check_wsl() {
    let output = Command::new("wsl").arg("--list").arg("--verbose").output();

    match output {
        Ok(output) => {
            if !output.status.success() {
                eprintln!(
                    "WSL is not installed or accessible: {}",
                    String::from_utf8_lossy(&output.stderr)
                );
                return;
            }

            let stdout = String::from_utf8_lossy(&output.stdout);
            println!("WSL Distributions:\n{}", stdout);

            if stdout.contains("WSL 2") {
                println!("WSL2 is enabled and accessible.");
                validate_wsl2();
            } else if stdout.contains("WSL 1") {
                println!("WSL1 is enabled and accessible.");
            } else {
                println!("No active WSL distributions found.");
            }
        }
        Err(e) => eprintln!("Error checking WSL: {}", e),
    }
}

fn validate_wsl2() {
    let output = Command::new("wsl")
        .arg("-e")
        .arg("uname")
        .arg("-a")
        .output();

    match output {
        Ok(output) => {
            if output.status.success() {
                println!(
                    "WSL2 Test Output:\n{}",
                    String::from_utf8_lossy(&output.stdout)
                );
            } else {
                eprintln!(
                    "Error validating WSL2: {}",
                    String::from_utf8_lossy(&output.stderr)
                );
            }
        }
        Err(e) => eprintln!("Error running WSL2 test: {}", e),
    }
}
