use clap::Parser;
use serde_json::json;
use modcraft_core::ModEngine;

#[derive(Parser, Debug)]
#[command(name = "modcraft", about = "ModCraft Parametric Solid Modeling Studio")]
struct Args {
    #[arg(long)]
    smoke: bool,
}

pub struct ModCraftApp {
    engine: ModEngine,
    status_msg: String,
}

impl ModCraftApp {
    pub fn new() -> Self {
        Self {
            engine: ModEngine::new(),
            status_msg: "Parametric ModCraft Engine Ready (Port 14120)".to_string(),
        }
    }
}

impl eframe::App for ModCraftApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::Panel::top("top_panel").show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.heading("📐 ModCraft — Sovereign Parametric 3D Solid Studio");
                ui.separator();
                ui.label(&self.status_msg);
            });
        });

        egui::Panel::left("assembly_tree").show(ui, |ui| {
            ui.heading("Assembly Feature Tree");
            for solid in &self.engine.active_solids {
                ui.group(|ui| {
                    ui.label(egui::RichText::new(&solid.name).strong());
                    ui.label(format!("ID: {}", solid.body_id));
                    ui.label(format!("Volume: {:.1} mm³", solid.volume_mm3));
                    ui.label(format!("Surface: {:.1} mm²", solid.surface_area_mm2));
                    let manifold_txt = if solid.is_manifold { "Manifold: VALID" } else { "Manifold: ERROR" };
                    ui.label(manifold_txt);
                });
            }
        });

        egui::CentralPanel::default().show(ui, |ui| {
            ui.heading("Bill of Materials (BOM) & Mass Estimation");
            if let Ok(bom) = self.engine.generate_bom() {
                ui.label(format!("Assembly: {}", bom.assembly_name));
                ui.label(format!("Total Parts: {} | Total Mass: {:.3} kg", bom.total_components, bom.total_mass_kg));
                ui.separator();

                for item in bom.items {
                    ui.horizontal(|ui| {
                        ui.label(format!("• {}", item.solid.name));
                        ui.label(format!("Qty: {}", item.quantity));
                        ui.label(format!("Mat: {}", item.material));
                        ui.label(format!("Unit Mass: {:.3} kg", item.unit_mass_kg));
                    });
                }
            }
        });
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    if args.smoke {
        println!("[modcraft] Running headless smoke verification...");
        let engine = ModEngine::new();
        let bom = engine.generate_bom()?;
        let smoke_payload = json!({
            "app": "modcraft",
            "version": "0.1.0",
            "status": "ready",
            "control_port": 14120,
            "success": true,
            "bodies_count": engine.active_solids.len(),
            "total_mass_kg": bom.total_mass_kg,
            "capabilities": ["ParametricSolidModeling", "CSGPrimitiveOperations", "ManifoldMeshValidation", "AutomatedBOMSynthesis"]
        });
        println!("[modcraft] Smoke OK: {}", smoke_payload);
        return Ok(());
    }

    let app = ModCraftApp::new();
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1280.0, 800.0])
            .with_title("ModCraft — Parametric Solid Studio"),
        ..Default::default()
    };

    eframe::run_native(
        "ModCraft",
        native_options,
        Box::new(|_cc| Ok(Box::new(app))),
    )?;
    Ok(())
}
