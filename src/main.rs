/* 
    I wanna rewrite my own update script in rust
    I need to figure out how to execute shell commands in rust
*/

use std::process::Command;
use std::error::Error;

fn update_with_yay() /* -> Result<(), Box<dyn Error>> */ {
    // println!("This will update your aur and pacman pkg's");
    // sudo::escalate_if_needed()?;
    Command::new("paru")
        // .arg("--noconfirm")
        .spawn()
        .expect("paru failed to start");
    // Ok(())
}

fn update_with_flatpak() {
    // println!("This will update your flatpaks");
    Command::new("flatpak")
        .arg("update")
        .arg("--user")
        .arg("-y")
        // .arg("--noninteractive")
        .spawn()
        .expect("flatpak failed to start");
}

fn pre_update_backup() {
    println!("This will backup important files before updating");
}

fn post_update_backup() {
    println!("This will backup important files after updating");
}

fn main() {
    // pre_update_backup();
    // update_with_yay();
    update_with_flatpak();
    // post_update_backup();
}