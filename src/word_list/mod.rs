use crate::ModelService;
use sqlx::Row;
use tonic::async_trait;

#[async_trait]
pub trait WordListTrait {
    /// make a word list
    async fn add_word_list(&self, mut s: abi::WordList) -> Result<abi::WordList, abi::Error>;
    /// query word list
    async fn query_word_list(
        &self,
        q: abi::WordListQuery,
    ) -> Result<Vec<abi::WordList>, abi::Error>;
}

#[async_trait]
impl WordListTrait for ModelService {
    /// make a word list
    async fn add_word_list(&self, mut s: abi::WordList) -> Result<abi::WordList, abi::Error> {
        let class = abi::WordClassification::from_i32(s.classification).unwrap();

        let id: i64 = sqlx::query(
            r#"
            INSERT INTO orion.word_list (word, paraphrase, classification)
            VALUES ($1, $2, $3::word_classification)
            RETURNING id
            "#,
        )
        .bind(s.word.clone())
        .bind(s.paraphrase.clone())
        .bind(class.to_string())
        .fetch_one(&self.pool)
        .await?
        .get(0);

        s.id = id;
        Ok(s)
    }

    /// query word list
    async fn query_word_list(
        &self,
        q: abi::WordListQuery,
    ) -> Result<Vec<abi::WordList>, abi::Error> {
        let condition = q.to_sql_condition();
        tracing::debug!("condition: {}", condition);
        let sql = format!("SELECT * FROM word_list {}", condition);
        println!("condition: {}", sql);

        let result: Vec<abi::WordList> = sqlx::query_as(&sql).fetch_all(&self.pool).await?;

        Ok(result)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    async fn get_db(
        pool: &sqlx::PgPool,
        word: String,
        paraphrase: String,
        classification: abi::WordClassification,
    ) -> Result<(ModelService, abi::WordList), abi::Error> {
        let db = ModelService::new(pool.clone());

        let v = abi::WordList {
            id: 0,
            word,
            paraphrase,
            classification: classification.into(),
            created_at: None,
            updated_at: None,
        };
        let v = db.add_word_list(v).await?;
        Ok((db, v))
    }

    async fn get_apple_db(
        pool: &sqlx::PgPool,
    ) -> Result<(ModelService, abi::WordList), abi::Error> {
        get_db(
            pool,
            "apple".into(),
            "苹果".into(),
            abi::WordClassification::Junior,
        )
        .await
    }

    #[sqlx_database_tester::test(pool(variable = "migrated_pool", migrations = "./migrations"))]
    async fn query_word_list_should_work() {
        let (db, _v) = get_apple_db(&migrated_pool).await.unwrap();
        let q = abi::WordListQueryBuilder::default()
            .word(Some("apple".into()))
            .build()
            .unwrap();

        let records = db.query_word_list(q).await.unwrap();
        assert_eq!(records.len(), 1);
        assert_eq!(records[0].word, "apple".to_string());
    }
}
