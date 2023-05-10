mod driver;
mod error;
mod gateway;
mod port;
mod rest;
mod usecase;
mod utils;

fn main() {
    if let Err(e) = rest::build() {
        eprintln!("Application startup failed: {:?}!", e);
        std::process::exit(1)
    }
}
