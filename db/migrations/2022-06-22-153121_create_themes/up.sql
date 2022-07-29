CREATE TABLE themes (
    id SERIAL PRIMARY KEY,
    theme_descriptor VARCHAR(50) NOT NULL,
    title VARCHAR(100) NOT NULL,
    key_phrase VARCHAR(100) NOT NULL,
    theme_type ENUM ("mythos", "logos", "crew", "extra") NOT NULL,
    attention TINYINT UNSIGNED NOT NULL,
    degrade TINYINT UNSIGNED NOT NULL,
    tags JSON NOT NULL
)
