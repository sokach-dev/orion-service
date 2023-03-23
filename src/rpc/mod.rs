use tonic::{async_trait, Request, Response, Status};

use crate::{
    learn_word::LearnWordTrait, story::StoryTrait, vocabulary::VocabularyTrait,
    word_list::WordListTrait, OrionService,
};

#[async_trait]
impl abi::orion_service_server::OrionService for OrionService {
    async fn add_vocabulary(
        &self,
        request: Request<abi::AddVocabularyRequest>,
    ) -> Result<Response<abi::VocabularyResponse>, Status> {
        let req = request.into_inner();

        if req.vocabulary.is_none() {
            return Err(Status::invalid_argument("missing vocabulary"));
        }
        let v = self.model.add_vocabulary(req.vocabulary.unwrap()).await?;

        Ok(Response::new(abi::VocabularyResponse {
            vocabulary: Some(v),
        }))
    }

    async fn query_vocabulary(
        &self,
        request: Request<abi::QueryVocabularyRequest>,
    ) -> Result<Response<abi::QueryVocabularyResponse>, Status> {
        let req = request.into_inner();

        if req.query.is_none() {
            return Err(Status::invalid_argument("missing query parameters"));
        }

        let v = self.model.get_vocabulary(req.query.unwrap()).await?;

        Ok(Response::new(abi::QueryVocabularyResponse {
            vocabulary: v,
        }))
    }

    async fn query_vocabulary_random(
        &self,
        request: Request<abi::QueryVocabularyRandomRequest>,
    ) -> Result<Response<abi::QueryVocabularyResponse>, Status> {
        let req = request.into_inner();

        if req.limit < 0 {
            return Err(Status::invalid_argument("missing query parameters"));
        }

        let v = self.model.get_random_vocabularys(req.limit).await?;

        Ok(Response::new(abi::QueryVocabularyResponse {
            vocabulary: v,
        }))
    }

    /// add a new story
    async fn add_story(
        &self,
        request: Request<abi::AddStoryRequest>,
    ) -> Result<Response<abi::StoryResponse>, Status> {
        let req = request.into_inner();

        if req.story.is_none() {
            return Err(Status::invalid_argument("missing story"));
        }

        let v = self.model.add_story(req.story.unwrap()).await?;

        Ok(Response::new(abi::StoryResponse { story: Some(v) }))
    }

    /// add a new word list
    async fn add_word_list(
        &self,
        request: Request<abi::AddWordListRequest>,
    ) -> Result<Response<abi::WordListResponse>, Status> {
        let req = request.into_inner();

        if req.word.is_none() {
            return Err(Status::invalid_argument("missing word_list"));
        }

        let v = self.model.add_word_list(req.word.unwrap()).await?;

        Ok(Response::new(abi::WordListResponse { word: Some(v) }))
    }

    /// query word list
    async fn query_word_list(
        &self,
        request: Request<abi::QueryWordListRequest>,
    ) -> Result<Response<abi::QueryWordListResponse>, Status> {
        let req = request.into_inner();

        if req.query.is_none() {
            return Err(Status::invalid_argument(
                "missing word list query parameters",
            ));
        }

        let v = self.model.query_word_list(req.query.unwrap()).await?;

        Ok(Response::new(abi::QueryWordListResponse { word: v }))
    }

    /// add a new learn word
    async fn add_learn_word(
        &self,
        request: Request<abi::AddLearnWordRequest>,
    ) -> Result<Response<abi::LearnWordResponse>, Status> {
        let req = request.into_inner();

        if req.word.is_none() {
            return Err(Status::invalid_argument("missing learn_word"));
        }

        let v = self.model.add_learn_word(req.word.unwrap()).await?;

        Ok(Response::new(abi::LearnWordResponse { word: Some(v) }))
    }

    /// query learn word
    async fn query_learn_word(
        &self,
        request: Request<abi::QueryLearnWordRequest>,
    ) -> Result<Response<abi::QueryLearnWordResponse>, Status> {
        let req = request.into_inner();

        if req.query.is_none() {
            return Err(Status::invalid_argument(
                "missing learn word query parameters",
            ));
        }

        let v = self.model.query_learn_word(req.query.unwrap()).await?;

        Ok(Response::new(abi::QueryLearnWordResponse { word: v }))
    }
}
