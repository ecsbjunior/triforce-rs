use std::env;
use std::fs::File;
// use std::io::Write;
use std::path::Path;

fn main() {
  let dest = env::var("OUT_DIR").unwrap();
  let mut _file = File::create(&Path::new(&dest).join("bindings.rs")).unwrap();
}
