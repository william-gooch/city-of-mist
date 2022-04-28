use crate::theme::ThemeType;
use derive_builder::Builder;
use derive_getters::Getters;
use derive_new::new;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, PartialEq, Clone, Default, Debug, new, Getters)]
pub struct ThemeConcept {
    initial_blurb: String,
    question: String,
    example_answers: Vec<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Default, Debug, new, Getters)]
pub struct ThemeImprovement {
    name: String,
    description: String,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Default, Debug, new, Getters)]
pub struct ThemePhrase {
    prompts: Vec<String>,
    example_answers: Vec<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Default, Debug, new, Getters)]
pub struct TagQuestion {
    question: String,
    example_answers: Vec<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Default, Debug, new, Getters)]
pub struct ThemeRelationships {
    example_answers: Vec<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Default, Debug, Builder, Getters)]
pub struct ThemeDescriptor {
    #[builder(setter(into))]
    name: String,
    theme_type: ThemeType,

    concept: ThemeConcept,
    phrase: ThemePhrase,
    relationships: ThemeRelationships,
    improvements: Vec<ThemeImprovement>,

    power_tag_questions: HashMap<char, TagQuestion>,
    weakness_tag_questions: HashMap<char, TagQuestion>,
}
