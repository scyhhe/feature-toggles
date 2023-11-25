-- Your SQL goes here
CREATE TABLE features
(
    -- TODO change to UUID
    id         VARCHAR(36) PRIMARY KEY,  
    name       VARCHAR(255) NOT NULL,
    state      BOOLEAN NOT NULL DEFAULT false
);

CREATE INDEX features_name_idx ON features(name);