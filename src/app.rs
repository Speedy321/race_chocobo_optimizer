#![allow(clippy::needless_return)]

mod chocobo;
mod choco_ui;
mod optimizer;

use chocobo::{Chocobo};
use choco_ui::{AddChocoboWindow, PairingWindow};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    // Example stuff:
    chocobos: Vec<Chocobo>,

    #[serde(skip)] // This how you opt-out of serialization of a field
    add_chocobo_window: AddChocoboWindow,
    #[serde(skip)] // This how you opt-out of serialization of a field
    pairing_window: PairingWindow
}

impl Default for TemplateApp {
    fn default() -> Self {
        let mut s = Self {
            chocobos: Vec::new(),
            add_chocobo_window: AddChocoboWindow::new(),
            pairing_window: PairingWindow::empty()
        };
        s.pairing_window.populate_from_chocobos(&s.chocobos);
        return s;
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        let mut s: Self;

        if let Some(storage) = cc.storage {
            s = eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        } else {
            s = Default::default();
        }

        s.pairing_window.populate_from_chocobos(&s.chocobos);
        return s;
    }
}

impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui
        let mut to_delete: Vec<usize> = Vec::new();

        if self.add_chocobo_window.open {
            self.add_chocobo_window.ui(ctx, &mut self.chocobos, &mut self.pairing_window);
        }

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::menu::bar(ui, |ui| {
                // NOTE: no File->Quit on web pages!
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.add_space(16.0);
                }

                egui::widgets::global_dark_light_mode_buttons(ui);
            });
        });
        egui::TopBottomPanel::bottom("footer").show(ctx, |ui| {
            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                ui.label("© 2024 Web App by Erdna. This project has no affiliation with Square Enix or anyone on the Final Fantasy XIV Team.");
                powered_by_egui_and_eframe(ui);
                egui::warn_if_debug_build(ui);
            });
        });
        egui::SidePanel::left("left_panel")
            .resizable(false)
            .exact_width(450.0)
            .show(ctx, |ui| {
                ui.heading("Chocobo Stables");
                
                let add_chocobo = ui.button("Add Chocobo");
                if add_chocobo.clicked() {
                    self.add_chocobo_window.open = true;
                };
 
                egui::ScrollArea::vertical().show(ui, |ui| {
                    for j in 0..(self.chocobos.len()/3 + 1){
                        ui.horizontal(|ui| {
                            for i in 0..3 {
                                let idx: usize = (j*3) + i;
                                if idx < self.chocobos.len(){
                                    ui.push_id(idx, |ui| {
                                        let delete = self.chocobos[idx].ui(ui, true);
                                        if delete {
                                            to_delete.push(idx);
                                        };
                                    });    
                                }
                            }
                        });
                    }
                });
            });
            
        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.heading("Best possible children: ");
            
            egui::ScrollArea::both()
                .auto_shrink(egui::Vec2b {x: false, y: false})
                .show(ui, |ui| {
                    self.pairing_window.ui(ui, 20);
            });
        });

        for i in to_delete {
            self.pairing_window.chocobo_removed(&self.chocobos[i]);
            self.chocobos.remove(i);
        }
    }
}

fn powered_by_egui_and_eframe(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("Powered by ");
        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        ui.label(" and ");
        ui.hyperlink_to(
            "eframe",
            "https://github.com/emilk/egui/tree/master/crates/eframe",
        );
        ui.label(".");
    });
}
