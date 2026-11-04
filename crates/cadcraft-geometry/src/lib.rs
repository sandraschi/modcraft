//! CADCraft Geometry & Parametric Modeling Engine
//!
//! Models CSG (Constructive Solid Geometry) primitives, 2D profile extrusions,
//! parametric constraints, and manifold boundary representations (B-Rep).

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Primitive3D {
    Cuboid { length_mm: f64, width_mm: f64, height_mm: f64 },
    Cylinder { radius_mm: f64, height_mm: f64 },
    Sphere { radius_mm: f64 },
    Extrusion { profile_points_mm: Vec<(f64, f64)>, depth_mm: f64 },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SolidBody {
    pub body_id: String,
    pub name: String,
    pub primitive: Primitive3D,
    pub volume_mm3: f64,
    pub surface_area_mm2: f64,
    pub is_manifold: bool,
}

pub struct GeometryEngine;

impl GeometryEngine {
    pub fn create_cuboid(id: &str, name: &str, l: f64, w: f64, h: f64) -> SolidBody {
        let volume = l * w * h;
        let surface_area = 2.0 * (l * w + l * h + w * h);
        SolidBody {
            body_id: id.to_string(),
            name: name.to_string(),
            primitive: Primitive3D::Cuboid { length_mm: l, width_mm: w, height_mm: h },
            volume_mm3: volume,
            surface_area_mm2: surface_area,
            is_manifold: true,
        }
    }

    pub fn create_cylinder(id: &str, name: &str, r: f64, h: f64) -> SolidBody {
        let volume = std::f64::consts::PI * r * r * h;
        let surface_area = 2.0 * std::f64::consts::PI * r * (r + h);
        SolidBody {
            body_id: id.to_string(),
            name: name.to_string(),
            primitive: Primitive3D::Cylinder { radius_mm: r, height_mm: h },
            volume_mm3: volume,
            surface_area_mm2: surface_area,
            is_manifold: true,
        }
    }
}
