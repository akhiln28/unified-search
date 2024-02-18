use clap::Parser;

use search_google::{search, SearchRequest};

#[tokio::main]
async fn main() {
    let google_search_request: SearchRequest = SearchRequest::parse();
    let google_search_response = search(google_search_request).await.unwrap();
    for item in google_search_response.items {
        println!("Title: {}", item.title);
        println!("Link: {}", item.link);
        println!("Snippet: {}", item.snippet);
    }
}