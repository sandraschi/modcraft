//! ModCraft Core Engine
//!
//! Provides parametric assembly graphs, BOM (Bill of Materials) generation,
//! and design version tracking backed by DatabaseCraft.

use serde::{Deserialize, Serialize};
use thiserror::Error;
use modcraft_geometry::{GeometryEngine, SolidBody};

#[derive(Error, Debug)]
pub enum ModError {
    #[error("Geometry error: {0}")]
    Geometry(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssemblyComponent {
    pub solid: SolidBody,
    pub quantity: u32,
    pub material: String,
    pub unit_mass_kg: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BillOfMaterials {
    pub assembly_name: String,
    pub total_components: u32,
    pub total_mass_kg: f64,
    pub items: Vec<AssemblyComponent>,
}

pub struct ModEngine {
    pub active_solids: Vec<SolidBody>,
}

impl ModEngine {
    pub fn new() -> Self {
        let mounting_bracket = GeometryEngine::create_cuboid("BRACKET-01", "Aluminium NEMA23 Bracket", 80.0, 60.0, 8.0);
        let guide_shaft = GeometryEngine::create_cylinder("SHAFT-01", "Precision Linear Rod", 6.0, 300.0);

        Self {
            active_solids: vec![mounting_bracket, guide_shaft],
        }
    }

    pub fn generate_bom(&self) -> Result<BillOfMaterials, ModError> {
        let mut items = Vec::new();
        let mut total_mass = 0.0;
        let mut total_count = 0;

        for solid in &self.active_solids {
            let (mat, density) = if solid.name.contains("Aluminium") {
                ("Aluminium 6061-T6".to_string(), 0.0000027)
            } else {
                ("Chrome Carbon Steel 1045".to_string(), 0.00000785)
            };

            let mass_kg = solid.volume_mm3 * density;
            total_mass += mass_kg;
            total_count += 1;

            items.push(AssemblyComponent {
                solid: solid.clone(),
                quantity: 1,
                material: mat,
                unit_mass_kg: mass_kg,
            });
        }

        Ok(BillOfMaterials {
            assembly_name: "Cartesian Gantry Stage Assembly".to_string(),
            total_components: total_count,
            total_mass_kg: total_mass,
            items,
        })
    }
}
