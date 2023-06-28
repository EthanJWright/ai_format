use chatgpt::prelude::*;
use chatgpt::Result as ChatResult;
use chatgpt::types::CompletionResponse;
use tokio::task::{spawn};


pub async fn process_chunks(key: &str, chunks: Vec<String>) -> ChatResult<Vec<CompletionResponse>> {
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


    let mut responses = Vec::new();
    for task in tasks {
        let result = task.await.unwrap().unwrap();
        println!("Adding response");
        responses.push(result);
    }


    println!("Done processing chunks");
    Ok(responses)
}

async fn handle_chunk(client: ChatGPT, chunk: String) -> ChatResult<CompletionResponse> {
    let message = format!("format as HTML\n\n{}", chunk);
    let response: CompletionResponse = client.send_message(&message).await?;
    Ok(response)
}
