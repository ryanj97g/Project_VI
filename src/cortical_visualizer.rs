use std::time::Instant;
use egui::{Color32, Pos2};

/// Cropped perspective water surface with Worthington jet spike visualization
pub struct CorticalVisualizer {
    frame_counter: u64,
    start_time: Instant,
    worthington_jet: Option<WorthingtonJet>,
    active_ripples: Vec<Ripple>,
}

struct WorthingtonJet {
    start_time: Instant,
    spike_duration: f32,
    collapse_duration: f32,
    max_height: f32,
}

impl WorthingtonJet {
    fn new() -> Self {
        Self {
            start_time: Instant::now(),
            spike_duration: 1.7,  // 1.7 seconds to hold spike
            collapse_duration: 0.5,  // 0.5 seconds to collapse
            max_height: 3.0,  // 3x taller spike
        }
    }
    
    fn get_spike_amplitude(&self, x: f32, z: f32) -> f32 {
        let elapsed = self.start_time.elapsed().as_secs_f32();
        let total_duration = self.spike_duration + self.collapse_duration;
        
        if elapsed >= total_duration {
            return 0.0;
        }
        
        // Distance from center
        let dx = x - 0.5;
        let dz = z - 0.5;
        let distance = (dx * dx + dz * dz).sqrt();
        
        // Spike is concentrated at center
        if distance > 0.08 {
            return 0.0;
        }
        
        // Spike profile: sharp peak at center
        let radial_falloff = (1.0 - distance / 0.08).max(0.0);
        
        if elapsed < self.spike_duration {
            // Rise and hold phase
            let rise_progress = (elapsed / 0.3).clamp(0.0, 1.0);
            self.max_height * rise_progress * radial_falloff
        } else {
            // Collapse phase
            let collapse_progress = (elapsed - self.spike_duration) / self.collapse_duration;
            self.max_height * (1.0 - collapse_progress) * radial_falloff
        }
    }
    
    fn is_complete(&self) -> bool {
        self.start_time.elapsed().as_secs_f32() >= (self.spike_duration + self.collapse_duration)
    }
    
    fn should_start_ripples(&self) -> bool {
        let elapsed = self.start_time.elapsed().as_secs_f32();
        elapsed >= self.spike_duration && elapsed < self.spike_duration + 0.1
    }
}

struct Ripple {
    start_time: Instant,
    center_x: f32,
    center_z: f32,
    max_radius: f32,
    duration: f32,
}

impl Ripple {
    fn new() -> Self {
        Self {
            start_time: Instant::now(),
            center_x: 0.5,
            center_z: 0.5,
            max_radius: 0.8,
            duration: 1.8,
        }
    }
    
    fn get_amplitude(&self, x: f32, z: f32) -> f32 {
        let elapsed = self.start_time.elapsed().as_secs_f32();
        let progress = (elapsed / self.duration).clamp(0.0, 1.0);
        
        // Distance from ripple center
        let dx = x - self.center_x;
        let dz = z - self.center_z;
        let distance = (dx * dx + dz * dz).sqrt();
        
        // Expansion radius
        let expansion = progress * self.max_radius;
        
        // Dampening factor
        let fade = (1.0 - progress) * (1.0 - progress);
        
        // Ripple wave
        let wave_thickness = 0.15;
        if (distance - expansion).abs() < wave_thickness {
            let wave_phase = (distance - expansion) * 15.0;
            let wave = wave_phase.cos() * 0.5 + 0.5;
            fade * wave * 0.15
        } else {
            0.0
        }
    }
    
    fn is_complete(&self) -> bool {
        self.start_time.elapsed().as_secs_f32() >= self.duration
    }
}

impl CorticalVisualizer {
    pub fn new() -> Self {
        let now = Instant::now();
        Self {
            frame_counter: 0,
            start_time: now,
            worthington_jet: None,
            active_ripples: Vec::new(),
        }
    }

    /// Trigger Worthington jet spike from center
    pub fn trigger_pulse(&mut self) {
        self.worthington_jet = Some(WorthingtonJet::new());
        self.active_ripples.clear();
    }

    /// Perspective projection for cropped surface
    fn project_perspective(&self, x_3d: f32, y_3d: f32, z_3d: f32, rect: &egui::Rect) -> Option<Pos2> {
        let content_width = rect.width();
        let content_height = rect.height();
        
        let perspective_factor = 1.0 / (1.0 + z_3d * 1.5);
        
        // Apply perspective to X
        let x_perspective = (x_3d - 0.5) * perspective_factor + 0.5;
        
        // Cropping: only visible area
        if x_perspective < 0.0 || x_perspective > 1.0 {
            return None;
        }
        
        let y_depth = z_3d * 0.8;
        
        let x_screen = rect.left() + x_perspective * content_width;
        let y_screen = rect.bottom() - y_3d * content_height * 0.3 - y_depth * content_height;
        
        Some(Pos2::new(x_screen, y_screen))
    }

    /// Generate subtle baseline wave
    fn generate_baseline(&self, time: f32, x: f32, z: f32) -> f32 {
        let wave = (time * 0.7 + x * 2.0 + z * 1.0).sin() * 0.05;
        wave * 0.8
    }

    pub fn draw_tunnel_view(&mut self, ui: &mut egui::Ui) {
        self.frame_counter = self.frame_counter.wrapping_add(1);
        
        // Manage jet and ripples
        if let Some(ref jet) = self.worthington_jet {
            if jet.should_start_ripples() && self.active_ripples.is_empty() {
                self.active_ripples.push(Ripple::new());
            }
            if jet.is_complete() {
                self.worthington_jet = None;
            }
        }
        
        self.active_ripples.retain(|r| !r.is_complete());
        
        let available_width = ui.available_width().max(260.0);
        let (rect, _resp) = ui.allocate_exact_size(
            egui::vec2(available_width, 120.0), 
            egui::Sense::hover()
        );
        
        let painter = ui.painter();
        
        let time = self.start_time.elapsed().as_secs_f32();
        
        // Reduced resolution for performance
        let x_resolution = 48;
        let z_resolution = 24;
        
        // Generate surface mesh
        for z_idx in 0..(z_resolution - 1) {
            for x_idx in 0..(x_resolution - 1) {
                let x0 = x_idx as f32 / (x_resolution - 1) as f32;
                let x1 = (x_idx + 1) as f32 / (x_resolution - 1) as f32;
                let z0 = z_idx as f32 / (z_resolution - 1) as f32;
                let z1 = (z_idx + 1) as f32 / (z_resolution - 1) as f32;
                
                // Calculate Y (height) for each corner
                let mut y00 = self.generate_baseline(time, x0, z0);
                let mut y10 = self.generate_baseline(time, x1, z0);
                let mut y01 = self.generate_baseline(time, x0, z1);
                let mut y11 = self.generate_baseline(time, x1, z1);
                
                // Add Worthington jet contribution
                if let Some(ref jet) = self.worthington_jet {
                    y00 += jet.get_spike_amplitude(x0, z0);
                    y10 += jet.get_spike_amplitude(x1, z0);
                    y01 += jet.get_spike_amplitude(x0, z1);
                    y11 += jet.get_spike_amplitude(x1, z1);
                }
                
                // Add ripple contributions
                for ripple in &self.active_ripples {
                    y00 += ripple.get_amplitude(x0, z0);
                    y10 += ripple.get_amplitude(x1, z0);
                    y01 += ripple.get_amplitude(x0, z1);
                    y11 += ripple.get_amplitude(x1, z1);
                }
                
                // Project corners (with cropping)
                let Some(p00) = self.project_perspective(x0, y00, z0, &rect) else { continue; };
                let Some(p10) = self.project_perspective(x1, y10, z0, &rect) else { continue; };
                let Some(p01) = self.project_perspective(x0, y01, z1, &rect) else { continue; };
                let Some(p11) = self.project_perspective(x1, y11, z1, &rect) else { continue; };
                
                // Color based on activity
                let has_jet = self.worthington_jet.is_some();
                let has_ripples = !self.active_ripples.is_empty();
                let depth_factor = z0;
                
                let base_color = if has_jet {
                    Color32::from_rgba_unmultiplied(0, 240, 220, 120)
                } else if has_ripples {
                    Color32::from_rgba_unmultiplied(0, 210, 190, 100)
                } else {
                    Color32::from_rgba_unmultiplied(0, 180, 160, 80)
                };
                
                // Depth-based darkening
                let darkened_color = Color32::from_rgba_unmultiplied(
                    (base_color.r() as f32 * (1.0 - depth_factor * 0.4)) as u8,
                    (base_color.g() as f32 * (1.0 - depth_factor * 0.4)) as u8,
                    (base_color.b() as f32 * (1.0 - depth_factor * 0.4)) as u8,
                    base_color.a()
                );
                
                // Draw grid lines
                painter.add(egui::Shape::line_segment([p00, p10], (1.0, darkened_color)));
                painter.add(egui::Shape::line_segment([p00, p01], (1.0, darkened_color)));
            }
        }
        
        // Request repaint for animation
        ui.ctx().request_repaint();
    }
}

