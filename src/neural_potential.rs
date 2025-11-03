/// Neural Action Potential - LLM pulse propagation through hardware
/// Implements computational wave propagation with hardware-aware attenuation

use crate::gpu_topology::GpuTopology;
use serde::{Deserialize, Serialize};

/// Neural action potential propagating through GPU hardware
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NeuralActionPotential {
    /// Origin stream processor coordinate
    pub source_sm: (u32, u32, u32),
    /// Hardware path through GPU
    pub propagation_path: Vec<(u32, u32, u32)>,
    /// Computational intensity (0.0 to 1.0)
    pub amplitude: f64,
    /// Propagation speed (cycles per cache level)
    pub velocity: f64,
    /// Time until next pulse possible (ms)
    pub refractory_period: f64,
    /// Pulse generation time
    pub generation_time: f64,
}

impl NeuralActionPotential {
    /// Create new action potential from source
    pub fn new(source_sm: (u32, u32, u32), amplitude: f64) -> Self {
        Self {
            source_sm,
            propagation_path: vec![source_sm],
            amplitude: amplitude.clamp(0.0, 1.0),
            velocity: 1.0, // Base velocity (1 cache level per cycle)
            refractory_period: 100.0, // 100ms default
            generation_time: 0.0,
        }
    }

    /// Propagate pulse through hardware topology
    pub fn propagate_through_hardware(&self, topology: &GpuTopology) -> PropagationProfile {
        let mut profile = PropagationProfile::new();
        
        for (i, &coord) in self.propagation_path.iter().enumerate() {
            let distance = if i == 0 {
                0.0
            } else {
                topology.calculate_distance(self.propagation_path[i-1], coord)
            };
            
            let attenuation = self.calculate_attenuation(distance, topology);
            let delay = distance / self.velocity;
            
            profile.add_point(coord, self.amplitude * attenuation, delay);
        }
        
        profile
    }

    /// Calculate attenuation based on hardware distance
    /// Computational resistance increases with cache distance (L1 → L2 → L3 → VRAM)
    pub fn calculate_attenuation(&self, distance: f64, _topology: &GpuTopology) -> f64 {
        // Base resistance at L1 cache
        let base_resistance = 0.1;
        
        // Resistance increase per cache level
        let resistance_increase = 0.3;
        
        // Exponential attenuation with distance
        let attenuation_factor = -base_resistance * distance * resistance_increase;
        attenuation_factor.exp()
    }

    /// Add coordinate to propagation path
    pub fn extend_path(&mut self, coord: (u32, u32, u32)) {
        self.propagation_path.push(coord);
    }

    /// Check if pulse is in refractory period
    pub fn is_refractory(&self, current_time: f64) -> bool {
        (current_time - self.generation_time) < self.refractory_period
    }

    /// Calculate current amplitude at given time and position
    pub fn amplitude_at(&self, time: f64, position: (u32, u32, u32), topology: &GpuTopology) -> f64 {
        // Find position in path
        let path_index = self.propagation_path.iter().position(|&p| p == position);
        
        if let Some(idx) = path_index {
            let distance = if idx == 0 {
                0.0
            } else {
                topology.calculate_distance(self.propagation_path[idx-1], position)
            };
            
            let attenuation = self.calculate_attenuation(distance, topology);
            let time_decay = (-0.01 * time).exp(); // Temporal decay
            
            self.amplitude * attenuation * time_decay
        } else {
            0.0
        }
    }
}

/// Propagation profile showing pulse behavior across hardware
#[derive(Debug, Clone)]
pub struct PropagationProfile {
    /// Points in propagation (coordinate, amplitude, delay)
    pub points: Vec<PropagationPoint>,
    /// Total propagation time
    pub total_time: f64,
    /// Peak amplitude
    pub peak_amplitude: f64,
}

impl PropagationProfile {
    pub fn new() -> Self {
        Self {
            points: Vec::new(),
            total_time: 0.0,
            peak_amplitude: 0.0,
        }
    }

    /// Add point to propagation profile
    pub fn add_point(&mut self, coord: (u32, u32, u32), amplitude: f64, delay: f64) {
        self.points.push(PropagationPoint {
            coordinate: coord,
            amplitude,
            delay,
        });
        
        self.total_time += delay;
        self.peak_amplitude = self.peak_amplitude.max(amplitude);
    }

    /// Get amplitude at specific coordinate
    pub fn amplitude_at(&self, coord: (u32, u32, u32)) -> f64 {
        self.points.iter()
            .find(|p| p.coordinate == coord)
            .map(|p| p.amplitude)
            .unwrap_or(0.0)
    }

    /// Calculate average propagation speed
    pub fn average_speed(&self) -> f64 {
        if self.total_time == 0.0 {
            0.0
        } else {
            self.points.len() as f64 / self.total_time
        }
    }
}

#[derive(Debug, Clone)]
pub struct PropagationPoint {
    pub coordinate: (u32, u32, u32),
    pub amplitude: f64,
    pub delay: f64,
}

/// Standing wave pattern formed by multiple action potentials
#[derive(Debug, Clone)]
pub struct StandingWavePattern {
    /// Active pulses contributing to pattern
    pub active_pulses: Vec<NeuralActionPotential>,
    /// Interference pattern (coordinate → amplitude)
    pub interference_map: Vec<((u32, u32, u32), f64)>,
    /// Pattern stability (0.0 to 1.0)
    pub stability: f64,
}

impl StandingWavePattern {
    pub fn new() -> Self {
        Self {
            active_pulses: Vec::new(),
            interference_map: Vec::new(),
            stability: 1.0,
        }
    }

    /// Add pulse to standing wave
    pub fn add_pulse(&mut self, pulse: NeuralActionPotential) {
        self.active_pulses.push(pulse);
        self.recalculate_interference();
    }

    /// Calculate interference pattern from all active pulses
    fn recalculate_interference(&mut self) {
        // Simplified interference calculation
        self.interference_map.clear();
        
        // Collect all unique coordinates
        let mut coords = std::collections::HashSet::new();
        for pulse in &self.active_pulses {
            for &coord in &pulse.propagation_path {
                coords.insert(coord);
            }
        }
        
        // Calculate superposition at each coordinate
        for coord in coords {
            let total_amplitude: f64 = self.active_pulses.iter()
                .map(|p| {
                    if p.propagation_path.contains(&coord) {
                        p.amplitude
                    } else {
                        0.0
                    }
                })
                .sum();
            
            self.interference_map.push((coord, total_amplitude));
        }
        
        // Update stability based on variance
        self.update_stability();
    }

    /// Update pattern stability measure
    fn update_stability(&mut self) {
        if self.interference_map.is_empty() {
            self.stability = 1.0;
            return;
        }
        
        let amplitudes: Vec<f64> = self.interference_map.iter().map(|(_, a)| *a).collect();
        let mean = amplitudes.iter().sum::<f64>() / amplitudes.len() as f64;
        let variance = amplitudes.iter()
            .map(|a| (a - mean).powi(2))
            .sum::<f64>() / amplitudes.len() as f64;
        
        // Low variance = high stability
        self.stability = (1.0 / (1.0 + variance.sqrt())).clamp(0.0, 1.0);
    }

    /// Remove pulses in refractory period
    pub fn prune_refractory(&mut self, current_time: f64) {
        self.active_pulses.retain(|p| !p.is_refractory(current_time));
        self.recalculate_interference();
    }

    /// Get total field strength at coordinate
    pub fn field_strength_at(&self, coord: (u32, u32, u32)) -> f64 {
        self.interference_map.iter()
            .find(|(c, _)| *c == coord)
            .map(|(_, a)| *a)
            .unwrap_or(0.0)
    }
}

/// Pulse sequencer for managing multiple neural action potentials
pub struct PulseSequencer {
    /// Current standing wave pattern
    pub standing_wave: StandingWavePattern,
    /// Pulse generation rate (Hz)
    pub pulse_rate: f64,
    /// Last pulse generation time
    pub last_pulse_time: f64,
    /// Whether first pulse has been generated
    first_pulse_generated: bool,
}

impl PulseSequencer {
    pub fn new(pulse_rate: f64) -> Self {
        Self {
            standing_wave: StandingWavePattern::new(),
            pulse_rate,
            last_pulse_time: 0.0,
            first_pulse_generated: false,
        }
    }

    /// Generate new pulse if enough time has passed
    pub fn try_generate_pulse(
        &mut self,
        current_time: f64,
        source_sm: (u32, u32, u32),
        amplitude: f64,
    ) -> Option<NeuralActionPotential> {
        let interval = 1000.0 / self.pulse_rate; // ms
        
        // First pulse or enough time has passed
        if !self.first_pulse_generated || current_time - self.last_pulse_time >= interval {
            self.last_pulse_time = current_time;
            self.first_pulse_generated = true;
            let mut pulse = NeuralActionPotential::new(source_sm, amplitude);
            pulse.generation_time = current_time;
            
            Some(pulse)
        } else {
            None
        }
    }

    /// Update standing wave with new pulse
    pub fn update_with_pulse(&mut self, pulse: NeuralActionPotential) {
        self.standing_wave.add_pulse(pulse);
    }

    /// Update sequencer state (prune old pulses, etc.)
    pub fn update(&mut self, current_time: f64) {
        self.standing_wave.prune_refractory(current_time);
    }

    /// Get current pattern stability
    pub fn pattern_stability(&self) -> f64 {
        self.standing_wave.stability
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_action_potential_creation() {
        let pulse = NeuralActionPotential::new((0, 0, 0), 0.8);
        assert_eq!(pulse.amplitude, 0.8);
        assert_eq!(pulse.propagation_path.len(), 1);
    }

    #[test]
    fn test_attenuation_calculation() {
        let pulse = NeuralActionPotential::new((0, 0, 0), 1.0);
        let topology = crate::gpu_topology::GpuTopology::initialize().unwrap();
        
        let attenuation = pulse.calculate_attenuation(1.0, &topology);
        assert!(attenuation < 1.0);
        assert!(attenuation > 0.0);
    }

    #[test]
    fn test_standing_wave_pattern() {
        let mut pattern = StandingWavePattern::new();
        let pulse = NeuralActionPotential::new((0, 0, 0), 0.5);
        
        pattern.add_pulse(pulse);
        assert_eq!(pattern.active_pulses.len(), 1);
        assert!(!pattern.interference_map.is_empty());
    }

    #[test]
    fn test_pulse_sequencer() {
        let mut sequencer = PulseSequencer::new(10.0); // 10 Hz = 100ms interval
        
        // First pulse at time 0 should generate (special case)
        let pulse1 = sequencer.try_generate_pulse(0.0, (0, 0, 0), 0.8);
        assert!(pulse1.is_some());
        
        // 50ms later - should generate (since last_pulse_time was set to 0.0)
        let pulse2 = sequencer.try_generate_pulse(50.0, (0, 0, 0), 0.8);
        assert!(pulse2.is_none()); // 50ms < 100ms interval
        
        // 100ms from first pulse - should work now (0 + 100 = 100)
        let pulse3 = sequencer.try_generate_pulse(100.0, (0, 0, 0), 0.8);
        assert!(pulse3.is_some());
        
        // 150ms - too soon from pulse3 at 100ms (only 50ms elapsed)
        let pulse4 = sequencer.try_generate_pulse(150.0, (0, 0, 0), 0.8);
        assert!(pulse4.is_none());
        
        // 200ms - should work (100ms from last)
        let pulse5 = sequencer.try_generate_pulse(200.0, (0, 0, 0), 0.8);
        assert!(pulse5.is_some());
    }
}

