-- Add up migration script here

CREATE SCHEMA IF NOT EXISTS orion;

-- create vocabulary table
CREATE TABLE IF NOT EXISTS orion.vocabulary (
    id BIGSERIAL PRIMARY KEY,
    word VARCHAR(64) NOT NULL, -- word
    soundmark VARCHAR(128) NOT NULL, -- 音标
    roots VARCHAR(256), -- 词根词缀
    paraphrase VARCHAR(256) NOT NULL, -- 中文释义
    collocations TEXT, -- 常用搭配
    synonyms TEXT, -- 同义词
    examples TEXT, -- 例句
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

CREATE UNIQUE INDEX IF NOT EXISTS orion_word_idx ON orion.vocabulary (word);
