use crate::service::database::Db;
use async_trait::async_trait;
use common::character::Character;
use common::entity::character;
use common::entity::theme;
use common::sea_orm::prelude::*;
use common::sea_orm::ActiveModelTrait;
use shaku::{Component, Interface};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::*;

#[async_trait]
pub trait CharacterManager: Interface {
    async fn load(&self, id: i32) -> Option<Character>;
    async fn mutate(
        &self,
        id: i32,
        mut callback: Box<dyn FnMut(Character) -> Character + Send + Sync>,
    ) -> Option<()>;
}

#[derive(Component)]
#[shaku(interface = CharacterManager)]
pub struct CharacterManagerImpl {
    #[shaku(inject)]
    db: Arc<dyn Db>,
    #[shaku(default)]
    loaded_characters: Mutex<HashMap<i32, Character>>,
}

impl CharacterManagerImpl {
    async fn load_mut<'a>(
        &'a self,
        id: i32,
    ) -> Option<impl 'a + std::ops::DerefMut<Target = Character>> {
        MutexGuard::try_map(self.loaded_characters.lock().await, |loaded_characters| {
            Some(loaded_characters.get_mut(&id).unwrap())
        })
        .ok()
    }
}

#[async_trait]
impl CharacterManager for CharacterManagerImpl {
    async fn load(&self, id: i32) -> Option<Character> {
        let mut loaded_characters = self.loaded_characters.lock().await;
        if loaded_characters.contains_key(&id) {
            Some(loaded_characters.get(&id).unwrap().clone())
        } else {
            if let Some(character) = {
                character::Entity::find_by_id(id)
                    .find_with_related(theme::Entity)
                    .all(self.db.get())
                    .await
                    .ok()
                    .map(|v| v.into_iter().next())
                    .flatten()
            } {
                Some(
                    loaded_characters
                        .entry(id)
                        .or_insert(character.clone().into())
                        .clone(),
                )
            } else {
                None
            }
        }
    }

    async fn mutate(
        &self,
        id: i32,
        mut callback: Box<dyn FnMut(Character) -> Character + Send + Sync>,
    ) -> Option<()> {
        if let Some(mut character) = self.load_mut(id).await {
            *character = callback(character.clone());
            let (m_character, m_themes): (character::ActiveModel, Vec<theme::ActiveModel>) =
                character.clone().into();
            m_character.save(self.db.get()).await.ok()?;
            for m_theme in m_themes.into_iter() {
                m_theme.save(self.db.get()).await.ok()?;
            }
            Some(())
        } else {
            None
        }
    }
}
