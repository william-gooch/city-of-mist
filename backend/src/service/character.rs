use crate::service::database::Db;
use async_trait::async_trait;
use common::models::character::*;
use shaku::{Component, Interface};
use std::collections::HashMap;
use std::ops::{Deref, DerefMut};
use std::sync::Arc;
use tokio::runtime::Handle;
use tokio::sync::{MappedMutexGuard, Mutex, MutexGuard};
use db::{
    diesel,
    diesel::prelude::*
};

#[async_trait]
pub trait CharacterManager: Interface {
    async fn load<'a>(&'a self, id: i32) -> Option<CharacterHandle<'a>>;
    async fn mutate<'a>(&'a self, id: i32) -> Option<CharacterHandleMut<'a>>;
    async fn mutate_from_json<'a>(
        &'a self,
        id: i32,
        json: serde_json::Value,
    ) -> Result<CharacterHandle<'a>, String>;
    async fn create<'a>(&'a self, character: Character) -> Result<CharacterHandle<'a>, DbErr>;
}

#[derive(Component)]
#[shaku(interface = CharacterManager)]
pub struct CharacterManagerImpl {
    #[shaku(inject)]
    db: Arc<dyn Db>,
    #[shaku(default = Mutex::new(HashMap::new()))]
    loaded_characters: Mutex<HashMap<i32, Character>>,
}

pub struct CharacterHandle<'a> {
    guard: MappedMutexGuard<'a, CharacterWithThemes>,
}

impl<'a> Deref for CharacterHandle<'a> {
    type Target = CharacterWithThemes;

    fn deref(&self) -> &Self::Target {
        self.guard.deref()
    }
}

pub struct CharacterHandleMut<'a> {
    guard: MappedMutexGuard<'a, CharacterWithThemes>,
    db: Arc<dyn Db>,
}

impl<'a> Deref for CharacterHandleMut<'a> {
    type Target = CharacterWithThemes;

    fn deref(&self) -> &Self::Target {
        self.guard.deref()
    }
}

impl<'a> DerefMut for CharacterHandleMut<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.guard.deref_mut()
    }
}

impl<'a> Drop for CharacterHandleMut<'a> {
    fn drop(&mut self) {
        let character = self.guard.clone();
        diesel::update(character::table).set(&character);
        for theme in character.themes.into_iter() {
            diesel::update(theme::table).set(&theme);
        }
    }
}

impl CharacterManagerImpl {
    async fn get_refresh<'a>(&'a self, id: i32) -> Option<MappedMutexGuard<'a, Character>> {
        MutexGuard::try_map(self.loaded_characters.lock().await, |loaded_characters| {
            if let Some(character) = {
                println!("Loading {} from database...", id);
                tokio::task::block_in_place(move || {
                    Handle::current().block_on(async move {
                        character::Entity::find_by_id(id)
                            .find_with_related(theme::Entity)
                            .all(self.db.get())
                            .await
                            .ok()
                            .map(|v| v.into_iter().next())
                            .flatten()
                    })
                })
            } {
                println!("Loaded!");
                loaded_characters.insert(id, character.clone().into());
                loaded_characters.get_mut(&id)
            } else {
                println!("Couldn't load!");
                None
            }
        })
        .ok()
    }

    async fn get_mut<'a>(&'a self, id: i32) -> Option<MappedMutexGuard<'a, Character>> {
        MutexGuard::try_map(self.loaded_characters.lock().await, |loaded_characters| {
            if loaded_characters.contains_key(&id) {
                Some(loaded_characters.get_mut(&id).unwrap())
            } else {
                if let Some(character) = {
                    println!("Loading {} from database...", id);
                    tokio::task::block_in_place(move || {
                        Handle::current().block_on(async move {
                            character::Entity::find_by_id(id)
                                .find_with_related(theme::Entity)
                                .all(self.db.get())
                                .await
                                .ok()
                                .map(|v| v.into_iter().next())
                                .flatten()
                        })
                    })
                } {
                    println!("Loaded!");
                    Some(
                        loaded_characters
                            .entry(id)
                            .or_insert(character.clone().into()),
                    )
                } else {
                    println!("Couldn't load!");
                    None
                }
            }
        })
        .ok()
    }

    async fn new<'a>(
        &'a self,
        character: Character,
    ) -> Result<MappedMutexGuard<'a, Character>, DbErr> {
        let (m_character, m_themes): (character::ActiveModel, Vec<theme::ActiveModel>) =
            character.into();

        // let character_insert = character::Entity::insert(m_character)
        //     .exec(self.db.get())
        //     .await?;
        // let theme_insert = theme::Entity::insert_many(m_themes)
        //     .exec(self.db.get())
        //     .await?;

        let new_character = m_character.insert(self.db.get()).await.unwrap();
        let new_themes = m_themes.into_iter().map(|m_theme| {
            let db = self.db.clone();
            async move { m_theme.insert(db.get()).await.unwrap() }
        });
        let new_themes = futures::future::join_all(new_themes).await;

        let new_character_themes = new_themes.iter().map(|new_theme| {
            let m_character_theme = character_theme::ActiveModel {
                character_id: Set(new_character.id),
                theme_id: Set(new_theme.id),
            };

            let db = self.db.clone();
            async move { m_character_theme.insert(db.get()).await.unwrap() }
        });
        futures::future::join_all(new_character_themes).await;

        Ok(self.get_mut(new_character.id).await.unwrap())
    }
}

#[async_trait]
impl CharacterManager for CharacterManagerImpl {
    async fn load<'a>(&'a self, id: i32) -> Option<CharacterHandle<'a>> {
        self.get_mut(id)
            .await
            .map(|guard| CharacterHandle { guard })
    }

    async fn mutate<'a>(&'a self, id: i32) -> Option<CharacterHandleMut<'a>> {
        self.get_mut(id).await.map(|guard| CharacterHandleMut {
            guard,
            db: self.db.clone(),
        })
    }

    async fn mutate_from_json<'a>(
        &'a self,
        id: i32,
        json: serde_json::Value,
    ) -> Result<CharacterHandle<'a>, String> {
        let character_builder = serde_json::from_value::<CharacterBuilder>(json)
            .map_err(|err| format!("Couldn't create active model: {}", err))?;
        let (mut m_character, m_themes): (character::ActiveModel, Vec<theme::ActiveModel>) =
            character_builder.into();

        let new_character = if m_character.is_changed() {
            m_character.id = Set(id);
            m_character
                .update(self.db.get())
                .await
                .map_err(|err| format!("Couldn't save character: {}", err))?
        } else {
            character::Entity::find_by_id(id)
                .one(self.db.get())
                .await
                .map_err(|err| format!("Couldn't get character: {}", err))?
                .ok_or("No such character.")?
        };

        let new_themes = m_themes.into_iter().map(|m_theme| async move {
            m_theme
                .update(self.db.get())
                .await
                .map_err(|err| format!("Couldn't save character: {}", err))
        });
        let _new_themes = futures::future::join_all(new_themes)
            .await
            .into_iter()
            .collect::<Result<Vec<theme::Model>, String>>()?;

        self.get_refresh(new_character.id)
            .await
            .map(|guard| CharacterHandle { guard })
            .ok_or("Couldn't get character after save.".into())
    }

    async fn create<'a>(&'a self, character: Character) -> Result<CharacterHandle<'a>, DbErr> {
        self.new(character)
            .await
            .map(|guard| CharacterHandle { guard })
    }
}
