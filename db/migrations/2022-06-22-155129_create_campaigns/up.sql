CREATE TABLE campaigns (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL
);

CREATE TABLE campaign_members (
    campaign_id BIGINT UNSIGNED NOT NULL,
    user_id BIGINT UNSIGNED NOT NULL,
    character_id BIGINT UNSIGNED NULL,
    member_type ENUM ("gm", "player") NOT NULL,
    PRIMARY KEY (campaign_id, user_id),
    FOREIGN KEY (user_id)
        REFERENCES users(id)
        ON DELETE CASCADE
        ON UPDATE CASCADE,
    FOREIGN KEY (campaign_id)
        REFERENCES campaigns(id)
        ON DELETE CASCADE
        ON UPDATE CASCADE,
    FOREIGN KEY (character_id)
        REFERENCES characters(id)
        ON DELETE CASCADE
        ON UPDATE CASCADE
);
