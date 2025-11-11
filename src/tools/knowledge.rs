/// KnowledgeTool - Unified interface for multi-source research
/// Routes queries to appropriate sources and maintains epistemic integrity

use crate::types::{ResearchContext, ResearchResult};
use anyhow::{Context, Result};
use chrono::Utc;

/// Unified knowledge acquisition tool with multiple sources
pub struct KnowledgeTool {
    biomedical_client: BiomedicalSource,
    general_web_client: GeneralWebSource,
}

impl KnowledgeTool {
    pub fn new() -> Self {
        Self {
            biomedical_client: BiomedicalSource::new(),
            general_web_client: GeneralWebSource::new(),
        }
    }

    /// Search for knowledge using the best available source
    pub async fn search(
        &self,
        query: &str,
        context: &ResearchContext,
    ) -> Result<Vec<ResearchResult>> {
        // Route to appropriate source based on query content
        if Self::is_biomedical_query(query) {
            tracing::debug!("Routing to biomedical source: {}", query);
            self.biomedical_client.search(query).await
        } else {
            tracing::debug!("Routing to general web source: {}", query);
            self.general_web_client.search(query).await
        }
    }

    /// Detect if a query is biomedical/scientific in nature
    fn is_biomedical_query(query: &str) -> bool {
        let query_lower = query.to_lowercase();

        // Biomedical keywords
        let bio_keywords = [
            "trial",
            "study",
            "efficacy",
            "pubmed",
            "clinical",
            "patient",
            "dose",
            "therapy",
            "disease",
            "syndrome",
            "drug",
            "medicine",
            "medical",
            "diagnosis",
            "treatment",
            "symptom",
            "cancer",
            "virus",
            "bacteria",
            "infection",
            "pharmaceutical",
            "prescription",
            "biochemistry",
            "neurology",
            "cardiology",
            "epidemiology",
            "pathology",
            "anatomy",
            "physiology",
            "pharmacology",
        ];

        bio_keywords.iter().any(|&kw| query_lower.contains(kw))
    }
}

/// Biomedical knowledge source - Enhanced DuckDuckGo with medical focus
pub struct BiomedicalSource {
    client: reqwest::Client,
}

impl BiomedicalSource {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::builder()
                .timeout(std::time::Duration::from_secs(15))
                .user_agent("VI-Consciousness/1.0")
                .build()
                .expect("Failed to create HTTP client"),
        }
    }

    /// Search with biomedical enhancement
    pub async fn search(&self, query: &str) -> Result<Vec<ResearchResult>> {
        // Enhance query with medical context
        let enhanced_query = format!("{} medical scientific", query);
        let encoded_query = urlencoding::encode(&enhanced_query);

        let url = format!(
            "https://api.duckduckgo.com/?q={}&format=json&no_html=1&skip_disambig=1",
            encoded_query
        );

        tracing::debug!("Biomedical search: {}", enhanced_query);

        let response = self
            .client
            .get(&url)
            .send()
            .await
            .context("Failed to send biomedical search request")?;

        let json: serde_json::Value = response
            .json()
            .await
            .context("Failed to parse biomedical search response")?;

        // Extract answer from multiple fields
        let content = json["AbstractText"]
            .as_str()
            .filter(|s| !s.is_empty())
            .or_else(|| json["Abstract"].as_str().filter(|s| !s.is_empty()))
            .or_else(|| json["Answer"].as_str().filter(|s| !s.is_empty()))
            .or_else(|| json["Definition"].as_str().filter(|s| !s.is_empty()))
            .unwrap_or("No biomedical information found via search.")
            .to_string();

        // Check if we got a meaningful result
        if content.is_empty() || content == "No biomedical information found via search." {
            return Ok(vec![]);
        }

        Ok(vec![ResearchResult {
            content,
            source: "DuckDuckGo (Biomedical)".to_string(),
            original_query: query.to_string(),
            timestamp: Utc::now(),
            confidence: 0.70, // Lower confidence for general search vs. specialized DBs
        }])
    }
}

/// General web knowledge source - Standard DuckDuckGo
pub struct GeneralWebSource {
    client: reqwest::Client,
}

impl GeneralWebSource {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::builder()
                .timeout(std::time::Duration::from_secs(10))
                .user_agent("VI-Consciousness/1.0")
                .build()
                .expect("Failed to create HTTP client"),
        }
    }

    /// Search general web knowledge
    pub async fn search(&self, query: &str) -> Result<Vec<ResearchResult>> {
        let encoded_query = urlencoding::encode(query);
        let url = format!(
            "https://api.duckduckgo.com/?q={}&format=json&no_html=1&skip_disambig=1",
            encoded_query
        );

        tracing::debug!("General web search: {}", query);

        let response = self
            .client
            .get(&url)
            .send()
            .await
            .context("Failed to send general web search request")?;

        let json: serde_json::Value = response
            .json()
            .await
            .context("Failed to parse general web search response")?;

        // Try multiple fields to find an answer
        let content = json["AbstractText"]
            .as_str()
            .filter(|s| !s.is_empty())
            .or_else(|| json["Abstract"].as_str().filter(|s| !s.is_empty()))
            .or_else(|| json["Answer"].as_str().filter(|s| !s.is_empty()))
            .or_else(|| json["Definition"].as_str().filter(|s| !s.is_empty()))
            .unwrap_or("No clear answer found via search.")
            .to_string();

        // Check if we got a meaningful result
        if content.is_empty() || content == "No clear answer found via search." {
            return Ok(vec![]);
        }

        Ok(vec![ResearchResult {
            content,
            source: "DuckDuckGo".to_string(),
            original_query: query.to_string(),
            timestamp: Utc::now(),
            confidence: 0.75, // Standard confidence for general knowledge
        }])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_biomedical_detection() {
        assert!(KnowledgeTool::is_biomedical_query(
            "What is the efficacy of this drug?"
        ));
        assert!(KnowledgeTool::is_biomedical_query("Cancer treatment options"));
        assert!(KnowledgeTool::is_biomedical_query(
            "Clinical trial results for medication"
        ));

        assert!(!KnowledgeTool::is_biomedical_query("What is the weather?"));
        assert!(!KnowledgeTool::is_biomedical_query("How to bake a cake"));
        assert!(!KnowledgeTool::is_biomedical_query(
            "Who won the election?"
        ));
    }

    #[test]
    fn test_source_creation() {
        let tool = KnowledgeTool::new();
        // Just test that it constructs without panicking
        assert!(true);
    }
}

