use clap::{Parser, Subcommand};
use cadcraft_core::CadEngine;
use cadcraft_geometry::GeometryEngine;

#[derive(Parser)]
#[command(name = "cadcraft-cli", about = "CADCraft Sovereign Parametric CAD & BOM CLI")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// List active solid bodies in assembly
    Solids,
    /// Generate Bill of Materials for assembly
    Bom,
    /// Create cuboid primitive
    Cube {
        name: String,
        length: f64,
        width: f64,
        height: f64,
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let engine = CadEngine::new();

    match cli.command {
        Commands::Solids => {
            println!("{}", serde_json::to_string_pretty(&engine.active_solids)?);
        }
        Commands::Bom => {
            let bom = engine.generate_bom()?;
            println!("{}", serde_json::to_string_pretty(&bom)?);
        }
        Commands::Cube { name, length, width, height } => {
            let cube = GeometryEngine::create_cuboid("USER-CUBE", &name, length, width, height);
            println!("{}", serde_json::to_string_pretty(&cube)?);
        }
    }
    Ok(())
}
