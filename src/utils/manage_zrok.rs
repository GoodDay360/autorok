
use whoami::{platform, arch};

pub fn check_install() {
    println!("Checking if zrok is installed");
    println!("Platform: {}", platform());
    println!("Arch: {}", arch());
}

pub fn download() {
    println!("Downloading zrok");
    println!("Platform: {}", platform());
    println!("Arch: {}", arch());

}