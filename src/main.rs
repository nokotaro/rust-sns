mod rest;

fn main() {
    if let Err(e) = rest::build() {
        eprintln!("Application startup failed: {:?}!", e);
        std::process::exit(1)
    }
}