use chatgpt::prelude::*;
use chatgpt::Result as ChatResult; // Import the Result type with a different name
use chatgpt::types::CompletionResponse;
use futures::future::{try_join_all};

pub async fn process_chunks(key: &str, chunks: Vec<String>) -> ChatResult<Vec<CompletionResponse>> {
    let config = ModelConfigurationBuilder::default()
        .engine(ChatGPTEngine::Gpt35Turbo)
        .build()
        .unwrap();

    let client = match ChatGPT::new_with_config(key, config) {
        Ok(val) => val,
        Err(err) => panic!("Failed to create ChatGPT client: {}", err),
    };

    let responses = try_join_all(chunks.into_iter().map(|chunk| handle_chunk(client.clone(), chunk))).await?;
    Ok(responses)
}

async fn handle_chunk(client: ChatGPT, chunk: String) -> ChatResult<CompletionResponse> {
    let message = format!("format as HTML\n\n{}", chunk);
    let response: CompletionResponse = client.send_message(&message).await?;
    Ok(response)
}


