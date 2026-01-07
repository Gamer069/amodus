use std::path::Path;

use cargo_toml::Manifest;
use susio::Sus;

use crate::{eh, log};

pub fn build(v: bool, release: bool) {
	log!("Building mod...");
	
	let args = if release { vec!["build", "--release"] } else { vec!["build"] };

	let status = std::process::Command::new("cargo")
		.args(args)
		.status();

	eh!(status, "Failed to cargo build");

	if status.success() {
		log!("Build successful!");
	} else {
		eprintln!("Build failed");
		std::process::exit(-1);
	}

	package_sus(v, release);
}

pub fn package_sus(v: bool, release: bool) {
	let cargo_toml = std::fs::read_to_string("Cargo.toml");
	eh!(cargo_toml, "Failed to read Cargo.toml");

	let manifest = Manifest::from_str(&cargo_toml);
	eh!(manifest, "Failed to parse Cargo.toml");

	let wasm = if release { "target/wasm32-unknown-unknown/release/{{MOD_NAME}}.wasm" } else { "target/wasm32-unknown-unknown/debug/{{MOD_NAME}}.wasm" };
	let wasm = wasm.replace("{{MOD_NAME}}", manifest.package().name());
	let wasm = Path::new(&wasm);

	let mod_toml = Path::new("mod.toml");
	let res = Path::new("res");
	let out = Path::new("mod.sus");

	let sus = Sus::new();

	sus.construct(
		v, 
		wasm,
		mod_toml,
		Some(res),
		out
	);
}
