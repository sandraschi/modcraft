# CADCraft

**CADCraft** is an ArtCraft-tier sovereign parametric 3D CAD, constructive solid geometry (CSG), and automated Bill of Materials (BOM) synthesis engine running natively on Goliath with zero cloud dependencies.

Features:
- **Parametric Solid Modeling**: Cuboids, cylinders, spheres, and 2D profile extrusions with volume and surface area calculations.
- **Manifold Mesh Validation**: B-Rep boundary integrity checking to prevent non-manifold slicing errors.
- **Automated Bill of Materials (BOM)**: Real-time component roll-ups with volumetric mass estimation (Aluminium 6061, Carbon Steel 1045, POM/Delrin).
- **FastMCP Protocol**: Exposed on port `14120` for multi-agent mechanical design swarms.

---

## Workspace Architecture

1. `crates/cadcraft-geometry`: CSG geometry primitives, mesh volume/surface integrators, and manifold validators.
2. `crates/cadcraft-core`: Assembly feature graphs, BOM synthesizer, and DatabaseCraft persistence.
3. `crates/cadcraft-mcp`: FastMCP JSON-RPC server on port `14120`.
4. `crates/cadcraft-app`: Native GUI (`eframe`/`egui`) and CLI tool (`cadcraft-cli`).

---

## Verification & Usage

```powershell
# Run test suite
cargo test

# Headless smoke test
./target/debug/cadcraft.exe --smoke

# CLI Operations
./target/debug/cadcraft-cli.exe solids
./target/debug/cadcraft-cli.exe bom
./target/debug/cadcraft-cli.exe cube "Enclosure Box" 120.0 80.0 40.0
```
