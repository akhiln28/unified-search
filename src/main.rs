#[macro_use] extern crate prettytable;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use prettytable::{Table, format};
use colored::Colorize;

#[derive(Debug, Deserialize, Serialize, Clone)]
struct YoutubeSearchResponse {
    kind: String,
    etag: String,
    #[serde(rename = "pageInfo")]
    page_info: PageInfo,
    items: Vec<YoutubeItem>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct PageInfo {
    total_results: Option<i64>,
    results_per_page: Option<i64>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct YoutubeItem {
    kind: String,
    etag: String,
    id: YoutubeId,
    snippet: YoutubeSnippet,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct YoutubeId {
    kind: String,
    #[serde(rename = "videoId")]
    video_id: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct YoutubeSnippet {
    #[serde(rename = "publishedAt")]
    published_at: String,
    title: String,
    description: String,
    #[serde(rename = "channelTitle")]
    channel_title: String,
    #[serde(rename = "publishTime")]
    publish_time: String,
}


#[tokio::main]
async fn main() {
    // https://youtube.googleapis.com/youtube/v3/search?part=snippet&order=viewCount&q=hello&type=video&videoDefinition=high&key=AIzaSyDVR9m1RNWnxr5bCFmJlTfmZ8Whj8LrNWA
    let youtube_folder = std::env::var("YOUTUBE_FOLDER").expect("YOUTUBE_FOLDER not set");
    let date_iso = chrono::Local::now().format("%Y-%m-%d").to_string() + ".json";
    let youtube_path = PathBuf::from(youtube_folder).join(date_iso);
    let google_search_api_key = std::env::var("GOOGLE_SEARCH_API_KEY").unwrap();

    let youtube_file = match std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(youtube_path)
    {
        Ok(file) => file,
        Err(e) => {
            println!("Error opening file: {}", e);
            return;
        }
    };

    let args = std::env::args().skip(1).collect::<Vec<String>>();
    let prompt = args.join(" ");

    let url = "https://youtube.googleapis.com/youtube/v3/search";
    let client = reqwest::Client::new();
    let params = [
        ("part", "snippet"),
        ("order", "viewCount"),
        ("q", prompt.as_str()),
        ("type", "video"),
        ("videoDefinition", "high"),
        ("key", google_search_api_key.as_str())
    ];
    let youtube_response: YoutubeSearchResponse = client
        .get(url)
        .query(&params)
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    // print!("{:?}", youtube_response);
    let mut youtube_history: Vec<YoutubeSearchResponse> = serde_json::from_reader(&youtube_file).unwrap_or_default();
    youtube_history.push(youtube_response.clone());
    serde_json::to_writer_pretty(youtube_file, &youtube_history).unwrap();
    // print items
    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
    table.set_titles(row!["Title", "Channel", "Link"]);
    for item in youtube_response.items {
        let link = format!("https://www.youtube.com/watch?v={}", &item.id.video_id);
        table.add_row(row![item.snippet.title.magenta(), item.snippet.channel_title.green(), link.blue()]);
    }
    table.printstd();
}
