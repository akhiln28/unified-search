# Unified search

This application allows you to query various platforms such as google, youtube, bing, and also chat with various LLMs such as GPT-3, llama, and others.

All your queries and responses are saved in an sqlite database.

If you have already searched for a query, the application will first look in the database if you have made any similar queries and will return the results from the database.

## Usage

```bash
us --help

# Search all services
us all "What is the capital of France?"

# Search only google
us google "What is the capital of France?"

# Search only youtube
us youtube "What is the capital of France?"

# Chat with GPT-3
us gpt3 "What is the capital of France?"

# Chat with llama
us llama "What is the capital of France?"
```

```bash
export UNIFIED_SEARCH_DB_PATH="path-to-your-database.db"
export OPENAI_API_KEY="your-openai-api-key"
export GOOGLE_GENERATE_API_KEY="your-google-generate-api-key"
export GOOGLE_SEARCH_ENGINE_ID="your-google-search-engine-id"
export GOOGLE_SEARCH_API_KEY="your-google-search-api-key"
export YOUTUBE_API_KEY="your-youtube-api-key"

# This is an unofficial together api. <https://docs.together.ai/docs/inference-rest>
ENDPOINT_URL="https://api.together.xyz/inference"
TOGETHER_API_KEY="YOUR_API_KEY"
```

```bash
curl -X POST "$ENDPOINT_URL" \
     -H "Authorization: Bearer $TOGETHER_API_KEY" \
     -H "Content-Type: application/json" \
     -d '{"model": "togethercomputer/RedPajama-INCITE-7B-Instruct", "prompt": "Q: The capital of France is?\nA:", "temperature": 0.8, "top_p": 0.7, "top_k": 50, "max_tokens": 1, "repetition_penalty": 1}'

curl 'https://api.together.xyz/inference' -X POST \
  -H 'Authorization: Bearer API_KEY' \
  -H 'Content-Type: application/json' \
  -d @- <<'EOF'
{
  "model": "togethercomputer/RedPajama-INCITE-7B-Instruct",
  "prompt": "Alan Turing was",
  "max_tokens": 128,
  "stop":["\n\n"],
  "temperature":0.7,
  "top_p":0.7,
  "top_k":50,
  "repetition_penalty": 1,
  "stream_tokens": true
}
```
