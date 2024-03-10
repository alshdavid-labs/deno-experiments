mod permissions;
mod create_snapshot;

fn main() {
    create_snapshot::create_snapshot();
    println!("{}", env!("CARGO_MANIFEST_DIR"));
}
