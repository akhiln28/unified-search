mod gemini;
mod gpt;

use clap::{Parser, ValueEnum};

#[derive(Debug, Parser)]
pub struct Input {
    #[clap(short, long, help = "The source to search from")]
    pub source: Option<Source>,
    #[clap(short, long, help = "The model to use")]
    pub model: Option<String>,
    #[clap(short, long, help = "The query to search for")]
    pub query: String,
}

#[derive(Debug, Clone, Parser, ValueEnum)]
pub enum Source {
    Gemini,
    Gpt,
    Google,
    Youtube,
    Github,
    Reddit,
    Wikipedia
}

#[tokio::main]
async fn main() {
    let input = Input::parse();
    match input.source {
        Some(Source::Gemini) => {
            gemini::search(&input.query).await.unwrap();
        }
        Some(Source::Gpt) => {
            // gpt::search(input.query, input.model.unwrap()).await;
        }
        _ => {
            println!("No source specified");
        }
    }
}
