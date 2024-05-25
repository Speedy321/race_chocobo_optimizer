#![allow(clippy::needless_return)]

use super::chocobo::{Chocobo, Gender, Pedigree, Ability, ChocoboColor};
use super::optimizer::{UniqChocoChild, generate_all_best_children, generate_new_best_children};

pub struct AddChocoboWindow {
    pub open: bool,
    
    temp_chocobo: Chocobo,
}
impl AddChocoboWindow {
    pub fn new() -> AddChocoboWindow{
        AddChocoboWindow {
            open: false,
            temp_chocobo: Chocobo::new(
                1,
                Gender::female, 
                Pedigree::new(), 
                Pedigree::new(), 
                Ability::choco_dash_1,
                10,
                ChocoboColor::unknown
            )
        }
    }
    pub fn ui(&mut self, ctx: &egui::Context, chocobo_list: &mut Vec<Chocobo>, pairings: &mut PairingWindow){
        egui::Window::new("New Chocobo stats")
            .auto_sized()
            .max_width(200.0)
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        ui.label("Rank/Grade: ");
                        ui.add(egui::DragValue::new(&mut self.temp_chocobo.grade)
                            .clamp_range(1..=9)
                        );
                    });
                    ui.horizontal(|ui| {
                        ui.label("Gender: ");
                        egui::ComboBox::from_id_source("chocobo_gender_picker")
                            .selected_text(self.temp_chocobo.gender.as_str())
                            .show_ui(ui, |ui| {
                                ui.selectable_value(&mut self.temp_chocobo.gender, Gender::female, Gender::female.as_str());
                                ui.selectable_value(&mut self.temp_chocobo.gender, Gender::male, Gender::male.as_str());
                            }
                        );
                    });
                    ui.horizontal(|ui| {
                        ui.label("Color: ");
                        egui::ComboBox::from_id_source("chocobo_color_picker")
                            .selected_text(self.temp_chocobo.color.value().name)
                            .show_ui(ui, |ui| {
                                for color in ChocoboColor::VALUES {
                                    let val = color.value();
                                    ui.selectable_value(
                                        &mut self.temp_chocobo.color, 
                                        color, 
                                        egui::RichText::new(val.name).background_color(
                                            egui::Color32::from_rgb(val.r, val.g, val.b)
                                        ).color(
                                            if ((val.r as f32) * 0.299 + (val.g as f32) * 0.587 + (val.b as f32) * 0.144) > 130.0 {
                                                egui::Color32::BLACK
                                            } else {
                                                egui::Color32::WHITE
                                            }
                                        )
                                    );
                                };
                            });
                        });
                    egui_extras::TableBuilder::new(ui)
                        .column(egui_extras::Column::exact(80.0))
                        .columns(egui_extras::Column::auto().clip(false), 2)
                        .striped(true)
                        .cell_layout(egui::Layout::default().with_cross_align(egui::Align::RIGHT))
                        .body(|body| {
                            body.rows(
                                15.0,
                                5,
                                |mut row| {
                                    match row.index() {
                                        0 => {
                                            row.col(|ui| {ui.label("Max Speed:");});
                                            row.col(|ui| {
                                                ui.add(egui::DragValue::new(&mut self.temp_chocobo.parents.0.speed)
                                                    .clamp_range(1..=4)
                                                );
                                            });
                                            row.col(|ui| {
                                                ui.add(egui::DragValue::new(&mut self.temp_chocobo.parents.1.speed)
                                                    .clamp_range(1..=4)
                                                );
                                            });
                                        }
                                        1 => {
                                            row.col(|ui| {ui.label("Acceleration:");});
                                            row.col(|ui| {
                                                ui.add(egui::DragValue::new(&mut self.temp_chocobo.parents.0.acceleration)
                                                    .clamp_range(1..=4)
                                                );
                                            });
                                            row.col(|ui| {
                                                ui.add(egui::DragValue::new(&mut self.temp_chocobo.parents.1.acceleration)
                                                    .clamp_range(1..=4)
                                                );
                                            });
                                        }
                                        2 => {
                                            row.col(|ui| {ui.label("Endurance:");});
                                            row.col(|ui| {
                                                ui.add(egui::DragValue::new(&mut self.temp_chocobo.parents.0.endurance)
                                                    .clamp_range(1..=4)
                                                );
                                            });
                                            row.col(|ui| {
                                                ui.add(egui::DragValue::new(&mut self.temp_chocobo.parents.1.endurance)
                                                    .clamp_range(1..=4)
                                                );
                                            });
                                        }
                                        3 => {
                                            row.col(|ui| {ui.label("Stamina:");});
                                            row.col(|ui| {
                                                ui.add(egui::DragValue::new(&mut self.temp_chocobo.parents.0.stamina)
                                                    .clamp_range(1..=4)
                                                );
                                            });
                                            row.col(|ui| {
                                                ui.add(egui::DragValue::new(&mut self.temp_chocobo.parents.1.stamina)
                                                    .clamp_range(1..=4)
                                                );
                                            });
                                        }
                                        4 => {
                                            row.col(|ui| {ui.label("Cunning:");});
                                            row.col(|ui| {
                                                ui.add(egui::DragValue::new(&mut self.temp_chocobo.parents.0.cunning)
                                                    .clamp_range(1..=4)
                                                );
                                            });
                                            row.col(|ui| {
                                                ui.add(egui::DragValue::new(&mut self.temp_chocobo.parents.1.cunning)
                                                    .clamp_range(1..=4)
                                                );
                                            });
                                        }
                                        _ => {
                                            row.col(|ui| {ui.label("???");});
                                        }
                                    };
                                }
                            )
                        });
                    ui.horizontal(|ui| {
                        ui.label("Breedings Left: ");
                        ui.add(egui::DragValue::new(&mut self.temp_chocobo.breeding_left)
                            .clamp_range(0..=10)
                        );
                    });
                    ui.horizontal(|ui| {
                        let add_btn = ui.button("Add");
                        if add_btn.clicked(){
                            pairings.chocobo_added(chocobo_list, &self.temp_chocobo);
                            chocobo_list.push(self.temp_chocobo);
                        }
                        let close_btn = ui.button("Close");
                        if close_btn.clicked(){
                            self.open = false;
                        }
                    });
                });
        });
    }
}

pub struct PairingWindow {
    children_list: Vec<UniqChocoChild>
}
impl PairingWindow {
    pub fn empty() -> Self {
        Self {
            children_list: Vec::new()
        }
    }

    pub fn populate_from_chocobos(&mut self, chocobos: &[Chocobo]) {
        self.children_list = generate_all_best_children(chocobos);
        self.children_list.sort_by(|a, b| {
            if a.star_score == b.star_score {
                b.avg_star_score.partial_cmp(&a.avg_star_score).unwrap()
            } else {
                b.star_score.cmp(&a.star_score)
            }
        });
    }

    pub fn chocobo_added(&mut self, old_chocobos: &[Chocobo], new_chocobo: &Chocobo) {
        generate_new_best_children(&mut self.children_list, old_chocobos, new_chocobo);
        self.children_list.sort_by(|a, b| {
            if a.star_score == b.star_score {
                b.avg_star_score.partial_cmp(&a.avg_star_score).unwrap()
            } else {
                b.star_score.cmp(&a.star_score)
            }
        });
    }

    pub fn chocobo_removed(&mut self, removed_chocobo: &Chocobo) {
        let mut to_remove: Vec<usize> = Vec::new();

        for idx in 0..self.children_list.len() {
            if (*removed_chocobo == self.children_list[idx].parents.0)
                || (*removed_chocobo == self.children_list[idx].parents.1) {
                    to_remove.push(idx);
            }
        }

        to_remove.sort_by(|a, b| b.cmp(a));
        for i in to_remove {
            self.children_list.remove(i);
        }
    }

    pub fn ui(&mut self, ui: &mut egui::Ui, max_pairs: usize) {
        ui.vertical(|ui| {
            for i in 0..max_pairs{
                if i < self.children_list.len(){
                    ui.push_id(i, |ui| {
                        ui.horizontal(|ui| {
                            ui.label(format!("{}: ", i));
                            ui.push_id(i*2 + 1, |ui| {
                                self.children_list[i].parents.0.ui(ui, false);
                            });
                            ui.label(" + ");
                            ui.push_id(i*3 + 2, |ui| {
                                self.children_list[i].parents.1.ui(ui, false);
                            });
                            ui.label(" => ");
                            ui.push_id(i*4 + 3, |ui| {
                                self.children_list[i].ui(ui, false);
                            });
                        });
                    });
                }
            }
        });
    }
}


