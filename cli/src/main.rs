pub mod new;
pub mod build;
pub mod install;
pub mod util;

use clap::{Parser, Subcommand};

#[derive(Parser, Clone)]
#[command(author="Illia Zhdanov", version="0.1", about="A powerful modding framework for Among Us, written in Rust.")]
pub struct Cli {
	#[command(subcommand)]
	command: Commands,

	#[arg(short, long)]
	verbose: bool,
}

#[derive(Subcommand, Clone)]
pub enum Commands {
	New {
		name: String,

		#[arg(short, long, default_value = "0.1.0")]
		version: String,

		#[arg(short, long, default_value = "2024")]
		edition: String
	},
	Build {
		#[arg(short, long)]
		release: bool,
	},
	Install
}

fn main() {
	let cli = Cli::parse();

	match cli.command {
		Commands::New { name, version, edition } => new::new(cli.verbose, name, version, edition),
		Commands::Build { release } => build::build(cli.verbose, release),
		Commands::Install => install::install(),
	}
}
