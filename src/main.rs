pub use libc;

fn main() {
    println!("Hello, world {}", libc::SIGUSR2);
}
