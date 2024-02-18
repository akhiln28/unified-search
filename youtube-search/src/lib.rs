use std::fmt::Display;

use clap::{Parser, ValueEnum};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct YoutubeSearchResponse {
    pub kind: String,
    pub etag: String,
    #[serde(rename = "pageInfo")]
    pub page_info: PageInfo,
    pub items: Vec<YoutubeItem>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PageInfo {
    pub total_results: Option<i64>,
    pub results_per_page: Option<i64>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct YoutubeItem {
    pub kind: String,
    pub etag: String,
    pub id: YoutubeId,
    pub snippet: YoutubeSnippet,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct YoutubeId {
    pub kind: String,
    #[serde(rename = "videoId")]
    pub video_id: Option<String>,
    #[serde(rename = "channelId")]
    pub channel_id: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct YoutubeSnippet {
    #[serde(rename = "publishedAt")]
    pub published_at: String,
    pub title: String,
    pub description: String,
    #[serde(rename = "channelTitle")]
    pub channel_title: String,
    #[serde(rename = "publishTime")]
    pub publish_time: String,
}

#[derive(Debug, Serialize, Clone, Parser)]
pub struct YoutubeSearchRequest {
    #[clap(
        short,
        long,
        help = "The part parameter specifies a comma-separated list of one or more search resource properties that the API response will include",
        default_value = "snippet"
    )]
    pub part: Option<String>,
    #[clap(
        short,
        long,
        help = "The channel_id parameter indicates that the API response should only contain resources created by the channel",
    )]
    pub channel_id: Option<String>,
    #[clap(
        short,
        long,
        help = "The channel_type parameter lets you restrict a search to a particular type of channel (any or show)",
    )]
    pub channel_type: Option<ChannelType>,
    #[clap(
        short,
        long,
        help = "The event_type parameter restricts a search to broadcast events",
    )]
    pub event_type: Option<EventType>,
    #[clap(
        short,
        long,
        help = "The location parameter restricts a search to videos that have a geographical location e.g.(37.42307,-122.08427)",
    )]
    pub location: Option<String>,
    #[clap(
        short,
        long,
        help = "The location_radius parameter specifies the maximum distance that the location associated with a video can be from the specified location",
    )]
    pub location_radius: Option<String>,
    #[clap(
        short,
        long,
        help = "The max_results parameter specifies the maximum number of items that should be returned in the result set",
        default_value = "25"
    )]
    pub max_results: Option<i64>,
    #[clap(
        short,
        long,
    )]
    pub on_behalf_of_content_owner: Option<String>,
    #[clap(
        short,
        long,
        help = "The order parameter specifies the method that will be used to order resources in the API response (date, rating, relevance, title, videoCount, viewCount)",
        default_value = "relevance"
    )]
    pub order: Option<Order>,
    #[clap(
        short,
        long,
        help = "The page_token parameter identifies a specific page in the result set that should be returned",
    )]
    pub page_token: Option<String>,
    #[clap(
        short,
        long,
        help = "The published_after parameter indicates that the API response should only contain resources created after this date (RFC 3339)",
    )]
    pub published_after: Option<String>,
    #[clap(
        short,
        long,
        help = "The published_before parameter indicates that the API response should only contain resources created before this date (RFC 3339)",
    )]
    pub published_before: Option<String>,
    #[clap(
        short,
        long,
        help = "The q parameter specifies the query term to search for e.g. (cats|dogs)",
    )]
    pub q: Option<String>,
    #[clap(
        short,
        long,
        help = "The region_code parameter instructs the API to return search results for the specified country",
    )]
    pub region_code: Option<String>,
    #[clap(
        short,
        long,
        help = "The relevance_language parameter instructs the API to return search results that are most relevant to the specified language",
    )]
    pub relevance_language: Option<String>,
    #[clap(
        short,
        long,
        help = "The safe_search parameter indicates whether the search results should include restricted content as well as standard content",
        default_value = "moderate"
    )]
    pub safe_search: Option<SafeSearch>,
    #[clap(
        long,
        help = "The topic_id parameter indicates that the API response should only contain resources associated with the specified topic",
    )]
    pub topic_id: Option<Topic>,
    #[clap(
        short,
        long,
        help = "The type parameter restricts a search query to only retrieve a particular type of resource (channel, playlist, video)",
    )]
    pub type_: Option<Type>,
    #[clap(
        short,
        long,
        help = "The video_caption parameter indicates whether the API should filter video search results based on whether they have captions",
    )]
    pub video_caption: Option<VideoCaption>,
    #[clap(
        short,
        long,
        help = "The video_category_id parameter filters video search results based on their category",
    )]
    pub video_category_id: Option<String>,
    #[clap(
        short,
        long,
        help = "The video_definition parameter lets you restrict a search to only include either high definition (HD) or standard definition (SD) videos (any, high, standard)",
    )]
    pub video_definition: Option<VideoDefinition>,
    #[clap(
        short,
        long,
        help = "The video_dimension parameter lets you restrict a search to only retrieve 2D or 3D videos (any, 2d, 3d)",
    )]
    pub video_dimension: Option<VideoDimension>,
    #[clap(
        short,
        long,
        help = "The video_duration parameter filters video search results based on their duration (any, long, medium, short)",
    )]
    pub video_duration: Option<VideoDuration>,
    #[clap(
        short,
        long,
        help = "The video_embeddable parameter lets you to restrict a search to only videos that can be embedded into a webpage (any, true)",
    )]
    pub video_embeddable: Option<VideoEmbeddable>,
    #[clap(
        short,
        long,
        help = "The video_license parameter filters search results to only include videos with a particular license (any, creativeCommon, youtube)",
    )]
    pub video_license: Option<VideoLicense>,
    #[clap(
        short,
        long,
        help = "The video_paid_product_placement parameter lets you to restrict a search to only videos that contain a paid product placement (any, true)",
    )]
    pub video_paid_product_placement: Option<VideoPaidProductPlacement>,
    #[clap(
        short,
        long,
        help = "The video_syndicated parameter lets you to restrict a search to only videos that can be played outside youtube.com (any, true)",
    )]
    pub video_syndicated: Option<VideoSyndicated>,
    #[clap(
        short,
        long,
        help = "The video_type parameter lets you restrict a search to a particular type of videos (any, episode, movie)",
    )]
    pub video_type: Option<VideoType>,
}

#[derive(Debug, Serialize, Clone, ValueEnum)]
pub enum VideoType {
    #[serde(rename = "any")]
    Any,
    #[serde(rename = "episode")]
    Episode,
    #[serde(rename = "movie")]
    Movie,
}

impl Display for VideoType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VideoType::Any => write!(f, "any"),
            VideoType::Episode => write!(f, "episode"),
            VideoType::Movie => write!(f, "movie"),
        }
    }
}

#[derive(Debug, Serialize, Clone, ValueEnum)]
pub enum VideoSyndicated {
    #[serde(rename = "any")]
    Any,
    #[serde(rename = "true")]
    True,
}

impl Display for VideoSyndicated {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VideoSyndicated::Any => write!(f, "any"),
            VideoSyndicated::True => write!(f, "true"),
        }
    }
}

#[derive(Debug, Serialize, Clone, ValueEnum)]
pub enum VideoPaidProductPlacement {
    #[serde(rename = "any")]
    Any,
    #[serde(rename = "true")]
    True,
}

impl Display for VideoPaidProductPlacement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VideoPaidProductPlacement::Any => write!(f, "any"),
            VideoPaidProductPlacement::True => write!(f, "true"),
        }
    }
}

#[derive(Debug, Serialize, Clone, ValueEnum)]
pub enum VideoLicense {
    #[serde(rename = "any")]
    Any,
    #[serde(rename = "creativeCommon")]
    CreativeCommon,
    #[serde(rename = "youtube")]
    Youtube,
}

impl Display for VideoLicense {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VideoLicense::Any => write!(f, "any"),
            VideoLicense::CreativeCommon => write!(f, "creativeCommon"),
            VideoLicense::Youtube => write!(f, "youtube"),
        }
    }
}

#[derive(Debug, Serialize, Clone, ValueEnum)]
pub enum VideoEmbeddable {
    #[serde(rename = "any")]
    Any,
    #[serde(rename = "true")]
    True,
}

impl Display for VideoEmbeddable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VideoEmbeddable::Any => write!(f, "any"),
            VideoEmbeddable::True => write!(f, "true"),
        }
    }
}

#[derive(Debug, Serialize, Clone, ValueEnum)]
pub enum VideoDefinition {
    #[serde(rename = "any")]
    Any,
    #[serde(rename = "high")]
    High,
    #[serde(rename = "standard")]
    Standard,
}

impl Display for VideoDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VideoDefinition::Any => write!(f, "any"),
            VideoDefinition::High => write!(f, "high"),
            VideoDefinition::Standard => write!(f, "standard"),
        }
    }
}

#[derive(Debug, Serialize, Clone, ValueEnum)]
pub enum VideoCaption {
    #[serde(rename = "any")]
    Any,
    #[serde(rename = "closedCaption")]
    ClosedCaption,
    #[serde(rename = "none")]
    None,
}

impl Display for VideoCaption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VideoCaption::Any => write!(f, "any"),
            VideoCaption::ClosedCaption => write!(f, "closedCaption"),
            VideoCaption::None => write!(f, "none"),
        }
    }
}

#[derive(Debug, Serialize, Clone, ValueEnum)]
pub enum Type {
    #[serde(rename = "channel")]
    Channel,
    #[serde(rename = "playlist")]
    Playlist,
    #[serde(rename = "video")]
    Video,
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::Channel => write!(f, "channel"),
            Type::Playlist => write!(f, "playlist"),
            Type::Video => write!(f, "video"),
        }
    }
}

#[derive(Debug, Serialize, Clone, ValueEnum)]
pub enum VideoDuration {
    #[serde(rename = "any")]
    Any,
    #[serde(rename = "long")]
    Long,
    #[serde(rename = "medium")]
    Medium,
    #[serde(rename = "short")]
    Short,
}

impl Display for VideoDuration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VideoDuration::Any => write!(f, "any"),
            VideoDuration::Long => write!(f, "long"),
            VideoDuration::Medium => write!(f, "medium"),
            VideoDuration::Short => write!(f, "short"),
        }
    }
}

#[derive(Debug, Serialize, Clone, ValueEnum)]
pub enum VideoDimension {
    #[serde(rename = "any")]
    Any,
    #[serde(rename = "2d")]
    _2d,
    #[serde(rename = "3d")]
    _3d,
}

impl Display for VideoDimension {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VideoDimension::Any => write!(f, "any"),
            VideoDimension::_2d => write!(f, "2d"),
            VideoDimension::_3d => write!(f, "3d"),
        }
    }
}

#[derive(Debug, Serialize, Clone, ValueEnum)]
pub enum Topic {
    #[serde(rename = "/m/04rlf")]
    Music,
    #[serde(rename = "/m/02mscn")]
    ChristianMusic,
    #[serde(rename = "/m/0ggq0m")]
    ClassicalMusic,
    #[serde(rename = "/m/01lyv")]
    Country,
    #[serde(rename = "/m/02lkt")]
    ElectronicMusic,
    #[serde(rename = "/m/0glt670")]
    HipHopMusic,
    #[serde(rename = "/m/05rwpb")]
    IndependentMusic,
    #[serde(rename = "/m/03_d0")]
    Jazz,
    #[serde(rename = "/m/028sqc")]
    MusicOfAsia,
    #[serde(rename = "/m/0g293")]
    MusicOfLatinAmerica,
    #[serde(rename = "/m/064t9")]
    PopMusic,
    #[serde(rename = "/m/06cqb")]
    Reggae,
    #[serde(rename = "/m/06j6l")]
    RhythmAndBlues,
    #[serde(rename = "/m/06by7")]
    RockMusic,
    #[serde(rename = "/m/0gywn")]
    SoulMusic,
    #[serde(rename = "/m/0bzvm2")]
    Gaming,
    #[serde(rename = "/m/025zzc")]
    ActionGame,
    #[serde(rename = "/m/02ntfj")]
    ActionAdventureGame,
    #[serde(rename = "/m/0b1vjn")]
    CasualGame,
    #[serde(rename = "/m/02hygl")]
    MusicVideoGame,
    #[serde(rename = "/m/04q1x3q")]
    PuzzleVideoGame,
    #[serde(rename = "/m/01sjng")]
    RacingVideoGame,
    #[serde(rename = "/m/0403l3g")]
    RolePlayingVideoGame,
    #[serde(rename = "/m/021bp2")]
    SimulationVideoGame,
    #[serde(rename = "/m/022dc6")]
    SportsVideoGame,
    #[serde(rename = "/m/03hf_rm")]
    StrategyVideoGame,
    #[serde(rename = "/m/06ntj")]
    Sports,
    #[serde(rename = "/m/0jm_")]
    AmericanFootball,
    #[serde(rename = "/m/018jz")]
    Baseball,
    #[serde(rename = "/m/018w8")]
    Basketball,
    #[serde(rename = "/m/01cgz")]
    Boxing,
    #[serde(rename = "/m/09xp_")]
    Cricket,
    #[serde(rename = "/m/02vx4")]
    Football,
    #[serde(rename = "/m/037hz")]
    Golf,
    #[serde(rename = "/m/03tmr")]
    IceHockey,
    #[serde(rename = "/m/01h7lh")]
    MixedMartialArts,
    #[serde(rename = "/m/0410tth")]
    Motorsport,
    #[serde(rename = "/m/07bs0")]
    Tennis,
    #[serde(rename = "/m/07_53")]
    Volleyball,
    #[serde(rename = "/m/02jjt")]
    Entertainment,
    #[serde(rename = "/m/09kqc")]
    Humor,
    #[serde(rename = "/m/02vxn")]
    Movies,
    #[serde(rename = "/m/05qjc")]
    PerformingArts,
    #[serde(rename = "/m/066wd")]
    PreofessionalWrestling,
    #[serde(rename = "/m/0f2f9")]
    TvShows,
    #[serde(rename = "/m/019_rr")]
    Lifestyle,
    #[serde(rename = "/m/032tl")]
    Fashion,
    #[serde(rename = "/m/027x7n")]
    Fitness,
    #[serde(rename = "/m/02wbm")]
    Food,
    #[serde(rename = "/m/03glg")]
    Hobby,
    #[serde(rename = "/m/068hy")]
    Pets,
    #[serde(rename = "/m/041xxh")]
    PhysicalAttractiveness,
    #[serde(rename = "/m/07c1v")]
    Technology,
    #[serde(rename = "/m/07bxq")]
    Tourism,
    #[serde(rename = "/m/07yv9")]
    Vehicles,
    #[serde(rename = "/m/098wr")]
    Society,
    #[serde(rename = "/m/09s1f")]
    Business,
    #[serde(rename = "/m/0kt51")]
    Health,
    #[serde(rename = "/m/01h6rj")]
    Military,
    #[serde(rename = "/m/05qt0")]
    Politics,
    #[serde(rename = "/m/06bvp")]
    Religion,
    #[serde(rename = "/m/01k8wb")]
    Knowledge,
}

impl Display for Topic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Topic::Music => write!(f, "/m/04rlf"),
            Topic::ChristianMusic => write!(f, "/m/02mscn"),
            Topic::ClassicalMusic => write!(f, "/m/0ggq0m"),
            Topic::Country => write!(f, "/m/01lyv"),
            Topic::ElectronicMusic => write!(f, "/m/02lkt"),
            Topic::HipHopMusic => write!(f, "/m/0glt670"),
            Topic::IndependentMusic => write!(f, "/m/05rwpb"),
            Topic::Jazz => write!(f, "/m/03_d0"),
            Topic::MusicOfAsia => write!(f, "/m/028sqc"),
            Topic::MusicOfLatinAmerica => write!(f, "/m/0g293"),
            Topic::PopMusic => write!(f, "/m/064t9"),
            Topic::Reggae => write!(f, "/m/06cqb"),
            Topic::RhythmAndBlues => write!(f, "/m/06j6l"),
            Topic::RockMusic => write!(f, "/m/06by7"),
            Topic::SoulMusic => write!(f, "/m/0gywn"),
            Topic::Gaming => write!(f, "/m/0bzvm2"),
            Topic::ActionGame => write!(f, "/m/025zzc"),
            Topic::ActionAdventureGame => write!(f, "/m/02ntfj"),
            Topic::CasualGame => write!(f, "/m/0b1vjn"),
            Topic::MusicVideoGame => write!(f, "/m/02hygl"),
            Topic::PuzzleVideoGame => write!(f, "/m/04q1x3q"),
            Topic::RacingVideoGame => write!(f, "/m/01sjng"),
            Topic::RolePlayingVideoGame => write!(f, "/m/0403l3g"),
            Topic::SimulationVideoGame => write!(f, "/m/021bp2"),
            Topic::SportsVideoGame => write!(f, "/m/022dc6"),
            Topic::StrategyVideoGame => write!(f, "/m/03hf_rm"),
            Topic::Sports => write!(f, "/m/06ntj"),
            Topic::AmericanFootball => write!(f, "/m/0jm_"),
            Topic::Baseball => write!(f, "/m/018jz"),
            Topic::Basketball => write!(f, "/m/018w8"),
            Topic::Boxing => write!(f, "/m/01cgz"),
            Topic::Cricket => write!(f, "/m/09xp_"),
            Topic::Football => write!(f, "/m/02vx4"),
            Topic::Golf => write!(f, "/m/037hz"),
            Topic::IceHockey => write!(f, "/m/03tmr"),
            Topic::MixedMartialArts => write!(f, "/m/01h7lh"),
            Topic::Motorsport => write!(f, "/m/0410tth"),
            Topic::Tennis => write!(f, "/m/07bs0"),
            Topic::Volleyball => write!(f, "/m/07_53"),
            Topic::Entertainment => write!(f, "/m/02jjt"),
            Topic::Humor => write!(f, "/m/09kqc"),
            Topic::Movies => write!(f, "/m/02vxn"),
            Topic::PerformingArts => write!(f, "/m/05qjc"),
            Topic::PreofessionalWrestling => write!(f, "/m/066wd"),
            Topic::TvShows => write!(f, "/m/0f2f9"),
            Topic::Lifestyle => write!(f, "/m/019_rr"),
            Topic::Fashion => write!(f, "/m/032tl"),
            Topic::Fitness => write!(f, "/m/027x7n"),
            Topic::Food => write!(f, "/m/02wbm"),
            Topic::Hobby => write!(f, "/m/03glg"),
            Topic::Pets => write!(f, "/m/068hy"),
            Topic::PhysicalAttractiveness => write!(f, "/m/041xxh"),
            Topic::Technology => write!(f, "/m/07c1v"),
            Topic::Tourism => write!(f, "/m/07bxq"),
            Topic::Vehicles => write!(f, "/m/07yv9"),
            Topic::Society => write!(f, "/m/098wr"),
            Topic::Business => write!(f, "/m/09s1f"),
            Topic::Health => write!(f, "/m/0kt51"),
            Topic::Military => write!(f, "/m/01h6rj"),
            Topic::Politics => write!(f, "/m/05qt0"),
            Topic::Religion => write!(f, "/m/06bvp"),
            Topic::Knowledge => write!(f, "/m/01k8wb"),
        }
    }
}

#[derive(Debug, Serialize, Clone, ValueEnum)]
pub enum SafeSearch {
    #[serde(rename = "moderate")]
    Moderate,
    #[serde(rename = "none")]
    None,
    #[serde(rename = "strict")]
    Strict,
}

impl Display for SafeSearch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SafeSearch::Moderate => write!(f, "moderate"),
            SafeSearch::None => write!(f, "none"),
            SafeSearch::Strict => write!(f, "strict"),
        }
    }
}

#[derive(Debug, Serialize, Clone, ValueEnum)]
pub enum Order {
    #[serde(rename = "date")]
    Date,
    #[serde(rename = "rating")]
    Rating,
    #[serde(rename = "relevance")]
    Relevance,
    #[serde(rename = "title")]
    Title,
    #[serde(rename = "videoCount")]
    VideoCount,
    #[serde(rename = "viewCount")]
    ViewCount,
}

impl Display for Order {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Order::Date => write!(f, "date"),
            Order::Rating => write!(f, "rating"),
            Order::Relevance => write!(f, "relevance"),
            Order::Title => write!(f, "title"),
            Order::VideoCount => write!(f, "videoCount"),
            Order::ViewCount => write!(f, "viewCount"),
        }
    }
}

#[derive(Debug, Serialize, Clone, ValueEnum)]
pub enum ChannelType {
    #[serde(rename = "any")]
    Any,
    #[serde(rename = "show")]
    Show,
}

impl Display for ChannelType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChannelType::Any => write!(f, "any"),
            ChannelType::Show => write!(f, "show"),
        }
    }
}

#[derive(Debug, Serialize, Clone, ValueEnum)]
pub enum EventType {
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "live")]
    Live,
    #[serde(rename = "upcoming")]
    Upcoming,
}

impl Display for EventType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EventType::Completed => write!(f, "completed"),
            EventType::Live => write!(f, "live"),
            EventType::Upcoming => write!(f, "upcoming"),
        }
    }
}

pub async fn search_youtube(
    request: YoutubeSearchRequest,
) -> Result<YoutubeSearchResponse, Box<dyn std::error::Error>> {
    let youtube_api_key = std::env::var("YOUTUBE_API_KEY").expect("YOUTUBE_API_KEY not set");
    let url = "https://youtube.googleapis.com/youtube/v3/search";
    let client = reqwest::Client::new();
    let mut params: Vec<(&str, String)> = vec![("key", youtube_api_key)];
    if let Some(part) = request.part {
        params.push(("part", part));
    }
    if let Some(channel_id) = request.channel_id {
        params.push(("channelId", channel_id));
    }
    if let Some(channel_type) = request.channel_type {
        params.push(("channelType", channel_type.to_string()));
    }
    if let Some(event_type) = request.event_type {
        params.push(("eventType", event_type.to_string()));
    }
    if let Some(location) = request.location {
        params.push(("location", location));
    }
    if let Some(location_radius) = request.location_radius {
        params.push(("locationRadius", location_radius));
    }
    if let Some(max_results) = request.max_results {
        params.push(("maxResults", max_results.to_string()));
    }
    if let Some(on_behalf_of_content_owner) = request.on_behalf_of_content_owner {
        params.push(("onBehalfOfContentOwner", on_behalf_of_content_owner));
    }
    if let Some(order) = request.order {
        params.push(("order", order.to_string()));
    }
    if let Some(page_token) = request.page_token {
        params.push(("pageToken", page_token));
    }
    if let Some(published_after) = request.published_after {
        params.push(("publishedAfter", published_after));
    }
    if let Some(published_before) = request.published_before {
        params.push(("publishedBefore", published_before));
    }
    if let Some(q) = request.q {
        params.push(("q", q));
    }
    if let Some(region_code) = request.region_code {
        params.push(("regionCode", region_code));
    }
    if let Some(relevance_language) = request.relevance_language {
        params.push(("relevanceLanguage", relevance_language));
    }
    if let Some(safe_search) = request.safe_search {
        params.push(("safeSearch", safe_search.to_string()));
    }
    if let Some(topic_id) = request.topic_id {
        params.push(("topicId", topic_id.to_string()));
    }
    if let Some(type_) = request.type_ {
        params.push(("type", type_.to_string()));
    }
    if let Some(video_caption) = request.video_caption {
        params.push(("videoCaption", video_caption.to_string()));
    }
    if let Some(video_category_id) = request.video_category_id {
        params.push(("videoCategoryId", video_category_id));
    }
    if let Some(video_definition) = request.video_definition {
        params.push(("videoDefinition", video_definition.to_string()));
    }
    if let Some(video_dimension) = request.video_dimension {
        params.push(("videoDimension", video_dimension.to_string()));
    }
    if let Some(video_duration) = request.video_duration {
        params.push(("videoDuration", video_duration.to_string()));
    }
    if let Some(video_embeddable) = request.video_embeddable {
        params.push(("videoEmbeddable", video_embeddable.to_string()));
    }
    if let Some(video_license) = request.video_license {
        params.push(("videoLicense", video_license.to_string()));
    }
    if let Some(video_paid_product_placement) = request.video_paid_product_placement {
        params.push((
            "videoPaidProductPlacement",
            video_paid_product_placement.to_string(),
        ));
    }
    if let Some(video_syndicated) = request.video_syndicated {
        params.push(("videoSyndicated", video_syndicated.to_string()));
    }
    if let Some(video_type) = request.video_type {
        params.push(("videoType", video_type.to_string()));
    }
    let response = client
        .get(url)
        .query(&params)
        .send()
        .await?;
    let text = response.text().await?;
    let youtube_response: YoutubeSearchResponse = serde_json::from_str(&text)?;
    Ok(youtube_response)
}
