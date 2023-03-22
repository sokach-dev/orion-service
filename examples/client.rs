use abi::vocabulary_service_client::VocabularyServiceClient;
use abi::{QueryVocabularyRequest, VocabularyQuery};
use tonic::transport::Channel;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client: VocabularyServiceClient<Channel> =
        VocabularyServiceClient::connect("http://127.0.0.1:5015").await?;

    let request = tonic::Request::new(QueryVocabularyRequest {
        query: Some(VocabularyQuery {
            word: Some("consist".to_string()),
            ..Default::default()
        }),
    });

    let response = client.query_vocabulary(request).await?;
    let response = response.into_inner();

    println!("RESPONSE={:#?}", response);

    Ok(())
}
