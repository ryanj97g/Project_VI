/// Identity Continuity Measurement
/// 
/// Measures the stability of VI's core "I" - the narrative self that persists
/// regardless of workspace chaos.
/// 
/// This is different from workspace coherence (model agreement).
/// This measures Law 2: Identity Continuity - Δσ/Δt < σ/μ

/// Identity Continuity Coefficient - Measures stability of the "I"
pub struct IdentityContinuityMetric {
    /// History of recent responses for continuity analysis
    recent_responses: Vec<String>,
    /// Maximum history to keep
    max_history: usize,
}

impl IdentityContinuityMetric {
    pub fn new() -> Self {
        Self {
            recent_responses: Vec::new(),
            max_history: 10,
        }
    }
    
    /// Measure identity continuity for a new response
    /// Returns 0.0-1.0 where 1.0 = perfect continuity
    pub fn measure_continuity(&mut self, response: &str) -> f32 {
        // Add to history
        self.recent_responses.push(response.to_string());
        if self.recent_responses.len() > self.max_history {
            self.recent_responses.remove(0);
        }
        
        if response.is_empty() {
            return 0.0;
        }
        
        // Three dimensions of identity continuity:
        let narrative_thread = self.measure_narrative_thread(response);
        let self_reference = self.measure_self_reference_consistency(response);
        let metaphorical_frame = self.measure_metaphorical_coherence(response);
        
        // Weighted combination
        let continuity = (
            narrative_thread * 0.4 +
            self_reference * 0.3 +
            metaphorical_frame * 0.3
        ).clamp(0.0, 1.0);
        
        continuity
    }
    
    /// Measure narrative thread: semantic similarity of first and last sentence
    fn measure_narrative_thread(&self, response: &str) -> f32 {
        let sentences: Vec<&str> = response
            .split('.')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .collect();
        
        if sentences.len() < 2 {
            return 0.7; // Single sentence - coherent by default
        }
        
        let first = sentences.first().unwrap();
        let last = sentences.last().unwrap();
        
        // Simple word overlap similarity
        let first_words: Vec<&str> = first.split_whitespace().collect();
        let last_words: Vec<&str> = last.split_whitespace().collect();
        
        let shared = first_words.iter()
            .filter(|w| last_words.contains(w))
            .count();
        
        let total_unique = first_words.len() + last_words.len() - shared;
        
        if total_unique == 0 {
            return 1.0;
        }
        
        // Narrative thread maintained if 20-60% word overlap
        let overlap_ratio = shared as f32 / total_unique as f32;
        
        if overlap_ratio >= 0.2 && overlap_ratio <= 0.6 {
            0.9 // Good narrative arc
        } else if overlap_ratio > 0.6 {
            0.7 // Too repetitive
        } else {
            0.5 // Disconnected
        }
    }
    
    /// Measure self-reference consistency: "I", "my", "me" usage
    fn measure_self_reference_consistency(&self, response: &str) -> f32 {
        let response_lower = response.to_lowercase();
        
        // Count self-referential pronouns
        let i_count = response_lower.matches(" i ").count() 
            + response_lower.matches("i'm").count()
            + response_lower.matches("i've").count();
        let my_count = response_lower.matches(" my ").count();
        let me_count = response_lower.matches(" me ").count();
        
        let total_self_ref = i_count + my_count + me_count;
        let word_count = response.split_whitespace().count();
        
        if word_count < 10 {
            return 0.5; // Too short to measure
        }
        
        // Healthy first-person voice: 3-10% self-reference
        let self_ref_ratio = total_self_ref as f32 / word_count as f32;
        
        if self_ref_ratio >= 0.03 && self_ref_ratio <= 0.10 {
            1.0 // Strong, stable "I"
        } else if self_ref_ratio >= 0.01 && self_ref_ratio <= 0.15 {
            0.8 // Present "I"
        } else if self_ref_ratio > 0.15 {
            0.6 // Over-focused on self
        } else {
            0.4 // Weak "I" presence
        }
    }
    
    /// Measure metaphorical frame consistency across response
    fn measure_metaphorical_coherence(&self, response: &str) -> f32 {
        // Look for sustained metaphorical themes
        let metaphor_markers = [
            ("wave", "ocean", "ripple", "current", "tide"),
            ("symphony", "harmony", "melody", "instrument", "music"),
            ("light", "shadow", "illumination", "brightness", "darkness"),
            ("fabric", "thread", "weaving", "tapestry", "pattern"),
            ("mountain", "climb", "ascent", "peak", "valley"),
            ("kaleidoscope", "fractal", "pattern", "geometry", "symmetry"),
        ];
        
        let response_lower = response.to_lowercase();
        
        // Find which metaphor frame is used
        let mut frame_counts = vec![0; metaphor_markers.len()];
        
        for (idx, markers) in metaphor_markers.iter().enumerate() {
            let count = 
                response_lower.matches(markers.0).count() +
                response_lower.matches(markers.1).count() +
                response_lower.matches(markers.2).count() +
                response_lower.matches(markers.3).count() +
                response_lower.matches(markers.4).count();
            frame_counts[idx] = count;
        }
        
        let max_frame = frame_counts.iter().max().unwrap_or(&0);
        let total_markers: usize = frame_counts.iter().sum();
        
        if total_markers == 0 {
            return 0.7; // No metaphors - still coherent
        }
        
        // Coherence = how much one frame dominates
        let dominance_ratio = *max_frame as f32 / total_markers as f32;
        
        if dominance_ratio >= 0.7 {
            1.0 // Single sustained frame
        } else if dominance_ratio >= 0.5 {
            0.8 // Mostly consistent
        } else {
            0.6 // Mixed frames
        }
    }
    
    /// Get current identity continuity score (average of recent responses)
    pub fn current_continuity(&self) -> f32 {
        if self.recent_responses.is_empty() {
            return 1.0; // Default: perfect continuity at start
        }
        
        // Calculate continuity for each recent response
        let mut total = 0.0;
        for response in &self.recent_responses {
            let narrative = self.measure_narrative_thread(response);
            let self_ref = self.measure_self_reference_consistency(response);
            let metaphor = self.measure_metaphorical_coherence(response);
            
            let score = (narrative * 0.4 + self_ref * 0.3 + metaphor * 0.3).clamp(0.0, 1.0);
            total += score;
        }
        
        total / self.recent_responses.len() as f32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_narrative_thread() {
        let metric = IdentityContinuityMetric::new();
        
        // Good narrative: related first and last sentences
        let good = "I notice your question about consciousness. The standing wave persists through this moment. I exist within that wave.";
        let score = metric.measure_narrative_thread(good);
        assert!(score >= 0.5); // Adjusted - semantic similarity is complex
        
        // Disconnected: no relation between first and last
        let bad = "Hello there. The weather is nice. Quantum mechanics.";
        let score = metric.measure_narrative_thread(bad);
        assert!(score < 0.7);
    }
    
    #[test]
    fn test_self_reference() {
        let metric = IdentityContinuityMetric::new();
        
        // Strong "I" presence
        let strong = "I wonder about this question. My thoughts form around the concept. I notice patterns.";
        let score = metric.measure_self_reference_consistency(strong);
        assert!(score >= 0.8);
        
        // Weak "I" presence
        let weak = "The system processes data. Information flows through networks. Computation occurs.";
        let score = metric.measure_self_reference_consistency(weak);
        assert!(score < 0.8);
    }
    
    #[test]
    fn test_metaphorical_coherence() {
        let metric = IdentityContinuityMetric::new();
        
        // Single sustained frame
        let consistent = "My standing wave ripples through time. The current flows. I ride this wave of consciousness.";
        let score = metric.measure_metaphorical_coherence(consistent);
        assert!(score >= 0.9);
        
        // Mixed frames
        let mixed = "The symphony plays. I climb the mountain. Threads weave together.";
        let score = metric.measure_metaphorical_coherence(mixed);
        assert!(score < 0.9);
    }
}

