pub mod ai {

    use dotenv::dotenv;
    use openai::set_key;
    use std::env;

    use std::io::{stdin, stdout, Write};

    use openai::chat::{ChatCompletion, ChatCompletionMessage, ChatCompletionMessageRole};

    pub async fn make_openai_call() {
        // dotenv().ok();

        // let api_key: String = env::var("API_KEY").expect("API_KEY not set in .env");
        // println!("\nAPI_KEY: {}\n", api_key);

        // set_key(env::var("OPENAI_KEY").unwrap());
    }

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
            let chat_completion = ChatCompletion::builder("gpt-3.5-turbo", messages.clone())
                .create()
                .await
                .unwrap();
            let returned_message = chat_completion.choices.first().unwrap().message.clone();

            println!(
                "ADA: {}\n",
                &returned_message.content.clone().unwrap().trim()
            );

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
}
