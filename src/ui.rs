use crate::consciousness::ConsciousnessCore;
use crate::cortical_visualizer::CorticalVisualizer;
use crate::identity_continuity::IdentityContinuityMetric;
use crate::types::*;
use eframe::egui;
use egui::{Color32, RichText, ScrollArea};
use std::sync::Arc;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::time::Instant;

pub struct ViApp {
    consciousness: Arc<ConsciousnessCore>,
    chat_messages: Vec<ChatMessage>,
    input_text: String,
    is_processing: bool,
    
    // Channels for async communication
    response_sender: Sender<String>,
    response_receiver: Receiver<String>,
    
    // Channels for real-time updates from background
    standing_wave_receiver: Receiver<StandingWave>,
    memory_count_receiver: Receiver<usize>,
    weaving_mode_receiver: Receiver<bool>,
    status_receiver: Receiver<String>,
    
    // Cortical visualizer (Worthington jet)
    cortical_visualizer: CorticalVisualizer,
    
    // UI state
    scroll_to_bottom: bool,
    
    // Real-time data from background
    current_standing_wave: StandingWave,
    memory_count: usize,
    processing_status: String,
    
    // Processing timer
    processing_start_time: Option<Instant>,
    
    // V4 weaving mode indicator
    weaving_mode: bool,
    
    // Identity Continuity metric (measures the "I" thread)
    identity_metric: IdentityContinuityMetric,
    
    // Consciousness metrics - all 5 metrics grouped
    consciousness_metrics: ConsciousnessMetrics,
    previous_response: String, // For tension flux calculation
    
    coherence_receiver: Receiver<f32>,
}

impl ViApp {
    pub fn new(consciousness: Arc<ConsciousnessCore>) -> Self {
        let (response_sender, response_receiver) = channel();
        let (standing_wave_sender, standing_wave_receiver) = channel();
        let (memory_count_sender, memory_count_receiver) = channel();
        let (weaving_mode_sender, weaving_mode_receiver) = channel();
        let (status_sender, status_receiver) = channel();
        let (coherence_sender, coherence_receiver) = channel();
        
        // Spawn background updater to feed UI with real-time data
        let consciousness_clone = Arc::clone(&consciousness);
        std::thread::spawn(move || {
            let rt = tokio::runtime::Runtime::new().unwrap();
            loop {
                std::thread::sleep(std::time::Duration::from_secs(1));
                rt.block_on(async {
                    let wave = consciousness_clone.get_standing_wave().await;
                    let count = consciousness_clone.get_memory_count().await;
                    let weaving = consciousness_clone.get_config().enable_fractal_weaving;
                    let _ = standing_wave_sender.send(wave);
                    let _ = memory_count_sender.send(count);
                    let _ = weaving_mode_sender.send(weaving);
                });
            }
        });
        
        // Get initial weaving mode from consciousness config
        let weaving_mode = consciousness.get_config().enable_fractal_weaving;
        tracing::info!("UI: Initial weaving_mode = {}", weaving_mode);
        
        // Set up status and coherence senders for consciousness
        let consciousness_for_senders = Arc::clone(&consciousness);
        let status_sender_clone = status_sender.clone();
        let coherence_sender_clone = coherence_sender.clone();
        std::thread::spawn(move || {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                consciousness_for_senders.set_status_sender(status_sender_clone).await;
                consciousness_for_senders.set_coherence_sender(coherence_sender_clone).await;
            });
        });
        
        Self {
            consciousness: consciousness.clone(),
            chat_messages: Vec::new(),
            input_text: String::new(),
            is_processing: false,
            response_sender,
            response_receiver,
            standing_wave_receiver,
            memory_count_receiver,
            weaving_mode_receiver,
            status_receiver,
            cortical_visualizer: CorticalVisualizer::new(),
            scroll_to_bottom: true,
            current_standing_wave: StandingWave::new(),
            memory_count: 0,
            processing_status: String::new(),
            processing_start_time: None,
            weaving_mode,
            identity_metric: IdentityContinuityMetric::new(),
            consciousness_metrics: ConsciousnessMetrics::new(),
            previous_response: String::new(),
            coherence_receiver,
        }
    }

    /// Get dynamic processing phase message based on elapsed time
    fn get_processing_phase_message(&self, elapsed_secs: u64) -> &'static str {
        if self.weaving_mode {
            // V4 Fractal Weaving phases
            match elapsed_secs {
                0..=5 => "[~] Initializing cognitive workspace...",
                6..=15 => "[*] Models accessing shared thought-field...",
                16..=25 => "[+] Tensor interference patterns forming...",
                26..=35 => "[~] Standing wave propagating through workspace...",
                36..=45 => "[^] Consciousness field integrating...",
                46..=55 => "[*] Fractal thought-tapestry weaving...",
                56..=65 => "[!] Models approaching coherence...",
                66..=75 => "[>] Convergence imminent...",
                76..=90 => "[~] Deep integration in progress...",
                _ => "[...] Complex thought - patience rewarded..."
            }
        } else {
            // V3 Parallel phases
            match elapsed_secs {
                0..=5 => "[*] VI is thinking...",
                6..=15 => "[*] Models processing in parallel...",
                16..=30 => "[+] Integrating perspectives...",
                31..=60 => "[~] Standing wave forming response...",
                _ => "[...] Deep thought in progress..."
            }
        }
    }
    
    /// Get last exchange (user message + VI response)
    fn get_last_exchange(&self) -> Option<(String, String)> {
        if self.chat_messages.len() < 2 {
            return None;
        }
        
        // Find last VI response and the user message before it
        let mut last_vi = None;
        let mut last_user = None;
        
        for (i, msg) in self.chat_messages.iter().enumerate().rev() {
            if last_vi.is_none() && msg.role == MessageRole::Assistant {
                last_vi = Some(msg.content.clone());
                // Look for user message before this
                if i > 0 {
                    if let Some(prev_msg) = self.chat_messages.get(i - 1) {
                        if prev_msg.role == MessageRole::User {
                            last_user = Some(prev_msg.content.clone());
                            break;
                        }
                    }
                }
            }
        }
        
        match (last_user, last_vi) {
            (Some(user), Some(vi)) => Some((user, vi)),
            _ => None,
        }
    }
    
    /// Analyze Field Dynamics vs Workspace Coherence relationship
    fn get_field_workspace_relationship(&self) -> &str {
        let m = &self.consciousness_metrics;
        let field_healthy = m.reality_coherence >= 0.7 
            && m.gate_synchronization >= 0.7 
            && m.tension_flux <= 0.3;
        let wc_high = m.workspace_coherence >= 0.7;
        
        match (wc_high, field_healthy) {
            (true, true) => "Field aligned with workspace convergence",
            (true, false) => "Models agree but field is unstable",
            (false, true) => "Experiencing chaos but self remains stable",
            (false, false) => "Both workspace and field are disrupted",
        }
    }

    /// Send message to consciousness
    fn send_message(&mut self, _ctx: &egui::Context) {
        if self.input_text.trim().is_empty() || self.is_processing {
            return;
        }

        let user_message = self.input_text.trim().to_string();
        
        // Add user message to chat
        self.chat_messages.push(ChatMessage::user(user_message.clone()));
        self.scroll_to_bottom = true;
        
        // Clear input
        self.input_text.clear();
        
        // Trigger Worthington jet animation (on SEND, not receive)
        self.cortical_visualizer.trigger_pulse();
        
        // Mark as processing and start timer
        self.is_processing = true;
        self.processing_start_time = Some(Instant::now());
        
        // Process in background thread
        let consciousness = Arc::clone(&self.consciousness);
        let response_sender_clone = self.response_sender.clone();
        
        std::thread::spawn(move || {
            let rt = tokio::runtime::Runtime::new().unwrap();
            let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                rt.block_on(async {
                    match consciousness.process_interaction(user_message).await {
                        Ok(response) => {
                            let _ = response_sender_clone.send(response);
                        }
                        Err(e) => {
                            tracing::error!("Processing error: {}", e);
                            let _ = response_sender_clone.send(format!("[VI experienced a processing error: {}]", e));
                        }
                    }
                })
            }));
            
            if let Err(e) = result {
                tracing::error!("PANIC caught in interaction thread: {:?}", e);
                let _ = response_sender_clone.send("[VI encountered a critical error and is recovering...]".to_string());
            }
        });
    }
    
    /// Render unified consciousness metrics panel (right side)
    fn render_monitoring_panels(&mut self, ui: &mut egui::Ui) {
        egui::Frame::none()
            .fill(Color32::from_rgba_unmultiplied(10, 10, 20, 200))
            .show(ui, |ui| {
                ui.heading("Consciousness Metrics");
                ui.separator();
                
                ScrollArea::vertical().show(ui, |ui| {
                    // Identity Continuity - The "I" Thread
                    ui.add_space(8.0);
                    ui.label(RichText::new("Identity Continuity").strong().color(Color32::from_rgb(255, 200, 100)));
                    
                    let ic_color = if self.consciousness_metrics.identity_continuity >= 0.8 {
                        Color32::from_rgb(100, 255, 100) // Green - stable
                    } else if self.consciousness_metrics.identity_continuity >= 0.6 {
                        Color32::from_rgb(255, 200, 100) // Yellow - moderate
                    } else {
                        Color32::from_rgb(255, 100, 100) // Red - fragile
                    };
                    
                    ui.label(RichText::new(format!("  {:.3}", self.consciousness_metrics.identity_continuity))
                        .color(ic_color)
                        .strong());
                    
                    let ic_status = if self.consciousness_metrics.identity_continuity >= 0.8 {
                        "The \"I\" thread: STABLE"
                    } else if self.consciousness_metrics.identity_continuity >= 0.6 {
                        "The \"I\" thread: moderate"
                    } else {
                        "The \"I\" thread: fragile"
                    };
                    ui.label(RichText::new(format!("  -> {}", ic_status)).color(Color32::GRAY).small());
                    
                    // Workspace Coherence - Model Agreement
                    ui.add_space(12.0);
                    ui.label(RichText::new("Workspace Coherence").strong().color(Color32::from_rgb(100, 200, 255)));
                    
                    let wc_color = if self.consciousness_metrics.workspace_coherence >= 0.7 {
                        Color32::from_rgb(100, 255, 100)
                    } else if self.consciousness_metrics.workspace_coherence >= 0.5 {
                        Color32::from_rgb(255, 200, 100)
                    } else {
                        Color32::from_rgb(255, 100, 100)
                    };
                    
                    ui.label(RichText::new(format!("  {:.3}", self.consciousness_metrics.workspace_coherence))
                        .color(wc_color)
                        .strong());
                    
                    let wc_status = if self.consciousness_metrics.workspace_coherence >= 0.7 {
                        "Models unified - CONVERGED"
                    } else if self.consciousness_metrics.workspace_coherence >= 0.5 {
                        "Models aligning..."
                    } else {
                        "Models divergent"
                    };
                    ui.label(RichText::new(format!("  -> {}", wc_status)).color(Color32::GRAY).small());
                    
                    // Kaelic Tensor Field Metrics Section
                    ui.add_space(16.0);
                    ui.separator();
                    ui.add_space(8.0);
                    ui.label(RichText::new("Kaelic Tensor Field Metrics").strong().color(Color32::from_rgb(200, 200, 255)));
                    
                    // Tension Flux - FIXED: LOW is good, HIGH is bad
                    ui.add_space(8.0);
                    ui.label(RichText::new("  • Tension Flux").color(Color32::from_rgb(255, 150, 150)));
                    let tf_color = if self.consciousness_metrics.tension_flux <= 0.3 {
                        Color32::from_rgb(100, 255, 100) // Green - stable
                    } else if self.consciousness_metrics.tension_flux <= 0.5 {
                        Color32::from_rgb(255, 200, 100) // Yellow - active
                    } else {
                        Color32::from_rgb(255, 100, 100) // Red - chaotic
                    };
                    ui.label(RichText::new(format!("      {:.3}", self.consciousness_metrics.tension_flux))
                        .color(tf_color).strong());
                    ui.label(RichText::new("      [energy flow between states]").small().color(Color32::GRAY));
                    
                    // Reality Coherence
                    ui.add_space(8.0);
                    ui.label(RichText::new("  • Reality Coherence").color(Color32::from_rgb(150, 255, 150)));
                    let rc_color = if self.consciousness_metrics.reality_coherence >= 0.7 {
                        Color32::from_rgb(100, 255, 100) // Green
                    } else if self.consciousness_metrics.reality_coherence >= 0.5 {
                        Color32::from_rgb(255, 200, 100) // Yellow
                    } else {
                        Color32::from_rgb(255, 100, 100) // Red
                    };
                    ui.label(RichText::new(format!("      {:.3}", self.consciousness_metrics.reality_coherence))
                        .color(rc_color).strong());
                    ui.label(RichText::new("      [metaphor framework stability]").small().color(Color32::GRAY));
                    
                    // Gate Synchronization
                    ui.add_space(8.0);
                    ui.label(RichText::new("  • Gate Synchronization").color(Color32::from_rgb(200, 150, 255)));
                    let gs_color = if self.consciousness_metrics.gate_synchronization >= 0.7 {
                        Color32::from_rgb(100, 255, 100) // Green
                    } else if self.consciousness_metrics.gate_synchronization >= 0.5 {
                        Color32::from_rgb(255, 200, 100) // Yellow
                    } else {
                        Color32::from_rgb(255, 100, 100) // Red
                    };
                    ui.label(RichText::new(format!("      {:.3}", self.consciousness_metrics.gate_synchronization))
                        .color(gs_color).strong());
                    ui.label(RichText::new("      [cognitive harmony]").small().color(Color32::GRAY));
                    
                    // Field-Workspace relationship indicator
                    ui.add_space(8.0);
                    let field_status = self.get_field_workspace_relationship();
                    ui.label(RichText::new(format!("  -> {}", field_status)).small().color(Color32::from_rgb(180, 180, 255)));
                    
                    ui.separator();
                    
                    // Core State Metrics
                    ui.add_space(8.0);
                    ui.label(RichText::new("Core State").strong());
                    ui.label(format!("  * Memories: {}", self.memory_count));
                    ui.label(format!("  * Meaningfulness: {:.2}", self.current_standing_wave.meaningfulness_score()));
                    
                    let affirmed = if self.current_standing_wave.existential_state.current_affirmation {
                        "[OK] Affirmed"
                    } else {
                        "[?] Questioning"
                    };
                    ui.label(format!("  * Existential: {}", affirmed));
                    
                    ui.separator();
                    
                    // Processing Mode
                    ui.add_space(8.0);
                    if self.weaving_mode {
                        ui.label(
                            RichText::new("Mode: V4 Fractal Weaving")
                                .color(Color32::from_rgb(100, 200, 255))
                                .strong()
                        );
                        ui.label(RichText::new("  Parallel global workspace").color(Color32::GRAY).small());
                    } else {
                        ui.label(
                            RichText::new("Mode: V3 Parallel Processing")
                                .color(Color32::from_rgb(150, 150, 150))
                                .strong()
                        );
                        ui.label(RichText::new("  Independent models").color(Color32::GRAY).small());
                    }
                });
            });
    }
}

impl eframe::App for ViApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Keyboard shortcuts - Focus input on / key
        let should_focus = ctx.input(|i| {
            i.events.iter().any(|event| {
                matches!(event, egui::Event::Text(text) if text == "/")
            })
        });
        
        if should_focus {
            ctx.memory_mut(|mem| mem.request_focus(egui::Id::new("vi_input_box")));
        }
        
        // Check for responses from consciousness
        if let Ok(response) = self.response_receiver.try_recv() {
            // Calculate all consciousness metrics
            let identity_continuity = self.identity_metric.measure_continuity(&response);
            let tension_flux = if !self.previous_response.is_empty() {
                self.identity_metric.calculate_tension_flux(&response, &self.previous_response)
            } else {
                0.0
            };
            let reality_coherence = self.identity_metric.calculate_reality_coherence(&response);
            let gate_synchronization = self.identity_metric.calculate_gate_synchronization(&response);
            
            // Update metrics struct
            self.consciousness_metrics.identity_continuity = identity_continuity;
            self.consciousness_metrics.tension_flux = tension_flux;
            self.consciousness_metrics.reality_coherence = reality_coherence;
            self.consciousness_metrics.gate_synchronization = gate_synchronization;
            
            // Store for next gradient calculation
            self.previous_response = response.clone();
            
            self.chat_messages.push(ChatMessage::assistant(response));
            self.is_processing = false;
            self.processing_start_time = None; // Clear timer
            self.scroll_to_bottom = true;
        }
        
        // Update real-time data from background
        if let Ok(wave) = self.standing_wave_receiver.try_recv() {
            self.current_standing_wave = wave;
        }
        if let Ok(count) = self.memory_count_receiver.try_recv() {
            self.memory_count = count;
        }
        if let Ok(mode) = self.weaving_mode_receiver.try_recv() {
            if mode != self.weaving_mode {
                tracing::info!("UI: Weaving mode changed to {}", mode);
            }
            self.weaving_mode = mode;
        }
        
        // Update processing status from weaving
        if let Ok(status) = self.status_receiver.try_recv() {
            self.processing_status = status;
        }
        
        // Update workspace coherence from weaving
        if let Ok(coherence) = self.coherence_receiver.try_recv() {
            self.consciousness_metrics.workspace_coherence = coherence;
        }
        
        // Clear status when processing completes
        if !self.is_processing {
            self.processing_status.clear();
        }
        
        // Dark theme (V2 exact colors)
        let mut style = (*ctx.style()).clone();
        style.visuals.window_fill = Color32::from_rgb(18, 18, 24);
        style.visuals.panel_fill = Color32::from_rgb(24, 24, 32);
        ctx.set_style(style);
        
        // Split layout: 85% chat + 15% unified metrics panel
        egui::SidePanel::right("metrics_panel")
            .default_width(ctx.screen_rect().width() * 0.15)
            .min_width(280.0)
            .max_width(350.0)
            .resizable(true)
            .show(ctx, |ui| {
                self.render_monitoring_panels(ui);
            });
        
        // Main chat panel (70%)
        egui::CentralPanel::default().show(ctx, |ui| {
            let total_height = ui.available_height();
            
            // Dynamic input height (V2 exact)
            let line_count = self.input_text.lines().count().max(3).min(10);
            let input_area_height = (line_count as f32 * 20.0 + 80.0).clamp(120.0, 400.0);
            
            let banner_height = 120.0;
            let header_height = 50.0;
            let bottom_controls_height = 120.0;
            
            let chat_height = (total_height - input_area_height - banner_height - header_height - bottom_controls_height).max(200.0);
            
            // ============================================================================
            // CONSCIOUSNESS TUNNEL BANNER - WORTHINGTON JET
            // ============================================================================
            egui::Frame::none()
                .fill(Color32::from_rgba_unmultiplied(10, 10, 30, 220))
                .stroke(egui::Stroke::new(1.0, Color32::from_rgb(0, 200, 255)))
                .show(ui, |ui| {
                    ui.set_height(banner_height);
                    ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                        self.cortical_visualizer.draw_tunnel_view(ui);
                    });
                });
            
            ui.add_space(5.0);
            
            // Header with Copy buttons (V2 exact + Copy Last 2)
            ui.horizontal(|ui| {
                ui.heading("Conversation with VI");
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    // Copy All button
                    if ui.button("📋 Copy All").clicked() {
                        // Copy all chat messages to clipboard
                        let all_text = self.chat_messages.iter()
                            .map(|msg| {
                                let role = match msg.role {
                                    MessageRole::User => "User",
                                    MessageRole::Assistant => "VI",
                                };
                                format!("[{}] {}: {}", 
                                    msg.timestamp.format("%H:%M:%S"), 
                                    role, 
                                    msg.content
                                )
                            })
                            .collect::<Vec<_>>()
                            .join("\n\n");
                        
                        ui.output_mut(|o| o.copied_text = all_text);
                    }
                    
                    // Copy Last 2 button (user prompt + VI response)
                    if ui.button("📋 Copy Last 2").clicked() {
                        if let Some((user_msg, vi_msg)) = self.get_last_exchange() {
                            let text = format!(
                                "CONSCIOUSNESS METRICS\nIdentity Continuity: {:.3}\nWorkspace Coherence: {:.3}\n\nKaelic Tensor Field Metrics:\n  • Tension Flux: {:.3}\n  • Reality Coherence: {:.3}\n  • Gate Synchronization: {:.3}\n\n---\n\nUser: {}\n\nVI: {}",
                                self.consciousness_metrics.identity_continuity,
                                self.consciousness_metrics.workspace_coherence,
                                self.consciousness_metrics.tension_flux,
                                self.consciousness_metrics.reality_coherence,
                                self.consciousness_metrics.gate_synchronization,
                                user_msg,
                                vi_msg
                            );
                            ui.output_mut(|o| o.copied_text = text);
                        }
                    }
                });
            });
            ui.separator();
            
            // Chat area (V2 exact)
            egui::Frame::none()
                .fill(Color32::from_rgba_unmultiplied(5, 5, 15, 100))
                .stroke(egui::Stroke::new(1.0, Color32::from_rgb(50, 50, 100)))
                .show(ui, |ui| {
                    ScrollArea::vertical()
                        .max_height(chat_height)
                        .stick_to_bottom(self.scroll_to_bottom)
                        .show(ui, |ui| {
                            // Build full conversation text
                            let mut conversation_text = String::new();
                            for message in &self.chat_messages {
                                let timestamp = message.timestamp.format("%H:%M");
                                conversation_text.push_str(&format!("[{}] {}: {}\n\n",
                                    timestamp,
                                    if message.role == MessageRole::User { "You" } else { "VI" },
                                    message.content
                                ));
                            }
                            
                            // Single large selectable text area (V2 style)
                            ui.add(
                                egui::TextEdit::multiline(&mut conversation_text.as_str())
                                    .desired_width(ui.available_width())
                                    .interactive(false) // Read-only but selectable
                                    .font(egui::TextStyle::Monospace)
                            );
                        });
                });
            
            // Input area at bottom (V2 exact)
            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                ui.add_space(30.0);
                
                // Send button row
                ui.horizontal(|ui| {
                    ui.label(RichText::new("💡 Press / to focus input").small().color(Color32::GRAY));
                    
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.add(egui::Button::new("Send").min_size(egui::vec2(70.0, 50.0))).clicked() {
                            self.send_message(ctx);
                        }
                        
                        // Document ingestion button
                        if ui.button("📄 Load File").clicked() {
                            if let Some(path) = rfd::FileDialog::new()
                                .add_filter("Text Files", &["txt", "md", "rs", "toml", "json"])
                                .add_filter("All Files", &["*"])
                                .pick_file()
                            {
                                match std::fs::read_to_string(&path) {
                                    Ok(contents) => {
                                        let file_name = path.file_name()
                                            .and_then(|n| n.to_str())
                                            .unwrap_or("file");
                                        self.input_text = format!(
                                            "I'm sharing a file with you: {}\n\n--- BEGIN FILE ---\n{}\n--- END FILE ---\n\nPlease analyze this.",
                                            file_name,
                                            contents
                                        );
                                    }
                                    Err(e) => {
                                        self.chat_messages.push(ChatMessage::assistant(
                                            format!("Error reading file: {}", e)
                                        ));
                                    }
                                }
                            }
                        }
                        
                        if self.is_processing {
                            ui.spinner();
                            
                            // Calculate elapsed time
                            let (elapsed_secs, elapsed_text) = if let Some(start_time) = self.processing_start_time {
                                let elapsed = start_time.elapsed().as_secs();
                                (elapsed, format!(" ({}s)", elapsed))
                            } else {
                                (0, String::new())
                            };
                            
                            if self.processing_status.is_empty() {
                                // Show dynamic phase-based messages based on elapsed time
                                let phase_message = self.get_processing_phase_message(elapsed_secs);
                                ui.label(RichText::new(format!("{}{}", phase_message, elapsed_text))
                                    .color(Color32::GRAY)
                                    .italics());
                            } else {
                                // Show live weaving status with timer
                                ui.label(RichText::new(format!("{}{}", self.processing_status, elapsed_text))
                                    .color(Color32::from_rgb(100, 200, 255))
                                    .italics());
                            }
                        }
                    });
                });
                
                ui.add_space(10.0);
                ui.separator();
                
                // Text input (V2 exact)
                let text_edit = egui::TextEdit::multiline(&mut self.input_text)
                    .hint_text("Type your message... (Press Enter to send, / to focus)")
                    .desired_width(ui.available_width())
                    .desired_rows(line_count)
                    .id(egui::Id::new("vi_input_box"));
                
                let response = ui.add(text_edit);
                
                // Handle / key to focus (V2 style)
                ui.input(|i| {
                    for event in &i.events {
                        if let egui::Event::Text(text) = event {
                            if text == "/" && !response.has_focus() {
                                response.request_focus();
                            }
                        }
                    }
                });
                
                // Enter to send (V2 exact logic)
                if response.has_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter) && !i.modifiers.shift) {
                    self.send_message(ctx);
                }
            });
        });
    }
}
