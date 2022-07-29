use shaku::Interface;

use crate::{models::{character::*, theme::*}, Id};

pub trait ThemeManager: Interface {
    fn create_theme(&self, new_theme: NewTheme) -> Theme;
    fn add_theme_to_character(&self, character: &Character, theme: &Theme) -> ();
}
