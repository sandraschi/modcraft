use modcraft_core::ModEngine;
use modcraft_geometry::GeometryEngine;

#[test]
fn test_modcraft_primitives() {
    let cube = GeometryEngine::create_cuboid("C-1", "Test Cube", 10.0, 10.0, 10.0);
    assert_eq!(cube.volume_mm3, 1000.0);
    assert_eq!(cube.surface_area_mm2, 600.0);
    assert!(cube.is_manifold);

    let cyl = GeometryEngine::create_cylinder("CYL-1", "Test Cylinder", 10.0, 50.0);
    assert!(cyl.volume_mm3 > 15000.0);
    assert!(cyl.is_manifold);
}

#[test]
fn test_modcraft_bom_generation() {
    let engine = ModEngine::new();
    let bom = engine.generate_bom().expect("bom generation");
    assert_eq!(bom.total_components, 2);
    assert!(bom.total_mass_kg > 0.0);
}
