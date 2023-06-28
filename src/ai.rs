use chatgpt::prelude::*; use chatgpt::Result as ChatGptResult;
use chatgpt::types::CompletionResponse;
use futures::future::{try_join_all, TryFutureExt};
use tokio::task::spawn;

pub async fn process_chunks(key: &str, prompt: String, chunks: Vec<String>) -> ChatGptResult<Vec<CompletionResponse>> {
    let config = ModelConfigurationBuilder::default()
        .engine(ChatGPTEngine::Gpt35Turbo)
        .build()
        .unwrap();

    let client = match ChatGPT::new_with_config(key, config) {
        Ok(val) => val,
        Err(err) => panic!("Failed to create ChatGPT client: {}", err),
    };

    let tasks = chunks.into_iter().map(|chunk| {
        let client = client.clone();
        let prompt = prompt.clone();
        spawn(async move { handle_chunk(client, chunk, prompt).await })
    });

    let responses: Vec<ChatGptResult<CompletionResponse>> = try_join_all(tasks)
        .map_ok(|results| {
            results.into_iter().collect::<Vec<_>>()
        })
        .await.unwrap();

    // change above to a reduce, where errors are dropped
    let mut unwrapped_responses: Vec<CompletionResponse> = Vec::new();
    for res in responses {
        match res {
            Ok(response) => {
                unwrapped_responses.push(response);
            },
            Err(err) => {
                println!("Error: {}", err);
            }
        }
    }

    Ok(unwrapped_responses)
}

async fn handle_chunk(client: ChatGPT, chunk: String, prompt: String) -> ChatGptResult<CompletionResponse> {
    let message = format!("{}\n\n{}", prompt, chunk);
    let response: CompletionResponse = client.send_message(&message).await?;
    Ok(response)
}

