/* 
    I wanna rewrite my own update script in rust
    I need to figure out how to execute shell commands in rust
*/

use std::process::Command;
use std::error::Error;

fn update_with_yay() {
    // println!("This will update your aur and pacman pkg's");
    Command::new("paru")
        // .arg("--noconfirm")
        .spawn()
        .expect("paru failed to start");
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

fn pre_update_backup() -> Result<(), Box<dyn Error>> {
    // println!("This will backup important files before updating");
    sudo::escalate_if_needed()?;
    Command::new("cp")
        .arg("/etc/fstab")
        .arg("/home/old/balls/")
        .spawn()
        .expect("cp failed to start");
    Command::new("cp")
        .arg("/etc/makepkg.conf")
        .arg("/home/old/balls/")
        .spawn()
        .expect("cp failed to start");
    Ok(())
}

fn post_update_backup() {
    println!("This will backup important files after updating");
}

fn main() {
    pre_update_backup();
    // update_with_yay();
    // update_with_flatpak();
    // post_update_backup();
}