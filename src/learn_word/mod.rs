use crate::{vocabulary::VocabularyTrait, word_list::WordListTrait, ModelService};
use sqlx::Row;
use tonic::async_trait;

#[async_trait]
pub trait LearnWordTrait {
    /// make a learn word
    async fn add_learn_word(&self, mut s: abi::LearnWord) -> Result<abi::LearnWord, abi::Error>;
    /// query learn word
    async fn query_learn_word(
        &self,
        q: abi::LearnWordQuery,
    ) -> Result<Vec<abi::LearnWord>, abi::Error>;
}

#[async_trait]
impl LearnWordTrait for ModelService {
    /// make a learn word
    async fn add_learn_word(&self, mut s: abi::LearnWord) -> Result<abi::LearnWord, abi::Error> {
        let word_list_query = abi::WordListQueryBuilder::default()
            .word(Some(s.word.clone()))
            .build()
            .unwrap();

        let words = self.query_word_list(word_list_query).await?;
        let mut word_list_id = 0;
        if words.len() > 1 {
            word_list_id = words[0].id;
        }

        let vocabulary_query = abi::VocabularyQueryBuilder::default()
            .word(Some(s.word.clone()))
            .build()
            .unwrap();
        let vocabularies = self.get_vocabulary(vocabulary_query).await?;
        let mut vocabulary_id = 0;
        if vocabularies.len() > 1 {
            vocabulary_id = vocabularies[0].id;
        }

        let id: i64 = sqlx::query(
            r#"
            INSERT INTO learn_word (word, vocabulary_id, word_list_id, last_learned_at, next_learn_at)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id
            "#,
        )
        .bind(s.word.clone())
        .bind(vocabulary_id)
        .bind(word_list_id)
        .bind(abi::convert_to_utc_time(&s.last_learned_at.clone().unwrap()))
        .bind(abi::convert_to_utc_time(&s.next_learn_at.clone().unwrap()))
        .fetch_one(&self.pool)
        .await?
        .get(0);

        s.id = id;
        Ok(s)
    }

    /// query learn word
    async fn query_learn_word(
        &self,
        q: abi::LearnWordQuery,
    ) -> Result<Vec<abi::LearnWord>, abi::Error> {
        let condition = q.to_sql_condition();
        tracing::debug!("condition: {}", condition);
        let sql = format!("SELECT * FROM learn_word {}", condition);
        println!("condition: {}", sql);

        let result: Vec<abi::LearnWord> = sqlx::query_as(&sql).fetch_all(&self.pool).await?;

        Ok(result)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    async fn get_db(
        pool: &sqlx::PgPool,
        word: String,
        learn_status: abi::LearnStatus,
    ) -> Result<(ModelService, abi::LearnWord), abi::Error> {
        let db = ModelService::new(pool.clone());

        let v = abi::LearnWord {
            id: 0,
            word,
            vocabulary_id: 0,
            word_list_id: 0,
            learn_count: 0,
            learn_status: learn_status.into(),
            last_learned_at: Some(abi::convert_to_timestamp(&chrono::Utc::now())),
            next_learn_at: Some(abi::convert_to_timestamp(&chrono::Utc::now())),
            created_at: None,
            updated_at: None,
        };
        let v = db.add_learn_word(v).await?;
        Ok((db, v))
    }

    async fn get_apple_db(
        pool: &sqlx::PgPool,
    ) -> Result<(ModelService, abi::LearnWord), abi::Error> {
        get_db(pool, "apple".into(), abi::LearnStatus::New).await
    }

    #[sqlx_database_tester::test(pool(variable = "migrated_pool", migrations = "./migrations"))]
    async fn query_learn_word_should_work() {
        let (db, _v) = get_apple_db(&migrated_pool).await.unwrap();
        let q = abi::LearnWordQueryBuilder::default()
            .word(Some("apple".into()))
            .build()
            .unwrap();

        let records = db.query_learn_word(q).await.unwrap();
        assert_eq!(records.len(), 1);
        assert_eq!(records[0].word, "apple".to_string());
    }
}
