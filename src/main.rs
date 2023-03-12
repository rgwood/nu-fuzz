use std::process::Command;

use anyhow::Result;

fn main() -> Result<()> {
    let nu_cmd = "$nu";

    let cmd = Command::new("pledge.com")
        .arg("-p")
        .arg("stdio rpath cpath wpath tty flock")
        .arg("nu")
        .arg("-c")
        .arg(nu_cmd)
        .output()
        .expect("failed to execute process");

    let str = String::from_utf8_lossy(&cmd.stdout);
    println!("{str}");

    Ok(())
}
