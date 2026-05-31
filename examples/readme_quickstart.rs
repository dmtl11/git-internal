//! README Quickstart: decode a pack file and print each object.
//!
//! ```powershell
//! # 1) Download test pack (once)
//! Invoke-WebRequest -Uri "https://download.libra.tools/libra/development/pack/small-sha1.pack" `
//!   -OutFile "tests/data/packs/small-sha1.pack"
//!
//! # 2) Run
//! cargo run --example readme_quickstart
//! ```

use std::{fs::File, io::BufReader, path::Path};

use git_internal::{
    hash::{HashKind, set_hash_kind},
    internal::pack::Pack,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    set_hash_kind(HashKind::Sha1);

    let pack_path = "tests/data/packs/small-sha1.pack";
    if !Path::new(pack_path).exists() {
        eprintln!("Pack file not found: {pack_path}");
        eprintln!("Download it first, for example:");
        eprintln!(
            "  Invoke-WebRequest -Uri \"https://download.libra.tools/libra/development/pack/small-sha1.pack\" -OutFile \"{pack_path}\""
        );
        std::process::exit(1);
    }

    let f = File::open(pack_path)?;
    let mut reader = BufReader::new(f);
    let mut pack = Pack::new(None, Some(64 * 1024 * 1024), None, true);

    println!("Decoding {pack_path} ...");
    pack.decode(
        &mut reader,
        |entry| {
            println!(
                "  object: {} | type: {:?}",
                entry.inner.hash, entry.inner.obj_type
            );
        },
        None::<fn(git_internal::hash::ObjectHash)>,
    )?;
    println!("Done. Total objects: {}", pack.number);
    Ok(())
}
