mod permissions;
mod create_snapshot;
mod runtime;

fn main() {
  create_snapshot::create_snapshot();
  println!("{}", env!("CARGO_MANIFEST_DIR"));
}
