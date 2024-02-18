use clap::Parser;
use search_youtube::{search_youtube, YoutubeSearchRequest};

#[tokio::main]
async fn main() {
    let youtube_search_request: YoutubeSearchRequest = YoutubeSearchRequest::parse();
    let youtube_search_response = search_youtube(youtube_search_request).await.unwrap();
    for item in youtube_search_response.items {
        println!("Title: {}", item.snippet.title);
        println!("Description: {}", item.snippet.description);
        println!("Published at: {}", item.snippet.published_at);
        println!("Channel title: {}", item.snippet.channel_title);
        if let Some(channel_id) = item.id.channel_id {
            println!(
                "Channel url: https://www.youtube.com/channel/{}",
                channel_id
            );
        }
        if let Some(video_id) = item.id.video_id {
            println!("Video url: https://www.youtube.com/watch?v={}", video_id);
        }
    }
}
