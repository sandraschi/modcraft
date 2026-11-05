# ModCraft

**ModCraft** is an ArtCraft-tier local-first parametric 3D solid modeling, constructive solid geometry (CSG), and automated Bill of Materials (BOM) synthesis engine running natively on Goliath with zero cloud dependencies.

Features:
- **Parametric Solid Modeling**: Cuboids, cylinders, spheres, and 2D profile extrusions with volume and surface area calculations.
- **Manifold Mesh Validation**: B-Rep boundary integrity checking to prevent non-manifold slicing errors.
- **Automated Bill of Materials (BOM)**: Real-time component roll-ups with volumetric mass estimation (Aluminium 6061, Carbon Steel 1045, POM/Delrin).
- **FastMCP Protocol**: Exposed on port `14120` for multi-agent mechanical design swarms.

---

## Workspace Architecture

1. `crates/modcraft-geometry`: CSG geometry primitives, mesh volume/surface integrators, and manifold validators.
2. `crates/modcraft-core`: Assembly feature graphs, BOM synthesizer, and DatabaseCraft persistence.
3. `crates/modcraft-mcp`: FastMCP JSON-RPC server on port `14120`.
4. `crates/modcraft-app`: Native GUI (`eframe`/`egui`) and CLI tool (`modcraft-cli`).

---

## Verification & Usage

```powershell
# Run test suite
cargo test

# Headless smoke test
./target/debug/modcraft.exe --smoke

# CLI Operations
./target/debug/modcraft-cli.exe solids
./target/debug/modcraft-cli.exe bom
./target/debug/modcraft-cli.exe cube "Enclosure Box" 120.0 80.0 40.0
```

## Status — does today / next week / non-goals

**Does today:** 4-crate Rust workspace (geometry/core split), CSG + BOM
verbs declared, 2 test fns.

**Next week:** kernel build-vs-wrap decision + SolidCraft relationship in
writing; primitive-volume + boolean-union tests; primitive-bom recipe.

**Non-goals:** no 2D drafting or PCB at v1 (solids → BOM first).

## Lineage — the Storytold craft-app pattern

Storytold ArtCraft ([storytold/artcraft](https://github.com/storytold/artcraft) — "the IDE for artists") was the starting point. The 16 craft apps in the Storytold stable each ship one binary that is both the human CLI and the agent MCP server (stdio), headless-first, with `run.json` + report artifacts. That modus operandi and harness is what this repo implements; the code here is independent, the pattern is shared.
