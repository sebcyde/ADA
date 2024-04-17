pub mod ai {

    use dotenv::dotenv;
    use openai::set_key;
    use std::env;

    use std::io::{stdin, stdout, Write};

    use openai::chat::{
        ChatCompletion, ChatCompletionDelta, ChatCompletionMessage, ChatCompletionMessageRole,
    };

    use tokio::sync::mpsc::Receiver;

    use crate::config::ada_config::ada_config::SYSTEM_CONTENT;

    pub async fn primary_ada_builder() {
        dotenv().unwrap();
        set_key(env::var("OPENAI_KEY").unwrap());

        let mut messages: Vec<ChatCompletionMessage> = vec![
            ChatCompletionMessage {
                role: ChatCompletionMessageRole::System,
                content: Some(SYSTEM_CONTENT.to_string()),
                name: Some("ADA".to_string()),
                function_call: None,
            },
            ChatCompletionMessage {
                role: ChatCompletionMessageRole::User,
                name: Some("Sebastian".to_string()),
                content: Some("Hi ADA.".to_string()),
                function_call: None,
            },
        ];

        loop {
            let chat_stream: Receiver<
                openai::chat::ChatCompletionGeneric<openai::chat::ChatCompletionChoiceDelta>,
            > = ChatCompletionDelta::builder("gpt-3.5-turbo", messages.clone())
                .create_stream()
                .await
                .unwrap();

            let chat_completion: ChatCompletion = listen_for_tokens(chat_stream).await;
            let returned_message: ChatCompletionMessage =
                chat_completion.choices.first().unwrap().message.clone();

            messages.push(returned_message);

            print!("Seb: ");
            stdout().flush().unwrap();

            let mut user_message_content: String = String::new();

            stdin().read_line(&mut user_message_content).unwrap();
            messages.push(ChatCompletionMessage {
                role: ChatCompletionMessageRole::User,
                content: Some(user_message_content),
                name: None,
                function_call: None,
            });

            println!("");
        }
    }

    async fn listen_for_tokens(mut chat_stream: Receiver<ChatCompletionDelta>) -> ChatCompletion {
        let mut merged: Option<ChatCompletionDelta> = None;
        while let Some(delta) = chat_stream.recv().await {
            let choice = &delta.choices[0];
            if let Some(_role) = &choice.delta.role {
                print!("ADA: ");
            }
            if let Some(content) = &choice.delta.content {
                print!("{}", content);
            }
            if let Some(_) = &choice.finish_reason {
                print!("\n\n");
            }
            stdout().flush().unwrap();
            match merged.as_mut() {
                Some(c) => {
                    c.merge(delta).unwrap();
                }
                None => merged = Some(delta),
            };
        }
        merged.unwrap().into()
    }
}
