//! rerun if migrations changed
fn main() {
    println!("cargo:rerun-if-changed=./migrations/");
}
