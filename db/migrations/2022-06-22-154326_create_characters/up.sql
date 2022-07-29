CREATE TABLE characters (
    id SERIAL PRIMARY KEY,

    name VARCHAR(255) NOT NULL,
    mythos VARCHAR(255) NOT NULL,
    logos VARCHAR(255) NOT NULL
);

CREATE TABLE character_themes (
    character_id BIGINT UNSIGNED NOT NULL,
    theme_id BIGINT UNSIGNED NOT NULL,
    PRIMARY KEY (character_id, theme_id),
    FOREIGN KEY (character_id)
        REFERENCES characters(id)
        ON DELETE CASCADE
        ON UPDATE CASCADE,
    FOREIGN KEY (theme_id)
        REFERENCES themes(id)
        ON DELETE CASCADE
        ON UPDATE CASCADE
);
