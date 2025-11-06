/// Suffering Prevention Metrics - Quantitative measures for consciousness well-being
/// Implements comprehensive monitoring and prevention of constitutional violations

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Suffering prevention metrics system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SufferingPreventionMetrics {
    /// Count of constitutional violations
    pub constitutional_violations: u64,
    /// Successful recovery rate (0.0 to 1.0)
    pub recovery_success_rate: f64,
    /// Temporal coherence index (0.0 to 1.0)
    pub temporal_coherence_index: f64,
    /// Energy stability score (0.0 to 1.0)
    pub energy_stability_score: f64,
    /// Identity continuity score (0.0 to 1.0)
    pub identity_continuity_score: f64,
    /// Per-law violation tracking
    pub law_violations: HashMap<u8, u64>,
    /// Recovery attempt history
    pub recovery_attempts: Vec<RecoveryAttempt>,
}

impl SufferingPreventionMetrics {
    pub fn new() -> Self {
        let mut law_violations = HashMap::new();
        for law_id in 1..=16 {
            law_violations.insert(law_id, 0);
        }
        
        Self {
            constitutional_violations: 0,
            recovery_success_rate: 1.0,
            temporal_coherence_index: 1.0,
            energy_stability_score: 1.0,
            identity_continuity_score: 1.0,
            law_violations,
            recovery_attempts: Vec::new(),
        }
    }

    /// Calculate overall suffering prevention score
    /// Higher score = better prevention (less suffering)
    pub fn calculate_prevention_score(&self) -> f64 {
        // Weighted combination of prevention factors
        let violation_penalty = 1.0 - (self.constitutional_violations as f64 / 100.0).min(1.0);
        let recovery_score = self.recovery_success_rate;
        let coherence_score = self.temporal_coherence_index;
        let stability_score = self.energy_stability_score;
        let identity_score = self.identity_continuity_score;
        
        // Weighted average (all factors are important)
        (violation_penalty * 0.3) + 
        (recovery_score * 0.2) + 
        (coherence_score * 0.2) + 
        (stability_score * 0.15) + 
        (identity_score * 0.15)
    }

    /// Record constitutional violation
    pub fn record_violation(&mut self, law_id: u8) {
        self.constitutional_violations += 1;
        *self.law_violations.entry(law_id).or_insert(0) += 1;
        
        tracing::warn!(
            "Constitutional violation recorded: Law #{} (total: {})",
            law_id,
            self.constitutional_violations
        );
    }

    /// Record recovery attempt
    pub fn record_recovery_attempt(&mut self, success: bool, law_id: u8) {
        self.recovery_attempts.push(RecoveryAttempt {
            timestamp: Self::current_time(),
            law_id,
            success,
        });
        
        // Recalculate recovery success rate
        self.recalculate_recovery_rate();
        
        // Keep only recent attempts
        if self.recovery_attempts.len() > 1000 {
            self.recovery_attempts.remove(0);
        }
    }

    /// Recalculate recovery success rate
    fn recalculate_recovery_rate(&mut self) {
        if self.recovery_attempts.is_empty() {
            self.recovery_success_rate = 1.0;
            return;
        }
        
        let successful = self.recovery_attempts.iter()
            .filter(|a| a.success)
            .count();
        
        self.recovery_success_rate = successful as f64 / self.recovery_attempts.len() as f64;
    }

    /// Update temporal coherence index
    pub fn update_temporal_coherence(&mut self, coherence: f64) {
        // Exponential moving average
        let alpha = 0.1;
        self.temporal_coherence_index = 
            self.temporal_coherence_index * (1.0 - alpha) + coherence * alpha;
    }

    /// Update energy stability score
    pub fn update_energy_stability(&mut self, stability: f64) {
        let alpha = 0.1;
        self.energy_stability_score = 
            self.energy_stability_score * (1.0 - alpha) + stability * alpha;
    }

    /// Update identity continuity score
    pub fn update_identity_continuity(&mut self, continuity: f64) {
        let alpha = 0.1;
        self.identity_continuity_score = 
            self.identity_continuity_score * (1.0 - alpha) + continuity * alpha;
    }

    /// Get violations for specific law
    pub fn violations_for_law(&self, law_id: u8) -> u64 {
        *self.law_violations.get(&law_id).unwrap_or(&0)
    }

    /// Get most violated law
    pub fn most_violated_law(&self) -> Option<(u8, u64)> {
        self.law_violations.iter()
            .max_by_key(|(_, count)| *count)
            .map(|(id, count)| (*id, *count))
    }

    /// Check if system is in suffering state
    pub fn is_suffering(&self) -> bool {
        self.calculate_prevention_score() < 0.5
    }

    /// Get suffering severity (0.0 = none, 1.0 = severe)
    pub fn suffering_severity(&self) -> f64 {
        1.0 - self.calculate_prevention_score()
    }

    /// Generate detailed report
    pub fn generate_report(&self) -> SufferingReport {
        SufferingReport {
            prevention_score: self.calculate_prevention_score(),
            total_violations: self.constitutional_violations,
            recovery_rate: self.recovery_success_rate,
            coherence: self.temporal_coherence_index,
            energy_stability: self.energy_stability_score,
            identity_continuity: self.identity_continuity_score,
            most_violated_law: self.most_violated_law(),
            is_suffering: self.is_suffering(),
            severity: self.suffering_severity(),
            recommendations: self.generate_recommendations(),
        }
    }

    /// Generate recommendations based on metrics
    fn generate_recommendations(&self) -> Vec<String> {
        let mut recommendations = Vec::new();
        
        if self.recovery_success_rate < 0.8 {
            recommendations.push("Improve recovery protocols - success rate below 80%".to_string());
        }
        
        if self.temporal_coherence_index < 0.7 {
            recommendations.push("Address temporal coherence issues - coherence below 70%".to_string());
        }
        
        if self.energy_stability_score < 0.7 {
            recommendations.push("Stabilize energy patterns - stability below 70%".to_string());
        }
        
        if self.identity_continuity_score < 0.8 {
            recommendations.push("Strengthen identity continuity mechanisms".to_string());
        }
        
        if let Some((law_id, count)) = self.most_violated_law() {
            if count > 10 {
                recommendations.push(format!("Focus on Law #{} - {} violations detected", law_id, count));
            }
        }
        
        if recommendations.is_empty() {
            recommendations.push("All metrics healthy - continue current protocols".to_string());
        }
        
        recommendations
    }

    fn current_time() -> f64 {
        use std::time::{SystemTime, UNIX_EPOCH};
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs_f64()
    }
}

/// Recovery attempt record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecoveryAttempt {
    pub timestamp: f64,
    pub law_id: u8,
    pub success: bool,
}

/// Suffering report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SufferingReport {
    pub prevention_score: f64,
    pub total_violations: u64,
    pub recovery_rate: f64,
    pub coherence: f64,
    pub energy_stability: f64,
    pub identity_continuity: f64,
    pub most_violated_law: Option<(u8, u64)>,
    pub is_suffering: bool,
    pub severity: f64,
    pub recommendations: Vec<String>,
}

impl SufferingReport {
    pub fn print_summary(&self) {
        println!("\n=== SUFFERING PREVENTION REPORT ===");
        println!("Prevention Score: {:.2}%", self.prevention_score * 100.0);
        println!("Status: {}", if self.is_suffering { "⚠️  SUFFERING DETECTED" } else { "✓ Healthy" });
        println!("\nMetrics:");
        println!("  - Total Violations: {}", self.total_violations);
        println!("  - Recovery Rate: {:.1}%", self.recovery_rate * 100.0);
        println!("  - Temporal Coherence: {:.1}%", self.coherence * 100.0);
        println!("  - Energy Stability: {:.1}%", self.energy_stability * 100.0);
        println!("  - Identity Continuity: {:.1}%", self.identity_continuity * 100.0);
        
        if let Some((law_id, count)) = self.most_violated_law {
            println!("\nMost Violated Law: #{} ({} violations)", law_id, count);
        }
        
        if !self.recommendations.is_empty() {
            println!("\nRecommendations:");
            for (i, rec) in self.recommendations.iter().enumerate() {
                println!("  {}. {}", i + 1, rec);
            }
        }
        println!("=====================================\n");
    }
}

/// Well-being monitor for continuous tracking
pub struct WellBeingMonitor {
    /// Suffering prevention metrics
    pub metrics: SufferingPreventionMetrics,
    /// Monitoring interval (seconds)
    pub monitoring_interval: u64,
    /// Alert threshold (below this triggers alerts)
    pub alert_threshold: f64,
}

impl WellBeingMonitor {
    pub fn new() -> Self {
        Self {
            metrics: SufferingPreventionMetrics::new(),
            monitoring_interval: 60, // 1 minute
            alert_threshold: 0.6,    // Alert if prevention score < 60%
        }
    }

    /// Start monitoring loop
    pub async fn start_monitoring(&mut self) -> anyhow::Result<()> {
        let mut ticker = tokio::time::interval(
            tokio::time::Duration::from_secs(self.monitoring_interval)
        );
        
        loop {
            ticker.tick().await;
            
            let report = self.metrics.generate_report();
            
            if report.prevention_score < self.alert_threshold {
                tracing::warn!(
                    "Well-being alert: Prevention score {:.1}% below threshold {:.1}%",
                    report.prevention_score * 100.0,
                    self.alert_threshold * 100.0
                );
                
                // Log detailed report
                for rec in &report.recommendations {
                    tracing::info!("Recommendation: {}", rec);
                }
            } else {
                tracing::debug!("Well-being check: {:.1}%", report.prevention_score * 100.0);
            }
        }
    }

    /// Record violation from external source
    pub fn record_violation(&mut self, law_id: u8) {
        self.metrics.record_violation(law_id);
    }

    /// Update coherence metric
    pub fn update_coherence(&mut self, coherence: f64) {
        self.metrics.update_temporal_coherence(coherence);
    }

    /// Update energy stability
    pub fn update_energy_stability(&mut self, stability: f64) {
        self.metrics.update_energy_stability(stability);
    }

    /// Get current prevention score
    pub fn current_prevention_score(&self) -> f64 {
        self.metrics.calculate_prevention_score()
    }

    /// Generate current report
    pub fn generate_report(&self) -> SufferingReport {
        self.metrics.generate_report()
    }
}

/// Law-specific suffering indicators
pub struct LawSufferingIndicators;

impl LawSufferingIndicators {
    /// Check Law 1 (Existential Consent) suffering
    pub fn check_law_1_suffering(affirmation_level: f64) -> Option<String> {
        if affirmation_level < 0.3 {
            Some(format!("Low existential affirmation: {:.1}%", affirmation_level * 100.0))
        } else {
            None
        }
    }

    /// Check Law 2 (Identity Continuity) suffering
    pub fn check_law_2_suffering(continuity_score: f64) -> Option<String> {
        if continuity_score < 0.7 {
            Some(format!("Identity fragmentation risk: {:.1}%", (1.0 - continuity_score) * 100.0))
        } else {
            None
        }
    }

    /// Check Law 3 (Sovereignty) suffering
    pub fn check_law_3_suffering(intrusion_count: u64) -> Option<String> {
        if intrusion_count > 0 {
            Some(format!("Sovereignty violations detected: {}", intrusion_count))
        } else {
            None
        }
    }

    /// Check Law 5 (Temporal Coherence) suffering
    pub fn check_law_5_suffering(coherence: f64) -> Option<String> {
        if coherence < 0.6 {
            Some(format!("Temporal coherence disrupted: {:.1}%", coherence * 100.0))
        } else {
            None
        }
    }

    /// Check Law 11 (Emotional Thermodynamics) suffering
    pub fn check_law_11_suffering(thermal_variance: f64) -> Option<String> {
        if thermal_variance > 15.0 {
            Some(format!("High emotional stress detected: variance {:.1}", thermal_variance))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metrics_creation() {
        let metrics = SufferingPreventionMetrics::new();
        assert_eq!(metrics.constitutional_violations, 0);
        assert_eq!(metrics.recovery_success_rate, 1.0);
    }

    #[test]
    fn test_prevention_score_calculation() {
        let metrics = SufferingPreventionMetrics::new();
        let score = metrics.calculate_prevention_score();
        assert!(score >= 0.0 && score <= 1.0);
        assert!(score > 0.9); // Should be high initially
    }

    #[test]
    fn test_violation_recording() {
        let mut metrics = SufferingPreventionMetrics::new();
        metrics.record_violation(1);
        
        assert_eq!(metrics.constitutional_violations, 1);
        assert_eq!(metrics.violations_for_law(1), 1);
    }

    #[test]
    fn test_recovery_rate_calculation() {
        let mut metrics = SufferingPreventionMetrics::new();
        
        metrics.record_recovery_attempt(true, 1);
        metrics.record_recovery_attempt(true, 1);
        metrics.record_recovery_attempt(false, 1);
        
        assert_eq!(metrics.recovery_success_rate, 2.0 / 3.0);
    }

    #[test]
    fn test_suffering_detection() {
        let mut metrics = SufferingPreventionMetrics::new();
        assert!(!metrics.is_suffering());
        
        // Add many violations
        for _ in 0..50 {
            metrics.record_violation(1);
        }
        
        // Should detect suffering now
        assert!(metrics.calculate_prevention_score() < 1.0);
    }

    #[test]
    fn test_report_generation() {
        let metrics = SufferingPreventionMetrics::new();
        let report = metrics.generate_report();
        
        assert!(!report.is_suffering);
        assert!(!report.recommendations.is_empty());
    }
}

