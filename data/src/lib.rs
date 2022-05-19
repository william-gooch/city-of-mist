pub mod theme_descriptors;

use crate::theme_descriptors::*;
use common::character::*;
use common::theme::*;
use common::theme_descriptor::*;

pub fn example_character() -> Character {
    let expression = ThemeBuilder::default()
        .theme_descriptor("expression")
        .theme_type(ThemeType::Mythos)
        .title("Tooth and Nail")
        .mystery_or_identity("blah")
        .build()
        .unwrap();

    let bastion = ThemeBuilder::default()
        .theme_descriptor("bastion")
        .theme_type(ThemeType::Mythos)
        .title("Demigod")
        .mystery_or_identity("blah")
        .build()
        .unwrap();

    let divination = ThemeBuilder::default()
        .theme_descriptor("divination")
        .theme_type(ThemeType::Mythos)
        .title("Animal Senses")
        .mystery_or_identity("blah")
        .build()
        .unwrap();

    let defining_event = ThemeBuilder::default()
        .theme_descriptor("defining_event")
        .theme_type(ThemeType::Logos)
        .title("Trust Is A Dagger")
        .mystery_or_identity("blah")
        .build()
        .unwrap();

    let crew = ThemeBuilder::default()
        .theme_descriptor("crew")
        .theme_type(ThemeType::Crew)
        .title("Crew")
        .mystery_or_identity("blah")
        .build()
        .unwrap();

    CharacterBuilder::default()
        .name("Fenrir")
        .mythos("Fenrir")
        .logos("Vagrant")
        .core_themes(vec![expression, bastion, divination, defining_event])
        .crew_theme(Some(crew))
        .build()
        .unwrap()
}

pub trait HasThemeDescriptor {
    fn descriptor(&self) -> &ThemeDescriptor;
}

impl HasThemeDescriptor for Theme {
    fn descriptor(&self) -> &ThemeDescriptor {
        &THEME_DESCRIPTORS.get(&self.theme_descriptor).unwrap()
    }
}
