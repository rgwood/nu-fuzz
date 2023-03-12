#![no_main]

use libfuzzer_sys::fuzz_target;
use std::process::Command;

fuzz_target!(|data: &[u8]| {
    let panicked_indicator = "thread 'main' panicked";

    if let Ok(nu_cmd) = std::str::from_utf8(data) {
        if !nu_cmd.is_ascii() {
            return;
        }

        // parser OOMs on {{{{
        // fuzzer frequently finds long/infinite ranges with ..
        if nu_cmd.contains("{{{{{") || nu_cmd.contains("..") || nu_cmd.contains(panicked_indicator)
        {
            return;
        }

        let trimmed_args = nu_cmd.trim();
        if trimmed_args
            .chars()
            .any(|c| c.is_ascii_control() || c.is_control())
        {
            return;
        }

        let nu_path = "nu";

        let cmd = Command::new("pledge.com")
            .arg("-p")
            .arg("stdio rpath cpath wpath tty flock vminfo")
            .arg("-v")
            .arg(".") // allow reading from current directory
            .arg(nu_path)
            .arg("-c")
            .arg(trimmed_args)
            .output()
            .expect("failed to execute process");

        if !cmd.status.success() {
            let stdout = String::from_utf8_lossy(&cmd.stdout);
            if stdout.contains(panicked_indicator) {
                eprintln!("{stdout}");
                panic!();
            }
        }
    }
});
