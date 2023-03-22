use abi::vocabulary_service_server::VocabularyService;
use tonic::{async_trait, Request, Response, Status};

use crate::{vocabulary::VocabularyTrait, OrionService};

#[async_trait]
impl VocabularyService for OrionService {
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
}
