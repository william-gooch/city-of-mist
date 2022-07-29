table! {
    campaigns (id) {
        id -> Unsigned<Bigint>,
        name -> Varchar,
    }
}

table! {
    use common::models::campaign::MemberTypeMapping;
    use diesel::sql_types::*;
    campaign_members (campaign_id, user_id) {
        campaign_id -> Unsigned<Bigint>,
        user_id -> Unsigned<Bigint>,
        character_id -> Nullable<Unsigned<Bigint>>,
        member_type -> MemberTypeMapping,
    }
}

table! {
    characters (id) {
        id -> Unsigned<Bigint>,
        name -> Varchar,
        mythos -> Varchar,
        logos -> Varchar,
    }
}

table! {
    character_themes (character_id, theme_id) {
        character_id -> Unsigned<Bigint>,
        theme_id -> Unsigned<Bigint>,
    }
}

table! {
    use common::models::theme::ThemeTypeMapping;
    use diesel::sql_types::*;
    themes (id) {
        id -> Unsigned<Bigint>,
        theme_descriptor -> Varchar,
        title -> Varchar,
        key_phrase -> Varchar,
        theme_type -> ThemeTypeMapping,
        attention -> Unsigned<Tinyint>,
        degrade -> Unsigned<Tinyint>,
        tags -> Json,
    }
}

table! {
    users (id) {
        id -> Unsigned<Bigint>,
        email -> Varchar,
        username -> Varchar,
        password_hash -> Varchar,
        password_salt -> Varchar,
    }
}

joinable!(campaign_members -> campaigns (campaign_id));
joinable!(campaign_members -> characters (character_id));
joinable!(campaign_members -> users (user_id));
joinable!(character_themes -> characters (character_id));
joinable!(character_themes -> themes (theme_id));

allow_tables_to_appear_in_same_query!(
    campaigns,
    campaign_members,
    characters,
    character_themes,
    themes,
    users,
);
