#![allow(clippy::needless_return)]

use egui::Color32;

#[derive(serde::Deserialize, serde::Serialize, PartialEq)]
pub struct Color{
    pub name: String,
    pub r: u8,
    pub g: u8,
    pub b: u8
}

#[allow(non_camel_case_types)]
#[derive(serde::Deserialize, serde::Serialize, PartialEq, Copy, Clone)]
pub enum ChocoboColor {
    unknown,
    snow_white,
    ash_grey,
    goobbue_grey,
    slate_grey,
    charcoal_grey,
    soot_black,
    rose_pink,
    lilac_purple,
    rolandberry_red,
    dalamud_red,
    rust_red,
    wine_red,
    coral_pink,
    blood_red,
    salmon_pink,
    sunset_orange,
    mesa_red,
    bark_brown,
    chocolat_brown,
    russet_brown,
    kobold_brown,
    cork_brown,
    qiqim_brown,
    opo_opo_brown,
    aldgoat_brown,
    pumpkin_orange,
    acorn_brown,
    orchard_brown,
    chestnut_brown,
    gobbiebag_brown,
    shale_brown,
    mole_brown,
    loam_brown,
    bone_white,
    ul_brown,
    desert_yellow,
    honey_yellow,
    millioncorn_yellow,
    coeurl_yellow,
    cream_yellow,
    halatali_yellow,
    raisin_brown,
    mud_green,
    sylph_green,
    lime_green,
    moss_green,
    meadow_green,
    olive_green,
    marsh_green,
    apple_green,
    cactuar_green,
    hunter_green,
    ochu_green,
    adamantoise_green,
    nophica_green,
    deepwood_green,
    celeste_green,
    turquoise_green,
    morbol_green,
    ice_blue,
    sky_blue,
    seafog_blue,
    peacock_blue,
    rhotano_blue,
    corpse_blue,
    ceruleam_blue,
    woad_blue,
    ink_blue,
    raptor_blue,
    othard_blue,
    storm_blue,
    void_blue,
    royal_blue,
    midnight_blue,
    shadow_blue,
    abyssal_blue,
    lavender_purple,
    gloom_purple,
    currant_purple,
    iris_purple,
    grape_purple,
    lotus_pink,
    colibri_pink,
    plum_purple,
    regal_purple
}
impl ChocoboColor {
    pub const VALUES: [Self; 86] = [
        ChocoboColor::unknown,
        ChocoboColor::snow_white,
        ChocoboColor::ash_grey,
        ChocoboColor::goobbue_grey,
        ChocoboColor::slate_grey,
        ChocoboColor::charcoal_grey,
        ChocoboColor::soot_black,
        ChocoboColor::rose_pink,
        ChocoboColor::lilac_purple,
        ChocoboColor::rolandberry_red,
        ChocoboColor::dalamud_red,
        ChocoboColor::rust_red,
        ChocoboColor::wine_red,
        ChocoboColor::coral_pink,
        ChocoboColor::blood_red,
        ChocoboColor::salmon_pink,
        ChocoboColor::sunset_orange,
        ChocoboColor::mesa_red,
        ChocoboColor::bark_brown,
        ChocoboColor::chocolat_brown,
        ChocoboColor::russet_brown,
        ChocoboColor::kobold_brown,
        ChocoboColor::cork_brown,
        ChocoboColor::qiqim_brown,
        ChocoboColor::opo_opo_brown,
        ChocoboColor::aldgoat_brown,
        ChocoboColor::pumpkin_orange,
        ChocoboColor::acorn_brown,
        ChocoboColor::orchard_brown,
        ChocoboColor::chestnut_brown,
        ChocoboColor::gobbiebag_brown,
        ChocoboColor::shale_brown,
        ChocoboColor::mole_brown,
        ChocoboColor::loam_brown,
        ChocoboColor::bone_white,
        ChocoboColor::ul_brown,
        ChocoboColor::desert_yellow,
        ChocoboColor::honey_yellow,
        ChocoboColor::millioncorn_yellow,
        ChocoboColor::coeurl_yellow,
        ChocoboColor::cream_yellow,
        ChocoboColor::halatali_yellow,
        ChocoboColor::raisin_brown,
        ChocoboColor::mud_green,
        ChocoboColor::sylph_green,
        ChocoboColor::lime_green,
        ChocoboColor::moss_green,
        ChocoboColor::meadow_green,
        ChocoboColor::olive_green,
        ChocoboColor::marsh_green,
        ChocoboColor::apple_green,
        ChocoboColor::cactuar_green,
        ChocoboColor::hunter_green,
        ChocoboColor::ochu_green,
        ChocoboColor::adamantoise_green,
        ChocoboColor::nophica_green,
        ChocoboColor::deepwood_green,
        ChocoboColor::celeste_green,
        ChocoboColor::turquoise_green,
        ChocoboColor::morbol_green,
        ChocoboColor::ice_blue,
        ChocoboColor::sky_blue,
        ChocoboColor::seafog_blue,
        ChocoboColor::peacock_blue,
        ChocoboColor::rhotano_blue,
        ChocoboColor::corpse_blue,
        ChocoboColor::ceruleam_blue,
        ChocoboColor::woad_blue,
        ChocoboColor::ink_blue,
        ChocoboColor::raptor_blue,
        ChocoboColor::othard_blue,
        ChocoboColor::storm_blue,
        ChocoboColor::void_blue,
        ChocoboColor::royal_blue,
        ChocoboColor::midnight_blue,
        ChocoboColor::shadow_blue,
        ChocoboColor::abyssal_blue,
        ChocoboColor::lavender_purple,
        ChocoboColor::gloom_purple,
        ChocoboColor::currant_purple,
        ChocoboColor::iris_purple,
        ChocoboColor::grape_purple,
        ChocoboColor::lotus_pink,
        ChocoboColor::colibri_pink,
        ChocoboColor::plum_purple,
        ChocoboColor::regal_purple
    ];
    pub fn value(&self) -> Color{
        match self{
            ChocoboColor::unknown => Color{name: "Unknown".to_string(), r: 0, g: 0, b: 0},
            ChocoboColor::snow_white => Color{name: "Snow White".to_string(), r: 228, g: 223, b: 208},
            ChocoboColor::ash_grey => Color{name: "Ash Grey".to_string(), r: 172, g: 168, b: 162},
            ChocoboColor::goobbue_grey => Color{name: "Goobbue Grey".to_string(), r: 137, g: 135, b: 132},
            ChocoboColor::slate_grey => Color{name: "Slate Grey".to_string(), r: 101, g: 101, b: 101},
            ChocoboColor::charcoal_grey => Color{name: "Charcoal Grey".to_string(), r: 72, g: 71, b: 66},
            ChocoboColor::soot_black => Color{name: "Soot Black".to_string(), r: 43, g: 41, b: 35},
            ChocoboColor::rose_pink => Color{name: "Rose Pink".to_string(), r: 230, g: 159, b: 150},
            ChocoboColor::lilac_purple => Color{name: "Lilac Purple".to_string(), r: 131, g: 105, b: 105},
            ChocoboColor::rolandberry_red => Color{name: "Rolandberry Red".to_string(), r: 91, g: 23, b: 41},
            ChocoboColor::dalamud_red => Color{name: "Dalamud Red".to_string(), r: 120, g: 26, b: 26},
            ChocoboColor::rust_red => Color{name: "Rust Red".to_string(), r: 98, g: 34, b: 7},
            ChocoboColor::wine_red => Color{name: "Wine Red".to_string(), r: 69, g: 21, b: 17},
            ChocoboColor::coral_pink => Color{name: "Coral Pink".to_string(), r: 204, g: 108, b: 94},
            ChocoboColor::blood_red => Color{name: "Blood Red".to_string(), r: 145, g: 59, b: 48},
            ChocoboColor::salmon_pink => Color{name: "Salmon Pink".to_string(), r: 228, g: 170, b: 138},
            ChocoboColor::sunset_orange => Color{name: "Sunset Orange".to_string(), r: 183, g: 92, b: 45},
            ChocoboColor::mesa_red => Color{name: "Mesa Red".to_string(), r: 125, g: 57, b: 6},
            ChocoboColor::bark_brown => Color{name: "Bark Brown".to_string(), r: 106, g: 75, b: 55},
            ChocoboColor::chocolat_brown => Color{name: "Chocolat Brown".to_string(), r: 110, g: 61, b: 36},
            ChocoboColor::russet_brown => Color{name: "Russet Brown".to_string(), r: 79, g: 45, b: 31},
            ChocoboColor::kobold_brown => Color{name: "Kobold Brown".to_string(), r: 48, g: 33, b: 27},
            ChocoboColor::cork_brown => Color{name: "Cork Brown".to_string(), r: 201, g: 145, b: 86},
            ChocoboColor::qiqim_brown => Color{name: "Qiqim Brown".to_string(), r: 153, g: 110, b: 63},
            ChocoboColor::opo_opo_brown => Color{name: "Opo-Opo Brown".to_string(), r: 213, g: 92, b: 45},
            ChocoboColor::aldgoat_brown => Color{name: "Aldgoat Brown".to_string(), r: 162, g: 135, b: 92},
            ChocoboColor::pumpkin_orange => Color{name: "Pumpkin Orange".to_string(), r: 197, g: 116, b: 36},
            ChocoboColor::acorn_brown => Color{name: "Acorn Brown".to_string(), r: 142, g: 88, b: 27},
            ChocoboColor::orchard_brown => Color{name: "Orchard Brown".to_string(), r: 100, g: 66, b: 22},
            ChocoboColor::chestnut_brown => Color{name: "Chestnut Brown".to_string(), r: 61, g: 41, b: 13},
            ChocoboColor::gobbiebag_brown => Color{name: "Gobbiebag Brown".to_string(), r: 185, g: 164, b: 137},
            ChocoboColor::shale_brown => Color{name: "Shale Brown".to_string(), r: 146, g: 129, b: 108},
            ChocoboColor::mole_brown => Color{name: "Mole Brown".to_string(), r:  97, g: 82, b: 69}, 
            ChocoboColor::loam_brown => Color{name: "Loam Brown".to_string(), r:  63, g: 51, b: 41}, 
            ChocoboColor::bone_white => Color{name: "Bone White".to_string(), r:  235, g: 211, b: 160}, 
            ChocoboColor::ul_brown => Color{name: "Ul Brown".to_string(), r:  183, g: 163, b: 112}, 
            ChocoboColor::desert_yellow => Color{name: "Desert Yellow".to_string(), r: 219, g: 180, b: 87}, 
            ChocoboColor::honey_yellow => Color{name: "Honey Yellow".to_string(), r:  250, g: 198, b: 43}, 
            ChocoboColor::millioncorn_yellow => Color{name: "Millioncorn Yellow".to_string(), r:  228, g: 158, b: 52}, 
            ChocoboColor::coeurl_yellow => Color{name: "Coeurl Yellow".to_string(), r:  188, g: 136, b: 4}, 
            ChocoboColor::cream_yellow => Color{name: "Cream Yellow".to_string(), r:  242, g: 215, b: 112}, 
            ChocoboColor::halatali_yellow => Color{name: "Halatali Yellow".to_string(), r:  165, g: 132, b: 48}, 
            ChocoboColor::raisin_brown => Color{name: "Raisin Brown".to_string(), r:  64, g: 51, b: 17}, 
            ChocoboColor::mud_green => Color{name: "Mud Green".to_string(), r:  88, g: 82, b: 48}, 
            ChocoboColor::sylph_green => Color{name: "Sylph Green".to_string(), r:  187, g: 187, b: 138}, 
            ChocoboColor::lime_green => Color{name: "Lime Green".to_string(), r:  171, g: 176, b: 84}, 
            ChocoboColor::moss_green => Color{name: "Moss Green".to_string(), r:  112, g: 115, b: 38}, 
            ChocoboColor::meadow_green => Color{name: "Meadow Green".to_string(), r:  139, g: 156, b: 99}, 
            ChocoboColor::olive_green => Color{name: "Olive Green".to_string(), r:  75, g: 82, b: 50}, 
            ChocoboColor::marsh_green => Color{name: "Marsh Green".to_string(), r:  50, g: 54, b: 33}, 
            ChocoboColor::apple_green => Color{name: "Apple Green".to_string(), r:  149, g: 174, b: 92}, 
            ChocoboColor::cactuar_green => Color{name: "Cactuar Green".to_string(), r:  101, g: 130, b: 65}, 
            ChocoboColor::hunter_green => Color{name: "Hunter Green".to_string(), r:  40, g: 75, b: 38}, 
            ChocoboColor::ochu_green => Color{name: "Ochu Green".to_string(), r:  64, g: 99, b: 57}, 
            ChocoboColor::adamantoise_green => Color{name: "Adamantoise Green".to_string(), r:  95, g: 117, b: 88}, 
            ChocoboColor::nophica_green => Color{name: "Nophica Green".to_string(), r:  59, g: 77, b: 60}, 
            ChocoboColor::deepwood_green => Color{name: "Deepwood Green".to_string(), r:  30, g: 42, b: 33}, 
            ChocoboColor::celeste_green => Color{name: "Celeste Green".to_string(), r:  150, g: 189, b: 185}, 
            ChocoboColor::turquoise_green => Color{name: "Turquoise Green".to_string(), r:  67, g: 114, b: 144}, 
            ChocoboColor::morbol_green => Color{name: "Morbol Green".to_string(), r:  31, g: 70, b: 70}, 
            ChocoboColor::ice_blue => Color{name: "Ice Blue".to_string(), r:  178, g: 196, b: 206}, 
            ChocoboColor::sky_blue => Color{name: "Sky Blue".to_string(), r:  131, g: 176, b: 210}, 
            ChocoboColor::seafog_blue => Color{name: "Seafog Blue".to_string(), r:  100, g: 129, b: 160}, 
            ChocoboColor::peacock_blue => Color{name: "Peacock Blue".to_string(), r:  59, g: 104, b: 134}, 
            ChocoboColor::rhotano_blue => Color{name: "Rhotano Blue".to_string(), r:  28, g: 61, b: 84}, 
            ChocoboColor::corpse_blue => Color{name: "Corpse Blue".to_string(), r:  142, g: 155, b: 172}, 
            ChocoboColor::ceruleam_blue => Color{name: "Ceruleam Blue".to_string(), r:  79, g: 87, b: 102}, 
            ChocoboColor::woad_blue => Color{name: "Woad Blue".to_string(), r:  47, g: 56, b: 81}, 
            ChocoboColor::ink_blue => Color{name: "Ink Blue".to_string(), r:  26, g: 31, b: 39}, 
            ChocoboColor::raptor_blue => Color{name: "Raptor Blue".to_string(), r:  91, g: 127, b: 192}, 
            ChocoboColor::othard_blue => Color{name: "Othard Blue".to_string(), r:  47, g: 88, b: 137}, 
            ChocoboColor::storm_blue => Color{name: "Storm Blue".to_string(), r:  35, g: 65, b: 114}, 
            ChocoboColor::void_blue => Color{name: "Void Blue".to_string(), r:  17, g: 41, b: 68}, 
            ChocoboColor::royal_blue => Color{name: "Royal Blue".to_string(), r:  39, g: 48, b: 103}, 
            ChocoboColor::midnight_blue => Color{name: "Midnight Blue".to_string(), r:  24, g: 25, b: 55}, 
            ChocoboColor::shadow_blue => Color{name: "Shadow Blue".to_string(), r:  55, g: 55, b: 71}, 
            ChocoboColor::abyssal_blue => Color{name: "Abyssal Blue".to_string(), r:  49, g: 45, b: 87}, 
            ChocoboColor::lavender_purple => Color{name: "Lavender Purple".to_string(), r:  135, g: 127, b: 174}, 
            ChocoboColor::gloom_purple => Color{name: "Gloom Purple".to_string(), r:  81, g: 69, b: 96}, 
            ChocoboColor::currant_purple => Color{name: "Currant Purple".to_string(), r:  50, g: 44, b: 59}, 
            ChocoboColor::iris_purple => Color{name: "Iris Purple".to_string(), r:  183, g: 158, b: 188}, 
            ChocoboColor::grape_purple => Color{name: "Grape Purple".to_string(), r:  59, g: 42, b: 61}, 
            ChocoboColor::lotus_pink => Color{name: "Lotus Pink".to_string(), r:  254, g: 206, b: 245}, 
            ChocoboColor::colibri_pink => Color{name: "Colibri Pink".to_string(), r:  220, g: 155, b: 202}, 
            ChocoboColor::plum_purple => Color{name: "Plum Purple".to_string(), r:  121, g: 82, b: 108}, 
            ChocoboColor::regal_purple => Color{name: "Regal Purple".to_string(), r:  102, g: 48, b: 78} 
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(serde::Deserialize, serde::Serialize, PartialEq, Copy, Clone)]
pub enum Ability {
    choco_dash_1,
    choco_dash_2,
    choco_dash_3,
    choco_cure_1,
    choco_cure_2,
    choco_cure_3,
    choco_esuna_1,
    choco_esuna_2,
    choco_esuna_3,
    choco_ease_1,
    choco_ease_2,
    choco_ease_3,
    choco_calm_1,
    choco_calm_2,
    choco_calm_3,
    choco_reflect_1,
    choco_reflect_2,
    choco_reflect_3,
    choco_steal_1,
    choco_steal_2,
    choco_steal_3,
    choco_silence_1,
    choco_silence_2,
    choco_silence_3,
    choco_shock_1,
    choco_shock_2,
    choco_shock_3,
    head_start,
    increased_stamina_1,
    increased_stamina_2,
    increased_stamina_3,
    heavy_resistance_1,
    heavy_resistance_2,
    heavy_resistance_3,
    level_head_1,
    level_head_2,
    level_head_3,
    speedy_recovery_1,
    speedy_recovery_2,
    speedy_recovery_3,
    dressage_1,
    dressage_2,
    dressage_3,
    choco_drain_1,
    choco_drain_2,
    choco_drain_3,
    paradigm_shift,
    mimic_1,
    mimic_2,
    mimic_3,
    feather_field_1,
    feather_field_2,
    feather_field_3,
    super_sprint_1,
    super_sprint_2,
    super_sprint_3,
    choco_reraise_1,
    choco_reraise_2,
    choco_reraise_3,
    enfeeblement_clause_1,
    enfeeblement_clause_2,
    enfeeblement_clause_3,
    breather_1,
    breather_2,
    breather_3
}
impl Ability {
    #[allow(dead_code)]
    fn as_str(&self) -> &'static str {
        match self {
            Ability::choco_dash_1 => "Choco Dash I",
            Ability::choco_dash_2 => "Choco Dash II",
            Ability::choco_dash_3 => "Choco Dash III",
            Ability::choco_cure_1 => "Choco Cure I",
            Ability::choco_cure_2 => "Choco Cure II",
            Ability::choco_cure_3 => "Choco Cure III",
            Ability::choco_esuna_1 => "Choco Esuna I",
            Ability::choco_esuna_2 => "Choco Esuna II",
            Ability::choco_esuna_3 => "Choco Esuna III",
            Ability::choco_ease_1 => "Choco Ease I",
            Ability::choco_ease_2 => "Choco Ease II",
            Ability::choco_ease_3 => "Choco Ease III",
            Ability::choco_calm_1 => "Choco Calm I",
            Ability::choco_calm_2 => "Choco Calm II",
            Ability::choco_calm_3 => "Choco Calm III",
            Ability::choco_reflect_1 => "Choco Reflect I",
            Ability::choco_reflect_2 => "Choco Reflect II",
            Ability::choco_reflect_3 => "Choco Reflect III",
            Ability::choco_steal_1 => "Choco Steal I",
            Ability::choco_steal_2 => "Choco Steal II",
            Ability::choco_steal_3 => "Choco Steal III",
            Ability::choco_silence_1 => "Choco Silence I",
            Ability::choco_silence_2 => "Choco Silence II",
            Ability::choco_silence_3 => "Choco Silence III",
            Ability::choco_shock_1 => "Choco Shock I",
            Ability::choco_shock_2 => "Choco Shock II",
            Ability::choco_shock_3 => "Choco Shock III",
            Ability::head_start => "Head Start",
            Ability::increased_stamina_1 => "Increased Stamina I",
            Ability::increased_stamina_2 => "Increased Stamina II",
            Ability::increased_stamina_3 => "Increased Stamina III",
            Ability::heavy_resistance_1 => "Heavy Resistance I",
            Ability::heavy_resistance_2 => "Heavy Resistance II",
            Ability::heavy_resistance_3 => "Heavy Resistance III",
            Ability::level_head_1 => "Level Head I",
            Ability::level_head_2 => "Level Head II",
            Ability::level_head_3 => "Level Head III",
            Ability::speedy_recovery_1 => "Speedy Recovery I",
            Ability::speedy_recovery_2 => "Speedy Recovery II",
            Ability::speedy_recovery_3 => "Speedy Recovery III",
            Ability::dressage_1 => "Dressage I",
            Ability::dressage_2 => "Dressage II",
            Ability::dressage_3 => "Dressage III",
            Ability::choco_drain_1 => "Choco Drain I",
            Ability::choco_drain_2 => "Choco Drain II",
            Ability::choco_drain_3 => "Choco Drain III",
            Ability::paradigm_shift => "Paradigm Shift",
            Ability::mimic_1 => "Mimic I",
            Ability::mimic_2 => "Mimic II",
            Ability::mimic_3 => "Mimic III",
            Ability::feather_field_1 => "Feather Field I",
            Ability::feather_field_2 => "Feather Field II",
            Ability::feather_field_3 => "Feather Field III",
            Ability::super_sprint_1 => "Super Sprint I",
            Ability::super_sprint_2 => "Super Sprint II",
            Ability::super_sprint_3 => "Super Sprint III",
            Ability::choco_reraise_1 => "Choco Reraise I",
            Ability::choco_reraise_2 => "Choco Reraise II",
            Ability::choco_reraise_3 => "Choco Reraise III",
            Ability::enfeeblement_clause_1 => "Enfeeblement Clause I",
            Ability::enfeeblement_clause_2 => "Enfeeblement Clause II",
            Ability::enfeeblement_clause_3 => "Enfeeblement Clause III",
            Ability::breather_1 => "Breather I",
            Ability::breather_2 => "Breather II",
            Ability::breather_3 => "Breather III"
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize, PartialEq, Copy, Clone)]
pub struct Pedigree {
    pub speed: i8,
    pub acceleration: i8,
    pub endurance: i8,
    pub stamina: i8,
    pub cunning: i8
}
impl Pedigree {
    pub fn new() -> Self {
        Pedigree {
            speed: 1,
            acceleration: 1,
            endurance: 1,
            stamina: 1,
            cunning: 1
        }
    }
    
    #[allow(dead_code)]
    pub fn as_array(&self) -> [i8; 5] {
        [self.speed, self.acceleration, self.endurance, self.stamina, self.cunning]
    }
}

#[allow(non_camel_case_types)]
#[derive(serde::Deserialize, serde::Serialize, PartialEq, Copy, Clone)]
pub enum Gender {
    female,
    male,
    unknown
}
impl Gender {
    pub fn as_str(&self) -> &'static str {
        match self {
            Gender::female => "F",
            Gender::male => "M",
            Gender::unknown => "X"
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize, Copy, Clone, PartialEq)]
pub struct Chocobo {
    pub grade: i8,
    pub color: ChocoboColor,
    pub gender: Gender,
    pub breeding_left: i8,

    pub parents: (Pedigree, Pedigree),

    pub ability: Ability,
    pub is_covering: bool
    //prefered_weather: String = "TODO"
}

impl Chocobo {
    pub fn new(
        grade: i8,
        gender: Gender,
        parent1: Pedigree,
        parent2: Pedigree,
        ability: Ability,
        breeding_left: i8,
        color: ChocoboColor,
        is_covering: bool
    ) -> Self {
        Chocobo {
            grade,
            color, 
            gender,
            parents: (parent1, parent2),
            ability,
            breeding_left,
            is_covering
        }
    }
    
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
                    ui.set_width(125.0);
                    ui.set_height(200.0);
                    ui.horizontal(|ui| {
                        ui.vertical(|ui| {
                            if self.is_covering {
                                ui.label(egui::RichText::new(format!("G{} {} <permit>", self.grade, self.gender.as_str()))
                                    .color(egui::Color32::DARK_RED)
                                );
                            } else {
                                ui.label(format!("G{} {}", self.grade, self.gender.as_str()));
                            }
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
                                            row.col(|ui| {ui.label(self.parents.0.speed.to_string());});
                                            row.col(|ui| {ui.label(self.parents.1.speed.to_string());});
                                        }
                                        1 => {
                                            row.col(|ui| {ui.label("Acceleration:");});
                                            row.col(|ui| {ui.label(self.parents.0.acceleration.to_string());});
                                            row.col(|ui| {ui.label(self.parents.1.acceleration.to_string());});
                                        }
                                        2 => {
                                            row.col(|ui| {ui.label("Endurance:");});
                                            row.col(|ui| {ui.label(self.parents.0.endurance.to_string());});
                                            row.col(|ui| {ui.label(self.parents.1.endurance.to_string());});
                                        }
                                        3 => {
                                            row.col(|ui| {ui.label("Stamina:");});
                                            row.col(|ui| {ui.label(self.parents.0.stamina.to_string());});
                                            row.col(|ui| {ui.label(self.parents.1.stamina.to_string());});
                                        }
                                        4 => {
                                            row.col(|ui| {ui.label("Cunning:");});
                                            row.col(|ui| {ui.label(self.parents.0.cunning.to_string());});
                                            row.col(|ui| {ui.label(self.parents.1.cunning.to_string());});
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