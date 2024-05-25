#![allow(clippy::needless_return)]

use std::cmp;
use egui::Color32;

use super::chocobo::{Chocobo, Gender, Ability, ChocoboColor};

#[derive(serde::Deserialize, serde::Serialize, PartialEq, Copy, Clone)]
pub struct FuturePedigree {
    // Stats are (value, chance)
    pub speed: (i8, f32),
    pub acceleration: (i8, f32),
    pub endurance: (i8, f32),
    pub stamina: (i8, f32),
    pub cunning: (i8, f32),
}
impl FuturePedigree {
    pub fn as_array(&self) -> [(i8, f32); 5] {
        [self.speed, self.acceleration, self.endurance, self.stamina, self.cunning]
    }
}

pub struct UniqChocoChild {
    pub grade: i8,
    pub color: ChocoboColor,
    pub gender: Gender,
    pub breeding_left: i8,

    pub pedigrees: (FuturePedigree, FuturePedigree),

    pub ability: Ability,
    pub chance: f32,
    pub star_score: i8,
    pub avg_star_score: f32,

    pub parents: (Chocobo, Chocobo)
    //prefered_weather: String = "TODO"
}
impl UniqChocoChild {

     // returns true is the current chocobo needs to be deleted from the list.
     pub fn ui(&mut self, ui: &mut egui::Ui, in_list: bool) -> bool {
        let col_val = self.color.value();
        let mut delete = false;

        egui::Frame::none()
            .stroke(egui::Stroke::new(2.0, Color32::GRAY))
            .rounding(egui::Rounding::same(3.0))
            .inner_margin(egui::Margin::same(10.0))
            .outer_margin(egui::Margin::same(2.5))
            .show(ui, |ui| {
                ui.vertical(|ui| {
                    ui.set_width(200.0);
                    ui.set_height(200.0);
                    ui.horizontal(|ui| {
                        ui.vertical(|ui| {
                            ui.label(format!("G{} {}", self.grade, self.gender.as_str()));
                            ui.label(egui::RichText::new(col_val.name)
                                .background_color(
                                    egui::Color32::from_rgb(col_val.r, col_val.g, col_val.b)
                                ).color(
                                    //if (red*0.299 + green*0.587 + blue*0.114) > 186 use #000000 else use #ffffff
                                    if ((col_val.r as f32) * 0.299 + (col_val.g as f32) * 0.587 + (col_val.b as f32) * 0.144) > 130.0 {
                                        egui::Color32::BLACK
                                    } else {
                                        egui::Color32::WHITE
                                    }
                                )
                            );
                            ui.label(format!("Stats: Max: {}, {}%, Avg: {}", self.star_score, (self.chance*100.0), self.avg_star_score));
                        });
                        if in_list{
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                                let delete_btn = ui.button("❌");
                                if delete_btn.clicked(){
                                    delete = true;
                                }
                            });
                        }
                    });
                    ui.separator();
                    ui.label("Pedigrees:");
                    ui.horizontal(|ui| {
                        egui_extras::TableBuilder::new(ui)
                        .column(egui_extras::Column::exact(80.0))
                        .column(egui_extras::Column::exact(5.0))
                        .column(egui_extras::Column::exact(32.0))
                        .column(egui_extras::Column::exact(5.0))
                        .column(egui_extras::Column::exact(32.0))
                        //.columns(egui_extras::Column::auto().clip(false), 4)
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
                                            row.col(|ui| {ui.label(format!("{}", self.pedigrees.0.speed.0));});
                                            row.col(|ui| {ui.label(format!("{}%", ((self.pedigrees.0.speed.1)*100.0)));});
                                            row.col(|ui| {ui.label(format!("{}", self.pedigrees.1.speed.0));});
                                            row.col(|ui| {ui.label(format!("{}%", ((self.pedigrees.1.speed.1)*100.0)));});
                                        }
                                        1 => {
                                            row.col(|ui| {ui.label("Acceleration:");});
                                            row.col(|ui| {ui.label(format!("{}", self.pedigrees.0.acceleration.0));});
                                            row.col(|ui| {ui.label(format!("{}%", ((self.pedigrees.0.acceleration.1)*100.0)));});
                                            row.col(|ui| {ui.label(format!("{}", self.pedigrees.1.acceleration.0));});
                                            row.col(|ui| {ui.label(format!("{}%", ((self.pedigrees.1.acceleration.1)*100.0)));});
                                        }
                                        2 => {
                                            row.col(|ui| {ui.label("Endurance:");});
                                            row.col(|ui| {ui.label(format!("{}", self.pedigrees.0.endurance.0));});
                                            row.col(|ui| {ui.label(format!("{}%", ((self.pedigrees.0.endurance.1)*100.0)));});
                                            row.col(|ui| {ui.label(format!("{}", self.pedigrees.1.endurance.0));});
                                            row.col(|ui| {ui.label(format!("{}%", ((self.pedigrees.1.endurance.1)*100.0)));});
                                        }
                                        3 => {
                                            row.col(|ui| {ui.label("Stamina:");});
                                            row.col(|ui| {ui.label(format!("{}", self.pedigrees.0.stamina.0));});
                                            row.col(|ui| {ui.label(format!("{}%", ((self.pedigrees.0.stamina.1)*100.0)));});
                                            row.col(|ui| {ui.label(format!("{}", self.pedigrees.1.stamina.0));});
                                            row.col(|ui| {ui.label(format!("{}%", ((self.pedigrees.1.stamina.1)*100.0)));});
                                        }
                                        4 => {
                                            row.col(|ui| {ui.label("Cunning:");});
                                            row.col(|ui| {ui.label(format!("{}", self.pedigrees.0.cunning.0));});
                                            row.col(|ui| {ui.label(format!("{}%", ((self.pedigrees.0.cunning.1)*100.0)));});
                                            row.col(|ui| {ui.label(format!("{}", self.pedigrees.1.cunning.0));});
                                            row.col(|ui| {ui.label(format!("{}%", ((self.pedigrees.1.cunning.1)*100.0)));});
                                        }
                                        _ => {
                                            row.col(|ui| {ui.label("???");});
                                        }
                                    };
                                }
                            )
                        });
                    });
                    ui.label(format!("Available Breeds: {}", self.breeding_left));
                });
            });
        
        return delete;
    }
}

// returns the tuple (best stat, chance of perfect) from any two stat values
fn get_best_stat(stat1: i8, stat2: i8) -> (i8, f32){
    let stat: i8 = cmp::max(stat1, stat2);
    let mut chance: f32 = 0.0;
    if stat == 4 {
        chance =  [stat1, stat2].iter().filter(|&n| *n == 4).count() as f32 / 2.0;
    }
    return (stat, chance);
}

fn get_best_child(parent1: &Chocobo, parent2: &Chocobo) -> UniqChocoChild {
    let child_grade = cmp::min(parent1.grade, parent2.grade) + 1;
    
    let pedi1: FuturePedigree = FuturePedigree {
        speed: get_best_stat(parent1.parents.0.speed, parent1.parents.1.speed),
        acceleration: get_best_stat(parent1.parents.0.acceleration, parent1.parents.1.acceleration),
        endurance: get_best_stat(parent1.parents.0.endurance, parent1.parents.1.endurance),
        stamina: get_best_stat(parent1.parents.0.stamina, parent1.parents.1.stamina),
        cunning: get_best_stat(parent1.parents.0.cunning, parent1.parents.1.cunning)
    };
    let pedi2: FuturePedigree = FuturePedigree {
        speed: get_best_stat(parent2.parents.0.speed, parent2.parents.1.speed),
        acceleration: get_best_stat(parent2.parents.0.acceleration, parent2.parents.1.acceleration),
        endurance: get_best_stat(parent2.parents.0.endurance, parent2.parents.1.endurance),
        stamina: get_best_stat(parent2.parents.0.stamina, parent2.parents.1.stamina),
        cunning: get_best_stat(parent2.parents.0.cunning, parent2.parents.1.cunning)
    };

    let mut perfect_count = 0; // number of stats at 4 stars
    let mut child_chance: f32 = 0.0;
    let mut avg_score: f32 = 0.0;
    for stat in pedi1.as_array() {
        if stat.0 == 4 {
            perfect_count += 1;
            if child_chance == 0.0 {child_chance = stat.1;}
            else {child_chance *= stat.1;}
            avg_score += stat.1;
        }
    }
    for stat in pedi2.as_array() {
        if stat.0 == 4 {
            perfect_count += 1;
            if child_chance == 0.0 {child_chance = stat.1;}
            else {child_chance *= stat.1;}
            avg_score += stat.1;
        }
    }

    UniqChocoChild {
        grade: child_grade,
        color: ChocoboColor::unknown,
        gender: Gender::unknown,
        breeding_left: 10,
        pedigrees: (pedi1, pedi2),
        ability: parent1.ability,
        chance: child_chance,
        star_score: perfect_count,
        avg_star_score: avg_score,
        parents: (*parent1, *parent2)
    }
}

pub fn generate_all_best_children(parents: &[Chocobo]) -> Vec<UniqChocoChild> {
    let mut children = Vec::new();
    
    let females: Vec<&Chocobo> = parents.iter().filter(|&p| p.gender == Gender::female).collect();
    let males: Vec<&Chocobo> = parents.iter().filter(|&p| p.gender == Gender::male).collect();

    for fem in females {
        for mal in &males {
            children.push(get_best_child(fem, mal));
        }
    }

    return children;
}

pub fn generate_new_best_children(children: &mut Vec<UniqChocoChild>, previous_parents: &[Chocobo], new_parent: &Chocobo) {
    let females: Vec<&Chocobo> = previous_parents.iter().filter(|&p| p.gender == Gender::female).collect();
    let males: Vec<&Chocobo> = previous_parents.iter().filter(|&p| p.gender == Gender::male).collect();

    if new_parent.gender == Gender::female {
        for mal in males {
            children.push(get_best_child(new_parent, mal));
        }
    } else if new_parent.gender == Gender::male {
        for fem in females {
            children.push(get_best_child(fem, new_parent));
        }
    }
}