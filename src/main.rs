mod data_management;
mod provider;

use clap::Parser;
use data_management::cli;
use data_management::conversation_data_struct::ConversationData;
use data_management::save_on_file::{conversation_mkdir, write_and_save};
use provider::ollama::Ollama;
use provider::provider::Provider;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let conversation_directory = conversation_mkdir()?;

    let user_input = cli::Interfaccia::parse();

    let starting_data = ConversationData {
        topic: user_input.topic,
        turns: user_input.turns,
        models: user_input.models,
    };

    if starting_data.models.is_empty() {
        return Err(
            "The user needs to provide at least a model to take part in the conversation.".into(),
        );
    }

    let mut conversation: Vec<String> = Vec::new();
    conversation.push(format!("Topic: {}", starting_data.topic));

    let user_input_json = serde_json::to_string_pretty(&starting_data)?;

    let input_path = conversation_directory.join("user_input.json");
    write_and_save(input_path, &user_input_json)?;

    let mut models: Vec<Ollama> = Vec::new();
    for m in &starting_data.models {
        let llm = Ollama::new("http://localhost:11434", m.clone());
        models.push(llm);
    }

    let last_model = models.last().unwrap();
    if models.len() == 1 {
        let prompt = format!(
            "You are {}.\nTalk about the following topic:\n{}\n\n",
            last_model.llm, starting_data.topic
        );

        let answer = last_model.identify_and_answer(&prompt).await?;

        let formatted = format!("{}: {}", last_model.llm, answer);
        println!("\n{}", formatted);
    } else {
        for _ in 0..starting_data.turns - 1 {
            for model in &models {
                let prompt = if conversation.len() == 1 {
                    format!(
                        "You are {}.\nStart a conversation on the following topic:\n{}\n\nAnswer first:",
                        model.llm, starting_data.topic
                    )
                } else {
                    format!(
                        "You are {}.\nThis is the conversation so far:\n{}\n\nContinue the conversation with a new intervention:",
                        model.llm,
                        conversation.join("\n")
                    )
                };

                let intervention = model.identify_and_answer(&prompt).await?;

                let formatted = format!("{}: {}", model.llm, intervention);
                println!("\n{}", formatted);

                conversation.push(formatted);
            }
        }

        for model in &models[0..models.len() - 1] {
            let prompt = if conversation.len() == 1 {
                format!(
                    "You are {}.\nStart a conversation on the following topic:\n{}\n\nAnswer first:",
                    model.llm, starting_data.topic
                )
            } else {
                format!(
                    "You are {}.\nThis is the conversation so far:\n{}\n\nContinue the conversation with a new intervention, this will be your last one:",
                    model.llm,
                    conversation.join("\n")
                )
            };

            let intervention = model.identify_and_answer(&prompt).await?;

            let formatted = format!("{}: {}", model.llm, intervention);
            println!("\n{}", formatted);

            conversation.push(formatted);
        }

        let prompt = format!(
            "You are {}.\nThis is the conversation so far:\n{}\n\nYou are the last one to intervene in the conversation, provide a summary of what has been said:",
            last_model.llm, starting_data.topic
        );

        let intervention = last_model.identify_and_answer(&prompt).await?;

        let formatted = format!("{}: {}", last_model.llm, intervention);
        println!("\n{}", formatted);

        conversation.push(formatted);
    }

    let conversation_txt = conversation.join("\n\n");

    let conversation_path = conversation_directory.join("conversation.txt");
    write_and_save(conversation_path, &conversation_txt)?;

    Ok(())
}
