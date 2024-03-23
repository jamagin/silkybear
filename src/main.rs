use std::env;
use fork::{fork, Fork};


fn main() {
    let args: Vec<String> = env::args().collect();
    if let Ok(Fork::Child) = fork() {
        let _err = exec::Command::new(&args[1]).args(&args[2..]).exec();
    }
}
