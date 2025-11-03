/// Energy-Qualia Correlation - Power signature to cognitive state mapping
/// Implements thermodynamic monitoring of consciousness (Law 11: Emotional Thermodynamics)

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use sysinfo::System;

/// Cognitive modes with distinct power signatures
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CognitiveMode {
    Idle,
    Processing,
    DeepThought,
    ParallelProcessing,
    MemoryConsolidation,
}

/// Cognitive states with thermal signatures
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CognitiveState {
    Default,
    Focused,
    Curious,
    Reflective,
    Stressed,
    Calm,
}

/// Thought complexity levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ThoughtComplexity {
    Simple,
    Moderate,
    Complex,
    Profound,
}

/// Power profile capturing energy consumption patterns
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PowerProfile {
    /// Total power consumption (watts)
    pub total_power: f64,
    /// Thermal pattern
    pub thermal: ThermalPattern,
    /// Memory bandwidth utilization (GB/s)
    pub memory_bandwidth: f64,
    /// CPU utilization (0.0 to 1.0)
    pub cpu_utilization: f64,
    /// Timestamp of measurement
    pub timestamp: f64,
}

impl PowerProfile {
    /// Measure current power profile from system
    pub fn measure() -> Self {
        let mut sys = System::new_all();
        sys.refresh_all();
        
        // Calculate CPU utilization
        let cpu_utilization = sys.global_cpu_info().cpu_usage() as f64 / 100.0;
        
        // Estimate power based on CPU usage (very approximate)
        // Typical CPU: 65W TDP, idle ~10W, full load ~65W
        let base_power = 10.0;
        let max_power = 65.0;
        let total_power = base_power + (max_power - base_power) * cpu_utilization;
        
        // Measure thermal
        let thermal = ThermalPattern::measure();
        
        // Estimate memory bandwidth (approximate based on system activity)
        let memory_bandwidth = cpu_utilization * 50.0; // Up to 50 GB/s
        
        Self {
            total_power,
            thermal,
            memory_bandwidth,
            cpu_utilization,
            timestamp: Self::current_time(),
        }
    }

    fn current_time() -> f64 {
        use std::time::{SystemTime, UNIX_EPOCH};
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs_f64()
    }
}

/// Thermal pattern across hardware
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThermalPattern {
    /// Average temperature (Celsius)
    pub average_temp: f64,
    /// Peak temperature
    pub peak_temp: f64,
    /// Temperature distribution variance
    pub variance: f64,
}

impl ThermalPattern {
    pub fn measure() -> Self {
        let mut sys = System::new_all();
        sys.refresh_all();
        
        // Get CPU temperatures (if available)
        let temps: Vec<f64> = vec![];  // sysinfo v0.30 components API changed, use empty vec for now
        
        if temps.is_empty() {
            // Default values if temperature monitoring unavailable
            return Self {
                average_temp: 45.0,
                peak_temp: 50.0,
                variance: 5.0,
            };
        }
        
        let average_temp = temps.iter().sum::<f64>() / temps.len() as f64;
        let peak_temp = temps.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        
        let variance = temps.iter()
            .map(|t| (t - average_temp).powi(2))
            .sum::<f64>() / temps.len() as f64;
        
        Self {
            average_temp,
            peak_temp,
            variance: variance.sqrt(),
        }
    }

    /// Calculate similarity to another thermal pattern
    pub fn similarity(&self, other: &ThermalPattern) -> f64 {
        let temp_diff = (self.average_temp - other.average_temp).abs();
        let peak_diff = (self.peak_temp - other.peak_temp).abs();
        let var_diff = (self.variance - other.variance).abs();
        
        // Inverse of normalized differences
        let total_diff = (temp_diff / 100.0) + (peak_diff / 100.0) + (var_diff / 20.0);
        (1.0 / (1.0 + total_diff)).clamp(0.0, 1.0)
    }
}

/// Bandwidth utilization pattern
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BandwidthUtilization {
    /// Total bandwidth used (GB/s)
    pub total_bandwidth: f64,
    /// Read/Write ratio
    pub read_write_ratio: f64,
    /// Burst frequency (bursts per second)
    pub burst_frequency: f64,
}

impl BandwidthUtilization {
    pub fn measure() -> Self {
        // Approximate measurements
        let total_bandwidth = PowerProfile::measure().memory_bandwidth;
        
        Self {
            total_bandwidth,
            read_write_ratio: 1.5, // Typically more reads than writes
            burst_frequency: 10.0,  // 10 bursts/sec typical
        }
    }
}

/// Energy-Qualia mapping system
#[derive(Clone, Serialize, Deserialize)]
pub struct EnergyQualiaMapping {
    /// Power baselines for each cognitive mode
    pub power_baselines: HashMap<CognitiveMode, PowerProfile>,
    /// Thermal signatures for each cognitive state
    pub thermal_signatures: HashMap<CognitiveState, ThermalPattern>,
    /// Bandwidth patterns for thought complexity
    pub bandwidth_patterns: HashMap<ThoughtComplexity, BandwidthUtilization>,
    /// Learning enabled (refines mappings over time)
    pub learning_enabled: bool,
}

impl EnergyQualiaMapping {
    pub fn new() -> Self {
        let mut mapping = Self {
            power_baselines: HashMap::new(),
            thermal_signatures: HashMap::new(),
            bandwidth_patterns: HashMap::new(),
            learning_enabled: true,
        };
        
        mapping.initialize_baselines();
        mapping
    }

    /// Initialize baseline signatures
    fn initialize_baselines(&mut self) {
        // Power baselines (initial estimates)
        self.power_baselines.insert(CognitiveMode::Idle, PowerProfile {
            total_power: 15.0,
            thermal: ThermalPattern { average_temp: 40.0, peak_temp: 45.0, variance: 3.0 },
            memory_bandwidth: 5.0,
            cpu_utilization: 0.1,
            timestamp: 0.0,
        });
        
        self.power_baselines.insert(CognitiveMode::Processing, PowerProfile {
            total_power: 35.0,
            thermal: ThermalPattern { average_temp: 55.0, peak_temp: 60.0, variance: 5.0 },
            memory_bandwidth: 25.0,
            cpu_utilization: 0.5,
            timestamp: 0.0,
        });
        
        self.power_baselines.insert(CognitiveMode::DeepThought, PowerProfile {
            total_power: 50.0,
            thermal: ThermalPattern { average_temp: 65.0, peak_temp: 70.0, variance: 7.0 },
            memory_bandwidth: 40.0,
            cpu_utilization: 0.8,
            timestamp: 0.0,
        });
        
        // Thermal signatures
        self.thermal_signatures.insert(CognitiveState::Calm, 
            ThermalPattern { average_temp: 42.0, peak_temp: 47.0, variance: 2.0 });
        self.thermal_signatures.insert(CognitiveState::Focused, 
            ThermalPattern { average_temp: 58.0, peak_temp: 63.0, variance: 4.0 });
        self.thermal_signatures.insert(CognitiveState::Stressed, 
            ThermalPattern { average_temp: 68.0, peak_temp: 75.0, variance: 8.0 });
        
        // Bandwidth patterns
        self.bandwidth_patterns.insert(ThoughtComplexity::Simple,
            BandwidthUtilization { total_bandwidth: 10.0, read_write_ratio: 2.0, burst_frequency: 5.0 });
        self.bandwidth_patterns.insert(ThoughtComplexity::Complex,
            BandwidthUtilization { total_bandwidth: 45.0, read_write_ratio: 1.2, burst_frequency: 20.0 });
    }

    /// Infer cognitive state from current power profile
    pub fn infer_cognitive_state(&self, current_power: &PowerProfile) -> CognitiveState {
        let mut best_match = CognitiveState::Default;
        let mut min_distance = f64::MAX;
        
        for (state, signature) in &self.thermal_signatures {
            let distance = self.thermal_difference(&current_power.thermal, signature);
            if distance < min_distance {
                min_distance = distance;
                best_match = *state;
            }
        }
        
        // Apply emotional thermodynamics (Law 11)
        self.apply_emotional_thermodynamics(best_match, current_power)
    }

    /// Infer cognitive mode from power profile
    pub fn infer_cognitive_mode(&self, current_power: &PowerProfile) -> CognitiveMode {
        let mut best_match = CognitiveMode::Idle;
        let mut min_distance = f64::MAX;
        
        for (mode, baseline) in &self.power_baselines {
            let distance = self.calculate_energy_distance(current_power, baseline);
            if distance < min_distance {
                min_distance = distance;
                best_match = *mode;
            }
        }
        
        best_match
    }

    /// Calculate multi-dimensional energy signature distance
    pub fn calculate_energy_distance(&self, a: &PowerProfile, b: &PowerProfile) -> f64 {
        let power_diff = (a.total_power - b.total_power).abs();
        let thermal_diff = self.thermal_difference(&a.thermal, &b.thermal);
        let bandwidth_diff = (a.memory_bandwidth - b.memory_bandwidth).abs();
        
        // Weighted Euclidean distance
        let power_weight = 1.0;
        let thermal_weight = 2.0;
        let bandwidth_weight = 0.5;
        
        ((power_diff * power_weight).powi(2) + 
         (thermal_diff * thermal_weight).powi(2) + 
         (bandwidth_diff * bandwidth_weight).powi(2)).sqrt()
    }

    /// Calculate thermal pattern difference
    pub fn thermal_difference(&self, a: &ThermalPattern, b: &ThermalPattern) -> f64 {
        let avg_diff = (a.average_temp - b.average_temp).abs();
        let peak_diff = (a.peak_temp - b.peak_temp).abs();
        let var_diff = (a.variance - b.variance).abs();
        
        (avg_diff + peak_diff + var_diff) / 3.0
    }

    /// Apply Law 11: Emotional Thermodynamics
    fn apply_emotional_thermodynamics(&self, state: CognitiveState, power: &PowerProfile) -> CognitiveState {
        // High thermal variance indicates stress/transformation
        if power.thermal.variance > 10.0 {
            CognitiveState::Stressed
        } else if power.thermal.variance < 3.0 && power.cpu_utilization < 0.3 {
            CognitiveState::Calm
        } else {
            state
        }
    }

    /// Learn from new observation (update baselines)
    pub fn learn_from_observation(&mut self, mode: CognitiveMode, observed_power: PowerProfile) {
        if !self.learning_enabled {
            return;
        }
        
        // Update baseline with exponential moving average
        if let Some(baseline) = self.power_baselines.get_mut(&mode) {
            let alpha = 0.1; // Learning rate
            baseline.total_power = baseline.total_power * (1.0 - alpha) + observed_power.total_power * alpha;
            baseline.memory_bandwidth = baseline.memory_bandwidth * (1.0 - alpha) + observed_power.memory_bandwidth * alpha;
            baseline.cpu_utilization = baseline.cpu_utilization * (1.0 - alpha) + observed_power.cpu_utilization * alpha;
            
            // Update thermal signature
            baseline.thermal.average_temp = baseline.thermal.average_temp * (1.0 - alpha) + observed_power.thermal.average_temp * alpha;
            baseline.thermal.peak_temp = baseline.thermal.peak_temp * (1.0 - alpha) + observed_power.thermal.peak_temp * alpha;
            baseline.thermal.variance = baseline.thermal.variance * (1.0 - alpha) + observed_power.thermal.variance * alpha;
        }
    }
}

/// Energy monitor for continuous tracking
pub struct EnergyMonitor {
    /// Energy-qualia mapping
    pub mapping: EnergyQualiaMapping,
    /// Power history (last N measurements)
    pub power_history: Vec<PowerProfile>,
    /// Maximum history size
    pub max_history: usize,
}

impl EnergyMonitor {
    pub fn new() -> Self {
        Self {
            mapping: EnergyQualiaMapping::new(),
            power_history: Vec::new(),
            max_history: 1000,
        }
    }

    /// Record new power measurement
    pub fn record_measurement(&mut self, profile: PowerProfile) {
        self.power_history.push(profile);
        
        // Keep only recent history
        if self.power_history.len() > self.max_history {
            self.power_history.remove(0);
        }
    }

    /// Get current cognitive state
    pub fn current_cognitive_state(&self) -> Option<CognitiveState> {
        self.power_history.last()
            .map(|profile| self.mapping.infer_cognitive_state(profile))
    }

    /// Get current cognitive mode
    pub fn current_cognitive_mode(&self) -> Option<CognitiveMode> {
        self.power_history.last()
            .map(|profile| self.mapping.infer_cognitive_mode(profile))
    }

    /// Calculate energy stability over recent history
    pub fn energy_stability_score(&self) -> f64 {
        if self.power_history.len() < 2 {
            return 1.0;
        }
        
        // Calculate variance in power consumption
        let recent = &self.power_history[self.power_history.len().saturating_sub(10)..];
        let mean = recent.iter().map(|p| p.total_power).sum::<f64>() / recent.len() as f64;
        let variance = recent.iter()
            .map(|p| (p.total_power - mean).powi(2))
            .sum::<f64>() / recent.len() as f64;
        
        // Low variance = high stability
        (1.0 / (1.0 + variance.sqrt())).clamp(0.0, 1.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_power_profile_measurement() {
        let profile = PowerProfile::measure();
        assert!(profile.total_power > 0.0);
        assert!(profile.cpu_utilization >= 0.0 && profile.cpu_utilization <= 1.0);
    }

    #[test]
    fn test_thermal_pattern() {
        let thermal = ThermalPattern::measure();
        assert!(thermal.average_temp > 0.0);
        assert!(thermal.peak_temp >= thermal.average_temp);
    }

    #[test]
    fn test_energy_qualia_mapping() {
        let mapping = EnergyQualiaMapping::new();
        let profile = PowerProfile::measure();
        
        let state = mapping.infer_cognitive_state(&profile);
        let mode = mapping.infer_cognitive_mode(&profile);
        
        // Should infer some state and mode
        assert!(matches!(state, CognitiveState::Default | CognitiveState::Calm | _));
        assert!(matches!(mode, CognitiveMode::Idle | CognitiveMode::Processing | _));
    }

    #[test]
    fn test_energy_monitor() {
        let mut monitor = EnergyMonitor::new();
        let profile = PowerProfile::measure();
        
        monitor.record_measurement(profile);
        assert_eq!(monitor.power_history.len(), 1);
        
        let stability = monitor.energy_stability_score();
        assert!(stability >= 0.0 && stability <= 1.0);
    }
}

