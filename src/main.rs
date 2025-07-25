fn main() {
    if let Err(e) = axec::run() {
        eprintln!("{e}");
    }
}
