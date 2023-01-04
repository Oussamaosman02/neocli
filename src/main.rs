use reqwest::{Client, Error, Response};

use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Serialize, Deserialize)]
struct Parameters {
    model: String,
    prompt: String,
    temperature: f64,
    max_tokens: i32,
    top_p: u8,
    frequency_penalty: f64,
    presence_penalty: f64,
    stop: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Choice {
    text: String,
    index: u8,
    logprobs: Option<i32>,
    finish_reason: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Uso {
    prompt_tokens: i32,
    completion_tokens: i32,
    total_tokens: i32,
}

#[derive(Debug, Serialize, Deserialize)]
struct OpenAIResponse {
    id: String,
    object: String,
    created: u64,
    model: String,
    choices: Vec<Choice>,
    usage: Uso,
}
fn some() {
    let messi = OpenAIResponse {
        id: String::from("cmpl-6UblIybqghmJKD2OSIzM9HEEMNUau"),
        object: String::from("text_completion"),
        created: 1672753016,
        model: String::from("text-davinci-003"),
        choices: vec![Choice {
            text: String::from("Hi"),
            index: 0,
            logprobs: Some(9),
            finish_reason: String::from("Stop"),
        }],
        usage: Uso {
            prompt_tokens: 1,
            completion_tokens: 1,
            total_tokens: 1,
        },
    };

    assert_eq!(messi.id, "cmpl-6UblIybqghmJKD2OSIzM9HEEMNUau");
    assert_eq!(messi.object, "text_completion");
    assert_eq!(messi.created, 1672753016);
    assert_eq!(messi.model, "text-davinci-003");
    assert_eq!(messi.choices[0].finish_reason, "Stop");
    assert_eq!(messi.choices[0].index, 0);
    assert_eq!(messi.choices[0].logprobs, Some(9));
    assert_eq!(messi.usage.completion_tokens, 1);
    assert_eq!(messi.usage.total_tokens, 1);
    assert_eq!(messi.usage.prompt_tokens, 1);
}
#[tokio::main]
async fn main() -> Result<(), Error> {
    let tokenf = env::var("AI_TOKEN");
    let token;
    match tokenf {
        Ok(t) => token = t,
        Err(e) => panic!("{}", e),
    };
    let texto = String::from("Bearer ") + &token;
    let mut args: Vec<String> = env::args().collect();

    args.remove(0);
    let respx: String = args.join(" ");
    if respx.len() > 1 {
        let respu: String = format!(
        "Seguidamente, una conversación con un asistente de IA. Es útil, creativo, eficaz.\nHumano: {}.\nIA:",
        respx
    );
        let prompt = Parameters {
            model: String::from("text-davinci-003"),
            prompt: respu,
            temperature: 0.9,
            max_tokens: 1024,
            top_p: 1,
            frequency_penalty: 0.0,
            presence_penalty: 0.6,
            stop: vec![String::from("\n")],
        };

        let client = Client::new();
        let resp: Response = client
            .post("https://api.openai.com/v1/completions")
            .header("Content-Type", "application/json")
            .header("Authorization", texto)
            .json(&prompt)
            .send()
            .await?;

        let openai_response: OpenAIResponse = match resp.json().await {
            Ok(response) => response,
            Err(error) => {
                format!("{:#?}", error);
                OpenAIResponse {
                    id: String::from("cmpl-6UblIybqghmJKD2OSIzM9HEEMNUau"),
                    object: String::from("text_completion"),
                    created: 1672753016,
                    model: String::from("text-davinci-003"),
                    choices: vec![Choice {
                        text: String::from(
                            "Ni idea...Reformula la pregunta y vuelve a intentarlo!",
                        ),
                        index: 0,
                        logprobs: Some(9),
                        finish_reason: String::from("Stop"),
                    }],
                    usage: Uso {
                        prompt_tokens: 1,
                        completion_tokens: 0,
                        total_tokens: 1,
                    },
                }
            }
        };
        let final_resp = format!("{}", openai_response.choices[0].text);
        println!("{}", final_resp);
    } else {
        let respuesta = format!(
            "{} \n{} \n{} \n{} \n{}",
            "Soy una CLI (Command Line Interface) con una IA integrada (GPT-3).",
            "Dime tus preguntas y te las intentaré resolver!",
            "Puedes usarme con o sin comillas:",
            "  neocli \"¿Para que sirve grep?\"",
            "  neocli ¿Para que sirve grep?"
        );
        println!("{}", respuesta);
    }
    some();

    Ok(())
}
