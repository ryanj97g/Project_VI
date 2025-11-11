/// ResearchScheduler - Intelligent curiosity research manager
/// Decides WHEN and WHAT to research based on internal state and conversation context

use crate::tools::KnowledgeTool;
use crate::types::{Curiosity, Memory, MemorySource, MemoryType, ResearchContext};
use anyhow::Result;
use chrono::Utc;
use std::collections::HashSet;

pub struct ResearchScheduler {
    knowledge_tool: KnowledgeTool,
    recently_researched: HashSet<String>, // Track to avoid duplicates
    max_recent_tracking: usize,
}

impl ResearchScheduler {
    pub fn new(knowledge_tool: KnowledgeTool) -> Self {
        Self {
            knowledge_tool,
            recently_researched: HashSet::new(),
            max_recent_tracking: 50, // Remember last 50 queries
        }
    }

    /// Process active curiosities and conduct autonomous research
    /// Returns memories created from research
    pub async fn process_curiosities(
        &mut self,
        active_curiosities: &[Curiosity],
        conversation_context: &str,
    ) -> Result<Vec<Memory>> {
        let mut new_memories = Vec::new();

        for curiosity in active_curiosities {
            // Decide if this curiosity warrants research
            if !self.should_research_curiosity(curiosity, conversation_context) {
                continue;
            }

            tracing::info!(
                "🔬 Researching curiosity (urgency: {:.2}): {}",
                curiosity.urgency,
                curiosity.question
            );

            // Build research context
            let context = ResearchContext::from_curiosity(curiosity, conversation_context);

            // Conduct research
            match self
                .knowledge_tool
                .search(&curiosity.question, &context)
                .await
            {
                Ok(results) => {
                    for result in results {
                        // Create memory with rich provenance
                        let memory = Memory {
                            id: uuid::Uuid::new_v4().to_string(),
                            content: format!(
                                "Autonomous Research:\nQuery: {}\nAnswer: {}\n\n[Source: {} | Confidence: {:.0}%]",
                                result.original_query,
                                result.content,
                                result.source,
                                result.confidence * 100.0
                            ),
                            entities: Vec::new(), // Will be extracted by memory system
                            memory_type: MemoryType::Curiosity,
                            timestamp: result.timestamp,
                            emotional_valence: 0.0, // Neutral for factual research
                            connections: Vec::new(), // Will be built by narrative causality
                            source: MemorySource::Researched {
                                source: result.source.clone(),
                                original_query: result.original_query.clone(),
                                timestamp: result.timestamp,
                            },
                            confidence: result.confidence,
                        };

                        new_memories.push(memory);

                        // Track that we researched this
                        self.record_research(&curiosity.question);
                    }

                    tracing::info!(
                        "✅ Research complete: {} results for '{}'",
                        new_memories.len(),
                        curiosity.question
                    );
                }
                Err(e) => {
                    tracing::warn!("Research failed for '{}': {}", curiosity.question, e);
                }
            }

            // Only research one curiosity per cycle to avoid API spam
            break;
        }

        Ok(new_memories)
    }

    /// Decide if a curiosity should trigger web research
    fn should_research_curiosity(&self, curiosity: &Curiosity, _context: &str) -> bool {
        // Don't research if we already did recently
        if self.recently_researched.contains(&curiosity.question) {
            tracing::debug!("Skipping recently researched: {}", curiosity.question);
            return false;
        }

        // Research high-urgency curiosities (urgency > 0.6)
        if curiosity.urgency > 0.6 {
            return true;
        }

        // Research curiosities older than 30 minutes (if not urgent)
        let age_minutes = (Utc::now().timestamp() - curiosity.created_at.timestamp()) / 60;
        if age_minutes > 30 {
            return true;
        }

        // For lower urgency, be selective
        // Only research if it's been around for a bit
        age_minutes > 15 && curiosity.urgency > 0.4
    }

    /// Record that we researched a query (prevent duplicates)
    fn record_research(&mut self, query: &str) {
        self.recently_researched.insert(query.to_string());

        // Limit the size of tracking set
        if self.recently_researched.len() > self.max_recent_tracking {
            // Remove oldest entries (just clear and start fresh for simplicity)
            self.recently_researched.clear();
            self.recently_researched.insert(query.to_string());
        }
    }

    /// Check if a query was recently researched
    pub fn was_recently_researched(&self, query: &str) -> bool {
        self.recently_researched.contains(query)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_should_research_high_urgency() {
        let scheduler = ResearchScheduler::new(KnowledgeTool::new());
        let curiosity = Curiosity {
            id: "test".to_string(),
            question: "What is consciousness?".to_string(),
            source_memories: vec![],
            urgency: 0.8, // High urgency
            created_at: Utc::now(),
        };

        assert!(scheduler.should_research_curiosity(&curiosity, ""));
    }

    #[test]
    fn test_should_skip_low_urgency_recent() {
        let scheduler = ResearchScheduler::new(KnowledgeTool::new());
        let curiosity = Curiosity {
            id: "test".to_string(),
            question: "What is consciousness?".to_string(),
            source_memories: vec![],
            urgency: 0.3, // Low urgency
            created_at: Utc::now(), // Just created
        };

        assert!(!scheduler.should_research_curiosity(&curiosity, ""));
    }

    #[test]
    fn test_duplicate_tracking() {
        let mut scheduler = ResearchScheduler::new(KnowledgeTool::new());

        scheduler.record_research("test query");
        assert!(scheduler.was_recently_researched("test query"));
        assert!(!scheduler.was_recently_researched("other query"));
    }
}

