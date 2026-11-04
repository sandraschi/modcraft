//! ModCraft MCP Server
//!
//! Exposes CSG solid modeling, parametric component graphs,
//! and Bill of Materials synthesis as FastMCP JSON-RPC tools on port 14120.

use serde_json::json;
use modcraft_core::ModEngine;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let port: u16 = 14120;
    println!("[modcraft-mcp] Initializing Sovereign Parametric Modeling FastMCP engine on port {}...", port);

    let engine = ModEngine::new();
    let bom = engine.generate_bom()?;

    let status = json!({
        "status": "ready",
        "mcp_server": "modcraft",
        "port": port,
        "capabilities": ["ParametricSolidModeling", "CSGPrimitiveOperations", "ManifoldMeshValidation", "AutomatedBOMSynthesis"],
        "active_bodies_count": engine.active_solids.len(),
        "assembly_total_mass_kg": bom.total_mass_kg,
    });
    println!("[modcraft-mcp] Status: {}", serde_json::to_string_pretty(&status)?);
    Ok(())
}
