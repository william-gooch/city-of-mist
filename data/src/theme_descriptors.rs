use common::models::theme_descriptor::ThemeDescriptor;
use include_dir::{include_dir, Dir};
use lazy_static::lazy_static;
use std::collections::HashMap;

static DATA_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/data");

lazy_static! {

    // MYTHOS THEMES

    pub static ref ADAPTATION: ThemeDescriptor = serde_json::from_str(
        DATA_DIR.get_file("adaptation.json").unwrap()
                .contents_utf8().unwrap()
    ).unwrap();

    pub static ref BASTION: ThemeDescriptor = serde_json::from_str(
        DATA_DIR.get_file("bastion.json").unwrap()
                .contents_utf8().unwrap()
    ).unwrap();

    pub static ref DIVINATION: ThemeDescriptor = serde_json::from_str(
        DATA_DIR.get_file("divination.json").unwrap()
                .contents_utf8().unwrap()
    ).unwrap();

    pub static ref EXPRESSION: ThemeDescriptor = serde_json::from_str(
        DATA_DIR.get_file("expression.json").unwrap()
                .contents_utf8().unwrap()
    ).unwrap();

    pub static ref MOBILITY: ThemeDescriptor = serde_json::from_str(
        DATA_DIR.get_file("mobility.json").unwrap()
                .contents_utf8().unwrap()
    ).unwrap();

    pub static ref RELIC: ThemeDescriptor = serde_json::from_str(
        DATA_DIR.get_file("relic.json").unwrap()
                .contents_utf8().unwrap()
    ).unwrap();

    pub static ref SUBVERSION: ThemeDescriptor = serde_json::from_str(
        DATA_DIR.get_file("subversion.json").unwrap()
                .contents_utf8().unwrap()
    ).unwrap();

    // LOGOS THEMES

    pub static ref DEFINING_EVENT: ThemeDescriptor = serde_json::from_str(
        DATA_DIR.get_file("defining_event.json").unwrap()
                .contents_utf8().unwrap()
    ).unwrap();

    pub static ref DEFINING_RELATIONSHIP: ThemeDescriptor = serde_json::from_str(
        DATA_DIR.get_file("defining_relationship.json").unwrap()
                .contents_utf8().unwrap()
    ).unwrap();

    pub static ref MISSION: ThemeDescriptor = serde_json::from_str(
        DATA_DIR.get_file("mission.json").unwrap()
                .contents_utf8().unwrap()
    ).unwrap();

    pub static ref PERSONALITY: ThemeDescriptor = serde_json::from_str(
        DATA_DIR.get_file("personality.json").unwrap()
                .contents_utf8().unwrap()
    ).unwrap();

    pub static ref POSSESSIONS: ThemeDescriptor = serde_json::from_str(
        DATA_DIR.get_file("possessions.json").unwrap()
                .contents_utf8().unwrap()
    ).unwrap();

    pub static ref ROUTINE: ThemeDescriptor = serde_json::from_str(
        DATA_DIR.get_file("routine.json").unwrap()
                .contents_utf8().unwrap()
    ).unwrap();

    pub static ref TRAINING: ThemeDescriptor = serde_json::from_str(
        DATA_DIR.get_file("training.json").unwrap()
                .contents_utf8().unwrap()
    ).unwrap();

    // CREW THEME

    pub static ref CREW: ThemeDescriptor = serde_json::from_str(
        DATA_DIR.get_file("crew.json").unwrap()
                .contents_utf8().unwrap()
    ).unwrap();

    // EXTRA THEMES

    pub static ref ALLY: ThemeDescriptor = serde_json::from_str(
        DATA_DIR.get_file("ally.json").unwrap()
                .contents_utf8().unwrap()
    ).unwrap();

    pub static ref BASE_OF_OPERATIONS: ThemeDescriptor = serde_json::from_str(
        DATA_DIR.get_file("base_of_operations.json").unwrap()
                .contents_utf8().unwrap()
    ).unwrap();

    pub static ref RIDE: ThemeDescriptor = serde_json::from_str(
        DATA_DIR.get_file("ride.json").unwrap()
                .contents_utf8().unwrap()
    ).unwrap();

    pub static ref THEME_DESCRIPTORS: HashMap<String, &'static ThemeDescriptor> = HashMap::from([
        ("adaptation".to_owned(), &*ADAPTATION),
        ("bastion".to_owned(), &*BASTION),
        ("divination".to_owned(), &*DIVINATION),
        ("expression".to_owned(), &*EXPRESSION),
        ("mobility".to_owned(), &*MOBILITY),
        ("relic".to_owned(), &*RELIC),
        ("subversion".to_owned(), &*SUBVERSION),

        ("defining_event".to_owned(), &*DEFINING_EVENT),
        ("defining_relationship".to_owned(), &*DEFINING_RELATIONSHIP),
        ("mission".to_owned(), &*MISSION),
        ("personality".to_owned(), &*PERSONALITY),
        ("possessions".to_owned(), &*POSSESSIONS),
        ("routine".to_owned(), &*ROUTINE),
        ("training".to_owned(), &*TRAINING),

        ("crew".to_owned(), &*CREW),
        ("ally".to_owned(), &*ALLY),
        ("base_of_operations".to_owned(), &*BASE_OF_OPERATIONS),
        ("ride".to_owned(), &*RIDE),
    ]);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
