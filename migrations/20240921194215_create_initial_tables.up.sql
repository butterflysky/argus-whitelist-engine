-- Create Players table
CREATE TABLE Players
(
    id                      INT AUTO_INCREMENT PRIMARY KEY,
    discord_user_id         BIGINT                                                                  NOT NULL UNIQUE,
    discord_username        VARCHAR(255)                                                            NOT NULL,
    discord_nickname        VARCHAR(255)                                                            NULL,
    minecraft_uuid          CHAR(36)                                                                NOT NULL UNIQUE,
    minecraft_username      VARCHAR(255)                                                            NOT NULL,
    is_patron               BOOLEAN                                                                 NOT NULL DEFAULT false,
    status                  ENUM ('applied', 'whitelisted', 'tempbanned', 'permabanned', 'removed') NOT NULL DEFAULT 'applied',
    application_timestamp   DATETIME                                                                NULL,
    status_changed_at       DATETIME                                                                NULL,
    last_modified_timestamp DATETIME                                                                NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
);

-- Create DiscordUsernameHistory table
CREATE TABLE DiscordUsernameHistory
(
    id               INT AUTO_INCREMENT PRIMARY KEY,
    player_id        INT          NOT NULL,
    discord_username VARCHAR(255) NOT NULL,
    discord_nickname VARCHAR(255) NULL,
    changed_at       DATETIME     NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (player_id) REFERENCES Players (id) ON DELETE CASCADE
);

-- Create MinecraftUsernameHistory table
CREATE TABLE MinecraftUsernameHistory
(
    id                 INT AUTO_INCREMENT PRIMARY KEY,
    player_id          INT          NOT NULL,
    minecraft_username VARCHAR(255) NOT NULL,
    changed_at         DATETIME     NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (player_id) REFERENCES Players (id) ON DELETE CASCADE
);

-- Create WhitelistChanges table
CREATE TABLE WhitelistChanges
(
    id         INT AUTO_INCREMENT PRIMARY KEY,
    player_id  INT                                                                     NOT NULL,
    changed_by BIGINT                                                                  NOT NULL,
    old_status ENUM ('applied', 'whitelisted', 'tempbanned', 'permabanned', 'removed') NOT NULL,
    new_status ENUM ('applied', 'whitelisted', 'tempbanned', 'permabanned', 'removed') NOT NULL,
    changed_at DATETIME                                                                NOT NULL DEFAULT CURRENT_TIMESTAMP,
    reason     TEXT                                                                    NULL,
    FOREIGN KEY (player_id) REFERENCES Players (id) ON DELETE CASCADE
);

-- Create Warnings table
CREATE TABLE Warnings
(
    id           INT AUTO_INCREMENT PRIMARY KEY,
    player_id    INT          NOT NULL,
    warned_by    BIGINT       NOT NULL,
    warning_text TEXT         NOT NULL,
    ticket_id    VARCHAR(255) NULL, -- reference to support ticket in discord
    created_at   DATETIME     NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (player_id) REFERENCES Players (id) ON DELETE CASCADE
);