use std::{env, path::Path, fs::File, io::Write};

use chrome_devtools_bindgen::generate_protocol_bindings;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let protocol_path = Path::new(&out_dir).join("__protocol.rs");
    let mut f = File::create(protocol_path).unwrap();

    let source_code = generate_protocol_bindings();

    f.write_all(source_code.as_bytes()).expect("Not writeable!");
}