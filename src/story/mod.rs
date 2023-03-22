use crate::ModelService;
use sqlx::Row;
use tonic::async_trait;

#[async_trait]
pub trait StoryTrait {
    /// make a story
    async fn add_story(&self, mut s: abi::Story) -> Result<abi::Story, abi::Error>;
}

#[async_trait]
impl StoryTrait for ModelService {
    /// make a story
    async fn add_story(&self, mut s: abi::Story) -> Result<abi::Story, abi::Error> {
        let id: i64 =
            sqlx::query(r#"INSERT INTO story (words, content) VALUES ($1, $2) RETURNING id"#)
                .bind(s.words.clone())
                .bind(s.content.clone())
                .fetch_one(&self.pool)
                .await?
                .get(0);

        s.id = id;
        Ok(s)
    }
}

#[cfg(test)]
mod test {

    use super::*;

    async fn get_db(
        pool: &sqlx::PgPool,
        words: Vec<String>,
        content: String,
    ) -> Result<(ModelService, abi::Story), abi::Error> {
        let db = ModelService::new(pool.clone());
        let v = abi::Story {
            id: 0,
            words,
            content,
            read_count: 0,
            created_at: None,
            updated_at: None,
        };
        let v = db.add_story(v).await?;
        Ok((db, v))
    }

    async fn get_a_small_story_db(
        pool: &sqlx::PgPool,
    ) -> Result<(ModelService, abi::Story), abi::Error> {
        get_db(
            pool,
            vec!["apple".to_string(), "fun".to_string()],
            "a apple fun story".to_string(),
        )
        .await
    }

    #[sqlx_database_tester::test(pool(variable = "migrated_pool", migrations = "../migrations"))]
    async fn add_story_should_work() {
        let (_db, s) = get_a_small_story_db(&migrated_pool).await.unwrap();
        assert_eq!(s.read_count, 0);
        assert_eq!(s.words, vec!["apple".to_string(), "fun".to_string()]);
    }
}
