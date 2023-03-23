-- Add up migration script here

-- create a new word table
CREATE TYPE orion.word_status AS ENUM ('new', 'easy', 'difficult', 'learned');

CREATE TABLE IF NOT EXISTS orion.learn_word (
    id BIGSERIAL PRIMARY KEY,
    word VARCHAR(64) NOT NULL,
    vocabulary_id BIGINT,
    word_list_id BIGINT,
    learn_count BIGINT NOT NULL DEFAULT 0,
    learn_status orion.word_status NOT NULL DEFAULT 'new',
    last_learned_at VARCHAR(64) NOT NULL DEFAULT '0', -- last learned time
    next_learn_at VARCHAR(64) NOT NULL DEFAULT '0', -- next learn time
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

CREATE UNIQUE INDEX IF NOT EXISTS orion_learn_word_idx ON orion.learn_word (word);
CREATE INDEX IF NOT EXISTS orion_learn_word_learn_status_idx ON orion.learn_word (learn_status);
CREATE INDEX IF NOT EXISTS orion_learn_word_next_learn_at_idx ON orion.learn_word (next_learn_at);
