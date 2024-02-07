use std::process::Command;


fn main() {
    println!("Starting Update");
    let update = String::from("yes | sudo pacman -Sy && sudo powerpill -Su --noconfirm && pikaur -Su --noconfirm --devel && sudo flatpak update && rustup update");
    let output = Command::new("sh").arg("-c").arg(update).output();
    println!("Done");
    println!("{:?}", output)
}
