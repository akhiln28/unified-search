# Youtube Search

<https://developers.google.com/youtube/v3/docs/search/list>

**Warning: initial working out; it will change very frequently.**

## Usage

Set the environment variable `YOUTUBE_API_KEY` to a gcp api key with the youtube data api enabled. You can create a new api key at <https://console.cloud.google.com/apis/credentials>.

```bash
export YOUTUBE_API_KEY="YOUR_API_KEY"
```

```bash
cargo run --release -- -q "rust programming"
```

## Description

This is a cli tool and library to search Youtube using the official Youtube API.

The q parameter specifies the query term to search for.
Your request can also use the Boolean NOT (-) and OR (|) operators to exclude videos or to find videos that are associated with one of several search terms. For example, to search for videos matching either "boating" or "sailing", set the q parameter value to boating|sailing. Similarly, to search for videos matching either "boating" or "sailing" but not "fishing", set the q parameter value to boating|sailing -fishing. Note that the pipe character must be URL-escaped when it is sent in your API request. The URL-escaped value for the pipe character is %7C.
