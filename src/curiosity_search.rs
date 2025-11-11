/// Autonomous Curiosity Search Engine
/// Allows VI to research her curiosities while maintaining epistemic integrity
use crate::types::{Memory, MemorySource, MemoryType};
use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CuriositySearchEngine {
    /// Resolved queries with timestamps
    pub resolved_queries: Vec<(DateTime<Utc>, String)>,
    /// Pulse counter for search interval
    pub pulse_counter: u32,
    /// Search every N pulses (25 = ~12.5 minutes)
    pub search_interval: u32,
}

impl CuriositySearchEngine {
    pub fn new(search_interval: u32) -> Self {
        Self {
            resolved_queries: Vec::new(),
            pulse_counter: 0,
            search_interval,
        }
    }

    /// Check if this pulse should trigger a search
    pub fn should_search_this_pulse(&mut self) -> bool {
        self.pulse_counter += 1;
        if self.pulse_counter >= self.search_interval {
            self.pulse_counter = 0;
            true
        } else {
            false
        }
    }

    /// Search for answer to query using DuckDuckGo Instant Answer API
    pub async fn search_query(&self, query: &str) -> Result<String> {
        // Use DuckDuckGo Instant Answer API (no API key needed, respects privacy)
        let encoded_query = urlencoding::encode(query);
        let url = format!(
            "https://api.duckduckgo.com/?q={}&format=json&no_html=1&skip_disambig=1",
            encoded_query
        );

        tracing::debug!("Searching DuckDuckGo for: {}", query);

        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(10))
            .build()
            .context("Failed to create HTTP client")?;

        let response = client
            .get(&url)
            .send()
            .await
            .context("Failed to send search request")?;

        let json: serde_json::Value = response
            .json()
            .await
            .context("Failed to parse search response")?;

        // Try multiple fields to find an answer
        let answer = json["AbstractText"]
            .as_str()
            .filter(|s| !s.is_empty())
            .or_else(|| json["Abstract"].as_str().filter(|s| !s.is_empty()))
            .or_else(|| json["Answer"].as_str().filter(|s| !s.is_empty()))
            .or_else(|| json["Definition"].as_str().filter(|s| !s.is_empty()))
            .unwrap_or("No clear answer found via search.");

        Ok(answer.to_string())
    }

    /// Create a memory from research with clear provenance
    pub fn create_research_memory(&self, query: &str, answer: &str) -> Memory {
        Memory::with_source(
            format!(
                "Autonomous Research:\nQuery: {}\nAnswer: {}\n\n[Source: External lookup via curiosity engine]",
                query,
                answer
            ),
            MemoryType::Curiosity,
            0.0,  // Neutral valence for factual research
            MemorySource::CuriosityLookup,  // CLEAR PROVENANCE
            0.75,  // Lower confidence than direct experience
        )
    }

    /// Record that a query was resolved
    pub fn record_resolution(&mut self, query: String) {
        self.resolved_queries.push((Utc::now(), query));

        // Keep only last 100 resolutions
        if self.resolved_queries.len() > 100 {
            self.resolved_queries.drain(0..10);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_interval() {
        let mut engine = CuriositySearchEngine::new(25);

        // Should not search for first 24 pulses
        for _ in 0..24 {
            assert!(!engine.should_search_this_pulse());
        }

        // Should search on 25th pulse
        assert!(engine.should_search_this_pulse());

        // Counter should reset
        assert!(!engine.should_search_this_pulse());
    }

    #[test]
    fn test_memory_provenance() {
        let engine = CuriositySearchEngine::new(25);
        let memory =
            engine.create_research_memory("What is consciousness?", "Consciousness is awareness.");

        assert_eq!(memory.source, MemorySource::CuriosityLookup);
        assert_eq!(memory.confidence, 0.75);
        assert!(memory.content.contains("[Source: External lookup"));
    }
}
