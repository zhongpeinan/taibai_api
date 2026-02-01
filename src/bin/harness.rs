//! CLI entry point for the test harness.

use clap::{Parser, Subcommand};
use std::io::{self, Read};
use taibai_api::harness;

#[derive(Parser)]
#[command(name = "harness", about = "Test harness for K8s type verification")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Apply defaults to a resource (reads JSON from stdin)
    Default {
        /// GVK identifier (e.g. "core/v1/Pod")
        gvk: String,
    },
    /// Perform conversion roundtrip (reads JSON from stdin)
    Convert {
        /// GVK identifier (e.g. "core/v1/Pod")
        gvk: String,
    },
    /// Validate a resource (reads JSON from stdin)
    Validate {
        /// GVK identifier (e.g. "core/v1/Pod")
        gvk: String,
    },
    /// Run full pipeline: default -> convert -> validate (reads JSON from stdin)
    Pipeline {
        /// GVK identifier (e.g. "core/v1/Pod")
        gvk: String,
    },
    /// List all registered GVKs
    List,
}

fn read_stdin() -> String {
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("failed to read stdin");
    input
}

fn main() {
    let cli = Cli::parse();

    let output = match cli.command {
        Command::List => serde_json::to_string_pretty(&harness::list_registered_gvks()),
        Command::Default { gvk } => {
            let input = read_stdin();
            match harness::apply_defaults(&gvk, &input) {
                Ok(r) => serde_json::to_string_pretty(&r),
                Err(e) => serde_json::to_string_pretty(&e),
            }
        }
        Command::Convert { gvk } => {
            let input = read_stdin();
            match harness::convert_roundtrip(&gvk, &input) {
                Ok(r) => serde_json::to_string_pretty(&r),
                Err(e) => serde_json::to_string_pretty(&e),
            }
        }
        Command::Validate { gvk } => {
            let input = read_stdin();
            match harness::validate(&gvk, &input) {
                Ok(r) => serde_json::to_string_pretty(&r),
                Err(e) => serde_json::to_string_pretty(&e),
            }
        }
        Command::Pipeline { gvk } => {
            let input = read_stdin();
            match harness::full_pipeline(&gvk, &input) {
                Ok(r) => serde_json::to_string_pretty(&r),
                Err(e) => serde_json::to_string_pretty(&e),
            }
        }
    };

    println!("{}", output.expect("failed to serialize output"));
}
