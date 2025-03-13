use std::process::Command;

pub fn get_screen_dimensions() -> Option<(u32, u32)> {
    let output = Command::new("sh")
        .arg("-c")
        .arg("xdpyinfo | grep dimensions")
        .output()
        .expect("Failed to execute command");

    let stdout = String::from_utf8_lossy(&output.stdout);

    stdout.split_whitespace().nth(1).and_then(|dimensions| {
        let parts: Vec<&str> = dimensions.split('x').collect();
        if parts.len() == 2 {
            Some((parts[0].parse::<u32>().ok()?, parts[1].parse::<u32>().ok()?))
        } else {
            None
        }
    })
}
