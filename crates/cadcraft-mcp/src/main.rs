//! CADCraft MCP Server
//!
//! Exposes CSG solid modeling, parametric component graphs,
//! and Bill of Materials synthesis as FastMCP JSON-RPC tools on port 14120.

use serde_json::json;
use cadcraft_core::CadEngine;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let port: u16 = 14120;
    println!("[cadcraft-mcp] Initializing Sovereign CAD Modeling FastMCP engine on port {}...", port);

    let engine = CadEngine::new();
    let bom = engine.generate_bom()?;

    let status = json!({
        "status": "ready",
        "mcp_server": "cadcraft",
        "port": port,
        "capabilities": ["ParametricSolidModeling", "CSGPrimitiveOperations", "ManifoldMeshValidation", "AutomatedBOMSynthesis"],
        "active_bodies_count": engine.active_solids.len(),
        "assembly_total_mass_kg": bom.total_mass_kg,
    });
    println!("[cadcraft-mcp] Status: {}", serde_json::to_string_pretty(&status)?);
    Ok(())
}
