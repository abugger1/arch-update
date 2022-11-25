use std::process::Command;


fn main() {
    println!("Starting Update");
    let update = String::from("yes | sudo pacman -Syu");
    let output = Command::new("sh").arg("-c").arg(update).output();
    println!("Done");
    println!("{:?}", output)
}
