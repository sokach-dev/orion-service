-- Add up migration script here

CREATE TABLE IF NOT EXISTS orion.word_list (
    id BIGSERIAL PRIMARY KEY,
    word VARCHAR(64) NOT NULL, -- word
    paraphrase VARCHAR(256) NOT NULL, -- 中文释义
    classification VARCHAR(125) NOT NULL DEFAULT 'unknown',
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS orion_word_list_idx ON orion.word_list (classification);
