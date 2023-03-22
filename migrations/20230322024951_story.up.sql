-- Add up migration script here

-- create a story table
CREATE TABLE IF NOT EXISTS orion.story (
    id BIGSERIAL PRIMARY KEY,
    words VARCHAR(64)[] NOT NULL,
    content TEXT NOT NULL,
    read_count BIGINT NOT NULL DEFAULT 0,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS orion_story_words_idx ON orion.story USING GIN (words);
CREATE INDEX IF NOT EXISTS orion_story_read_count_idx ON orion.story (read_count);

COMMENT ON COLUMN orion.story.words IS 'story key words';
COMMENT ON COLUMN orion.story.content IS 'story content';
COMMENT ON COLUMN orion.story.read_count IS 'read count';
