use davinci::davinci;
use std::env;

fn main() {
    // Reads the OpenAi API from enviroment variables
    let token_exists = env::var("AI_TOKEN");
    let token;
    match token_exists {
        Ok(t) => token = t,
        Err(e) => panic!("API token does not exist as AI_TOKEN env variable: {}", e),
    };

    // Read arguments
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);
    let query: String = args.join(" ");

    // if arguments there are arguments
    if query.len() > 1 {
        // max. number of tokens
        let max_tokens: i32 = 2048;

        // declaring context
        let context: String = String::from(
            "Seguidamente, una conversación con un asistente de IA. Es útil, creativo y eficaz.",
        );

        // Passing arguments to davinci
        let response: String = match davinci(token, context, query, max_tokens) {
            Ok(res) => res,
            Err(error) => error.to_string(),
        };
        println!("{}", response);
    } else {
        let response = format!(
            "{} \n{} \n{} \n{} \n{}",
            "Soy una CLI (Command Line Interface) con una IA integrada (GPT-3).",
            "Dime tus preguntas y te las intentaré resolver!",
            "Puedes usarme con o sin comillas:",
            "  neocli \"¿Para que sirve grep?\"",
            "  neocli ¿Para que sirve grep?"
        );
        println!("{}", response);
    }
}
