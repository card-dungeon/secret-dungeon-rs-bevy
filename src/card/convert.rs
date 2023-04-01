use crate::card::*;

pub fn string_to_class(class: &str) -> Class {
    match class {
        "WARRIOR" => Class::Warrior,
        "MAGE" => Class::Mage,
        "PRIEST" => Class::Priest,
        "ROGUE" => Class::Rogue,
        "HUNTER" => Class::Hunter,
        "DRUID" => Class::Druid,
        "SHAMAN" => Class::Shaman,
        "PALADIN" => Class::Paladin,
        "WARLOCK" => Class::Warlock,
        _ => Class::NONE,
    }
}
