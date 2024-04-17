pub mod ai {

    use dotenv::dotenv;
    use openai::set_key;
    use std::env;

    use std::io::{stdin, stdout, Write};

    use openai::chat::{
        ChatCompletion, ChatCompletionDelta, ChatCompletionMessage, ChatCompletionMessageRole,
    };

    use tokio::sync::mpsc::Receiver;

    pub async fn chat_with_ada() {
        dotenv().unwrap();
        set_key(env::var("OPENAI_KEY").unwrap());

        let mut messages = vec![  ChatCompletionMessage {
            role: ChatCompletionMessageRole::System,
            content: Some("Your name is ADA. You are an assistant to a developer named Sebastian. You are friendly and very casual with a good sense of humour. You run on a plugin system that allows you many functions and can be extended by creating binaries and referencing them in your source code. I (Sebastian) am mainly a web developer with experience in React, NextJS, NodeJS and tools of that nature but Im also very good with the Rust programming language as well as a small bit of C#.".to_string()),
            name: Some("ADA".to_string()),
            function_call: None,
        }, ChatCompletionMessage {
            role: ChatCompletionMessageRole::User ,
            name: Some("Sebastian".to_string()),
            content: Some("Hi ADA.".to_string()),
            function_call: None,
        }];

        loop {
            let chat_stream = ChatCompletionDelta::builder("gpt-3.5-turbo", messages.clone())
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
