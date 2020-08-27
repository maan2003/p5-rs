mod ast;
mod gen;
mod types;
use std::io::prelude::*;
use std::fs::{self, File};
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let json = fs::read("data2.json")?;
    let json: ast::Ast = serde_json::from_slice(&json)?;
    let file = File::create("constants.rs")?;
    gen::gen_enums(&json, file);
    let global = File::create("global.rs")?;

    gen::gen_global_fn(&json, global);
    Ok(())
}
