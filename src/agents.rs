use async_openai::types::{
    ChatCompletionRequestMessage, ChatCompletionRequestSystemMessageArgs,
    ChatCompletionRequestUserMessageArgs, CreateChatCompletionRequestArgs,
};
use async_openai::{config::OpenAIConfig, Client as OpenAIClient};

#[derive(Clone)]
pub struct MyAgent {
    openai_client: OpenAIClient<OpenAIConfig>,
}

impl Default for MyAgent {
    fn default() -> Self {
        Self::new()
    }
}

impl MyAgent {
    pub fn new() -> Self {
        let api_key = std::env::var("OPENAI_API_KEY").unwrap();
        let config = OpenAIConfig::new().with_api_key(api_key);

        let openai_client = OpenAIClient::with_config(config);

        Self { openai_client }
    }

    fn system_message(&self) -> String {
        "You are an AI agent.

        Your job is to do whatever the user asks you to, following the provided context.
        If you don't know something, don't make something up but instead respond 'I don't know.'
"
        .to_string()
    }

    pub async fn prompt(&self, prompt: &str) -> anyhow::Result<String> {
        let res = self
            .openai_client
            .chat()
            .create(
                CreateChatCompletionRequestArgs::default()
                    .model("gpt-4o")
                    .messages(vec![
                        //First we add the system message to define what the Agent does
                        ChatCompletionRequestMessage::System(
                            ChatCompletionRequestSystemMessageArgs::default()
                                .content(&self.system_message())
                                .build()?,
                        ),
                        //Then we add our prompt
                        ChatCompletionRequestMessage::User(
                            ChatCompletionRequestUserMessageArgs::default()
                                .content(prompt)
                                .build()?,
                        ),
                    ])
                    .build()?,
            )
            .await
            .map(|res| {
                //We extract the first one
                res.choices[0].message.content.clone().unwrap()
            })?;

        println!("Retrieved result from prompt: {res}");

        Ok(res)
    }
}
