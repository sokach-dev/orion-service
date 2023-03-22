-- Add up migration script here

-- 创建单词分类的枚举，有CET-4, CET-6, 初中，高中，考研，托福，SAT
CREATE TYPE orion.word_classification AS ENUM ('CET-4', 'CET-6', 'junior', 'senior', 'graduate', 'TOEFL', 'SAT', 'unknown');

CREATE TABLE IF NOT EXISTS orion.word_list (
    id BIGSERIAL PRIMARY KEY,
    word VARCHAR(64) NOT NULL, -- word
    paraphrase VARCHAR(256) NOT NULL, -- 中文释义
    classification orion.word_classification NOT NULL DEFAULT 'unknown',
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS orion_word_list_idx ON orion.word_list (classification);
