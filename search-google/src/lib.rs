use clap::Parser;
use serde::{Deserialize, Serialize};
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SearchResponse {
    pub kind: String,
    pub url: Url,
    pub queries: Queries,
    pub context: Context,
    pub search_information: Option<SearchInformation>,
    pub items: Vec<SearchItem>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Url {
    #[serde(rename = "type")]
    pub type_: String,
    pub template: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Queries {
    pub request: Vec<Request>,
    #[serde(rename = "nextPage")]
    pub next_page: Vec<NextPage>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct NextPage {
    pub title: String,
    #[serde(rename = "totalResults")]
    pub total_results: String,
    #[serde(rename = "searchTerms")]
    pub search_terms: String,
    pub count: i64,
    #[serde(rename = "startIndex")]
    pub start_index: i64,
    #[serde(rename = "inputEncoding")]
    pub input_encoding: String,
    #[serde(rename = "outputEncoding")]
    pub output_encoding: String,
    pub safe: String,
    pub cx: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Context {
    pub title: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SearchInformation {
    #[serde(rename = "searchTime")]
    pub search_time: f64,
    #[serde(rename = "formattedSearchTime")]
    pub formatted_search_time: String,
    #[serde(rename = "totalResults")]
    pub total_results: String,
    #[serde(rename = "formattedTotalResults")]
    pub formatted_total_results: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SearchItem {
    pub kind: String,
    pub title: String,
    pub link: String,
    pub snippet: String,
    #[serde(rename = "cacheId")]
    pub cache_id: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Request {
    pub title: String,
    #[serde(rename = "totalResults")]
    pub total_results: String,
    #[serde(rename = "searchTerms")]
    pub search_terms: String,
    pub count: i64,
    #[serde(rename = "startIndex")]
    pub start_index: i64,
    #[serde(rename = "inputEncoding")]
    pub input_encoding: String,
    #[serde(rename = "outputEncoding")]
    pub output_encoding: String,
    pub safe: String,
    pub cx: String,
}

#[derive(Debug, Parser, Serialize, Clone)]
pub struct SearchRequest {
    #[clap(short, long, help = "Search Query")]
    pub q: Option<String>,
    #[clap(
        short,
        long,
        help = "Enables (0 default) or disables (1) Simplified and Traditional Chinese Search."
    )]
    pub c2coff: Option<String>,
    #[clap(short, long, help = "Country Restriction")]
    pub cr: Option<String>, // Country Restriction
    #[clap(short, long, help = "Custom Search Engine ID")]
    pub cx: Option<String>, // Custom Search Engine ID
    #[clap(short, long, help = "Date Restrict")]
    pub date_restrict: Option<String>, // Date Restrict
    #[clap(short, long, help = "Exact Terms")]
    pub exact_terms: Option<String>, // Exact Terms
    #[clap(short, long, help = "Exclude Terms")]
    pub exclude_terms: Option<String>, // Exclude Terms
    #[clap(short, long, help = "File Type")]
    pub file_type: Option<String>, // File Type
    #[clap(short, long, help = "Filter for duplicate content 0 or 1, default 0")]
    pub filter: Option<String>, // Filter for duplicate content 0 or 1, default 0
    #[clap(short, long, help = "Geolocation of end user")]
    pub gl: Option<String>, // Geolocation of end user
    #[clap(short, long, help = "Highest post in range")]
    pub high_range: Option<String>, // Highest post in range
    #[clap(short, long, help = "Interface Language")]
    pub hl: Option<String>, // Interface Language
    #[clap(
        short,
        long,
        help = "Appends the specified query terms to the query, as if they were combined with a logical AND operator."
    )]
    pub hq: Option<String>,
    #[clap(short, long, help = "Image Color Type")]
    pub img_color_type: Option<String>, // Image Color Type
    #[clap(short, long, help = "Image Dominant Color")]
    pub img_dominant_color: Option<String>, // Image Dominant Color
    #[clap(short, long, help = "Image Size")]
    pub img_size: Option<String>, // Image Size
    #[clap(short, long, help = "Image Type")]
    pub img_type: Option<String>, // Image Type: clipart, face, lineart, stock, photo, animated
    #[clap(
        short,
        long,
        help = "Specifies that all search results should contain a link to a particular URL"
    )]
    pub link_site: Option<String>, // Specifies that all search results should contain a link to a particular URL
    #[clap(short, long, help = "Lowest post in range")]
    pub low_range: Option<String>, // Specifies the starting value for a search range. Use lowRange and highRange to append an inclusive search range of lowRange...highRange to the query.
    #[clap(short, long, help = "Language Restrict")]
    pub lr: Option<String>, // Language Restrict
    #[clap(short, long, help = "Number of search results to return 1-10")]
    pub num: Option<String>, // Number of search results to return 1-10
    #[clap(
        short,
        long,
        help = "Provides additional search terms to check for in a document, where each document in the search results must contain at least one of the additional search terms."
    )]
    pub or_terms: Option<String>,
    #[clap(
        short,
        long,
        help = "Specifies that all search results should be pages that are related to the specified URL"
    )]
    pub related_site: Option<String>,
    #[clap(
        short,
        long,
        help = "Filters based on licensing. Supported values include: cc_publicdomain, cc_attribute, cc_sharealike, cc_noncommercial, cc_nonderived"
    )]
    pub rights: Option<String>,
    #[clap(short, long, help = "Search Safety Level")]
    pub safe: Option<String>,
    #[clap(short, long, help = "Search Type (image, news, video)")]
    pub search_type: Option<String>,
    #[clap(
        short,
        long,
        help = "Specifies all search results should be pages from a given site"
    )]
    pub site_search: Option<String>,
    #[clap(
        short,
        long,
        help = "Controls whether to include or exclude results from the site named in the sitesearch parameter"
    )]
    pub site_search_filter: Option<String>,
    #[clap(short, long, help = "Sorts results")]
    pub sort: Option<String>,
    #[clap(
        short,
        long,
        help = "The index of the first result to return. The default number of results per page is 10, so &start=11 would start at the top of the second page of results. Note: The JSON API will never return more than 100 results, even if more than 100 documents match the query, so setting the sum of start + num to a number greater than 100 will produce an error. Also note that the maximum value for num is 10."
    )]
    pub start: Option<String>,
}

pub async fn search(
    // search_query: &str,
    search_request: SearchRequest,
) -> Result<SearchResponse, Box<dyn std::error::Error>> {
    let url = "https://customsearch.googleapis.com/customsearch/v1";
    let api_key = std::env::var("GOOGLE_SEARCH_API_KEY")
        .expect("GOOGLE_SEARCH_API_KEY environment variable not set");
    let engine_id = std::env::var("GOOGLE_SEARCH_ENGINE_ID")
        .expect("GOOGLE_SEARCH_ENGINE_ID environment variable not set"); // a7f51c925c0474034

    let mut query_params = vec![
        ("key", api_key),
        ("cx", engine_id),
        ("q", search_request.q.unwrap_or("".to_string())),
    ];
    if let Some(c2coff) = search_request.c2coff {
        query_params.push(("c2coff", c2coff));
    }
    if let Some(cr) = search_request.cr {
        query_params.push(("cr", cr));
    }
    if let Some(cx) = search_request.cx {
        query_params.push(("cx", cx));
    }
    if let Some(date_restrict) = search_request.date_restrict {
        query_params.push(("dateRestrict", date_restrict));
    }
    if let Some(exact_terms) = search_request.exact_terms {
        query_params.push(("exactTerms", exact_terms));
    }
    if let Some(exclude_terms) = search_request.exclude_terms {
        query_params.push(("excludeTerms", exclude_terms));
    }
    if let Some(file_type) = search_request.file_type {
        query_params.push(("fileType", file_type));
    }
    if let Some(filter) = search_request.filter {
        query_params.push(("filter", filter));
    }
    if let Some(gl) = search_request.gl {
        query_params.push(("gl", gl));
    }
    if let Some(high_range) = search_request.high_range {
        query_params.push(("highRange", high_range));
    }
    if let Some(hl) = search_request.hl {
        query_params.push(("hl", hl));
    }
    if let Some(hq) = search_request.hq {
        query_params.push(("hq", hq));
    }
    if let Some(img_color_type) = search_request.img_color_type {
        query_params.push(("imgColorType", img_color_type));
    }
    if let Some(img_dominant_color) = search_request.img_dominant_color {
        query_params.push(("imgDominantColor", img_dominant_color));
    }
    if let Some(img_size) = search_request.img_size {
        query_params.push(("imgSize", img_size));
    }
    if let Some(img_type) = search_request.img_type {
        query_params.push(("imgType", img_type));
    }
    if let Some(link_site) = search_request.link_site {
        query_params.push(("linkSite", link_site));
    }
    if let Some(low_range) = search_request.low_range {
        query_params.push(("lowRange", low_range));
    }
    if let Some(lr) = search_request.lr {
        query_params.push(("lr", lr));
    }
    if let Some(num) = search_request.num {
        query_params.push(("num", num));
    }
    if let Some(or_terms) = search_request.or_terms {
        query_params.push(("orTerms", or_terms));
    }
    if let Some(related_site) = search_request.related_site {
        query_params.push(("relatedSite", related_site));
    }
    if let Some(rights) = search_request.rights {
        query_params.push(("rights", rights));
    }
    if let Some(safe) = search_request.safe {
        query_params.push(("safe", safe));
    }
    if let Some(search_type) = search_request.search_type {
        query_params.push(("searchType", search_type));
    }
    if let Some(site_search) = search_request.site_search {
        query_params.push(("siteSearch", site_search));
    }
    if let Some(site_search_filter) = search_request.site_search_filter {
        query_params.push(("siteSearchFilter", site_search_filter));
    }
    if let Some(sort) = search_request.sort {
        query_params.push(("sort", sort));
    }
    if let Some(start) = search_request.start {
        query_params.push(("start", start));
    }

    let search_response: SearchResponse = reqwest::Client::new()
        .get(url)
        .query(&query_params)
        .send()
        .await?
        .json()
        .await?;
    Ok(search_response)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_search_with_params() {
        println!("Running test_search_with_params");
        let search_request = SearchRequest {
            q: Some("reliance industries".to_string()),
            c2coff: Some("1".to_string()),
            cr: Some("countryIN".to_string()),
            cx: None,
            date_restrict: Some("d1".to_string()), // d1: Past 24 hours, w1: Past week, m1: Past month, y1: Past year
            exact_terms: None,
            exclude_terms: None,
            file_type: None,
            filter: Some("1".to_string()),
            gl: None,
            high_range: None,
            hl: None,
            hq: None,
            img_color_type: None,
            img_dominant_color: None,
            img_size: None,
            img_type: None,
            link_site: None,
            low_range: None,
            lr: Some("lang_en".to_string()),
            num: None,
            or_terms: None,
            related_site: None,
            rights: None,
            safe: None,
            search_type: None,
            site_search: None,
            site_search_filter: None,
            sort: None,
            start: None,
        };
        let search_response = search(search_request).await.unwrap();
        println!(
            "total_results: {}",
            search_response.queries.request[0].total_results
        );
        println!("search_response.kind: {}", search_response.kind);
        println!(
            "search_response.items.len(): {}",
            search_response.items.len()
        );
        for item in &search_response.items {
            println!("item.title: {}", item.title);
            println!("item.link: {}", item.link);
            println!("item.snippet: {}", item.snippet);
            println!("=========================================================");
        }
        assert_eq!(search_response.kind, "customsearch#search");
        assert_eq!(search_response.items.len(), 10);
    }
}
