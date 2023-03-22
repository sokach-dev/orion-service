use crate::{story::StoryTrait, OrionService};
use abi::story_service_server::StoryService;
use tonic::{async_trait, Request, Response, Status};

#[async_trait]
impl StoryService for OrionService {
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
}
