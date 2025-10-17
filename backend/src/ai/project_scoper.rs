use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;

// --- Structs to match Google's API JSON structure ---

#[derive(Serialize)]
struct GeminiRequest {
    contents: Vec<Content>,
}

#[derive(Serialize)]
struct Content {
    parts: Vec<Part>,
}

#[derive(Serialize)]
struct Part {
    text: String,
}

#[derive(Deserialize)]
struct GeminiResponse {
    candidates: Vec<Candidate>,
}

#[derive(Deserialize)]
struct Candidate {
    content: ContentResponse,
}

#[derive(Deserialize)]
struct ContentResponse {
    parts: Vec<PartResponse>,
}

#[derive(Deserialize)]
struct PartResponse {
    text: String,
}

// --- The main function ---

pub async fn scope_project(
    project_description: String,
) -> Result<String, Box<dyn std::error::Error>> {
    let api_key = env::var("GOOGLE_AI_API_KEY").expect("GOOGLE_AI_API_KEY must be set");
    let url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.5-pro:generateContent?key={}",
        api_key
    );

    let prompt = format!(
        "You are an expert project manager. A user has provided the following project description: '{}'.
        Based on this description, generate a list of high-level tasks required to complete the project.
        Return the tasks as a JSON array of strings. For example: [\"Task 1\", \"Task 2\", \"Task 3\"].
        IMPORTANT: Only output the raw JSON array. Do not include any other text, explanations, or Markdown code blocks like ```json.",
        project_description
    );

    // Build the request body using our structs
    let request_body = GeminiRequest {
        contents: vec![Content {
            parts: vec![Part { text: prompt }],
        }],
    };

    // Create a client and send the request
    let client = Client::new();
    let response = client.post(&url).json(&request_body).send().await?;

    // Check if the request was successful
    if !response.status().is_success() {
        return Err(format!("API Error: {}", response.status()).into());
    }

    // Deserialize the JSON response into our response structs
    let gemini_response = response.json::<GeminiResponse>().await?;

    // Extract the text from the response
    if let Some(candidate) = gemini_response.candidates.get(0) {
        if let Some(part) = candidate.content.parts.get(0) {
            return Ok(part.text.clone());
        }
    }

    Err("No text found in AI response".into())
}
