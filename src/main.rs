/* wanna rewrite my own update script in rust
 * I need to figure out how to execute shell commands in rust
*/

use std::process::Command;

fn update_with_yay() {
    println!("This will update your aur and pacman pkg's");
}

fn update_with_flatpak() {
    println!("This will update your flatpaks");
}

fn pre_update_backup() {
    println!("This will backup important files before updating");
}

fn post_update_backup() {
    println!("This will backup important files after updating");
}

fn main() {
    pre_update_backup();
    update_with_yay();
    update_with_flatpak();
    post_update_backup();
}