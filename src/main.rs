extern crate tree_sitter_cleva;

fn main() {
    let config: Args = Figment::new()
        .merge(Serialized::defaults(Args::parse()))
        .join(Toml::file("config.toml"))
        .merge(Env::prefixed("APP_"))
        .extract()
        .expect("Failed to load configs");

    let mut parser = tree_sitter::Parser::new();
    parser.set_language(&tree_sitter_cleva::language()).unwrap();

    let code = std::fs::read_to_string(config.file).expect("Failed to read file");

    let tree = parser.parse(code, None);
    println!("{:?}", tree);

    println!("compiler used:{:?}", config.cc);
}

use clap::Parser;
use figment::{
    providers::{Env, Format, Serialized, Toml},
    Figment,
};

use serde::{Deserialize, Serialize};
#[derive(Deserialize, Serialize, clap::Parser, Debug)]
struct Args {
    /// The input file
    file: String,
    /// c compiler to use
    #[clap(long, required = false)]
    #[serde(skip_serializing_if = "Option::is_none")]
    cc: Option<String>,
}
