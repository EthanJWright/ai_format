use chatgpt::prelude::*;
use chatgpt::Result as ChatGptResult;
use chatgpt::types::CompletionResponse;
use futures::future::{try_join_all, TryFutureExt};
use tokio::task::spawn;

pub async fn process_chunks(key: &str, chunks: Vec<String>) -> ChatGptResult<Vec<CompletionResponse>> {
    let config = ModelConfigurationBuilder::default()
        .engine(ChatGPTEngine::Gpt35Turbo)
        .build()
        .unwrap();

    let client = match ChatGPT::new_with_config(key, config) {
        Ok(val) => val,
        Err(err) => panic!("Failed to create ChatGPT client: {}", err),
    };

    let tasks = chunks.into_iter().map(|chunk| {
        println!("Processing chunk...");
        let client = client.clone();
        spawn(async move { handle_chunk(client, chunk).await })
    });

    let responses: Vec<ChatGptResult<CompletionResponse>> = try_join_all(tasks)
        .map_ok(|results| {
            results.into_iter().collect::<Vec<_>>()
        })
        .await.unwrap();

    let unwrapped_responses: Vec<CompletionResponse> = responses.into_iter().map(|res| res.unwrap()).collect();

    println!("Done processing chunks");
    Ok(unwrapped_responses)
}

async fn handle_chunk(client: ChatGPT, chunk: String) -> ChatGptResult<CompletionResponse> {
    let message = format!("format as HTML\n\n{}", chunk);
    let response: CompletionResponse = client.send_message(&message).await?;
    Ok(response)
}

