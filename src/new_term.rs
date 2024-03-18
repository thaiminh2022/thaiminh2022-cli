use std::{process::Command, thread};

pub fn new_term() {
    thread::spawn(|| {
        Command::new("cmd")
            .arg("/C")
            .arg("alacritty")
            .status()
            .expect("cannot open alacritty, check your %PATH");
    });
}
