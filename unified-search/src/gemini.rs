use google_generative_ai_rs::v1::{api::Client, gemini::{request::Request, Content, Part, Role}};

pub async fn search(query: &str) -> Result<(), Box<dyn std::error::Error>>{
    let google_generative_api_key =
        std::env::var("GOOGLE_GENERATIVE_API_KEY").expect("GOOGLE_GENERATIVE_API_KEY must be set");
    let client: Client = Client::new(google_generative_api_key);
    let txt_request = Request {
        contents: vec![Content {
            role: Role::User,
            parts: vec![Part {
                text: Some(query.to_string()),
                inline_data: None,
                file_data: None,
                video_metadata: None,
            }],
        }],
        tools: vec![],
        safety_settings: vec![],
        generation_config: None,
    };
    let response = client.post(30, &txt_request).await?;
    println!("{:#?}", response);
    Ok(())
}
