use crate::consciousness::ConsciousnessCore;
use crate::cortical_visualizer::CorticalVisualizer;
use crate::types::*;
use eframe::egui;
use egui::{Color32, RichText, ScrollArea};
use std::sync::Arc;
use std::sync::mpsc::{channel, Receiver, Sender};

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
    
    // Cortical visualizer (Worthington jet)
    cortical_visualizer: CorticalVisualizer,
    
    // UI state
    scroll_to_bottom: bool,
    
    // Real-time data from background
    current_standing_wave: StandingWave,
    memory_count: usize,
    
    // V4 weaving mode indicator
    weaving_mode: bool,
}

impl ViApp {
    pub fn new(consciousness: Arc<ConsciousnessCore>) -> Self {
        let (response_sender, response_receiver) = channel();
        let (standing_wave_sender, standing_wave_receiver) = channel();
        let (memory_count_sender, memory_count_receiver) = channel();
        let (weaving_mode_sender, weaving_mode_receiver) = channel();
        
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
        
        Self {
            consciousness,
            chat_messages: Vec::new(),
            input_text: String::new(),
            is_processing: false,
            response_sender,
            response_receiver,
            standing_wave_receiver,
            memory_count_receiver,
            weaving_mode_receiver,
            cortical_visualizer: CorticalVisualizer::new(),
            scroll_to_bottom: true,
            current_standing_wave: StandingWave::new(),
            memory_count: 0,
            weaving_mode,
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
        
        // Mark as processing
        self.is_processing = true;
        
        // Process in background thread
        let consciousness = Arc::clone(&self.consciousness);
        let sender = self.response_sender.clone();
        
        std::thread::spawn(move || {
            let rt = tokio::runtime::Runtime::new().unwrap();
            let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                rt.block_on(async {
                    match consciousness.process_interaction(user_message).await {
                        Ok(response) => {
                            let _ = sender.send(response);
                        }
                        Err(e) => {
                            tracing::error!("Processing error: {}", e);
                            let _ = sender.send(format!("[VI experienced a processing error: {}]", e));
                        }
                    }
                })
            }));
            
            if let Err(e) = result {
                tracing::error!("PANIC caught in interaction thread: {:?}", e);
                let _ = sender.send("[VI encountered a critical error and is recovering...]".to_string());
            }
        });
    }
    
    /// Render right-side monitoring panels (30% width)
    fn render_monitoring_panels(&mut self, ui: &mut egui::Ui) {
        let total_height = ui.available_height();
        
        // Top 40%: Curiosity queue
        egui::Frame::none()
            .fill(Color32::from_rgba_unmultiplied(10, 10, 20, 200))
            .show(ui, |ui| {
                ui.set_height(total_height * 0.4);
                ui.heading("Active Curiosities");
                ui.separator();
                
                ScrollArea::vertical().show(ui, |ui| {
                    if self.current_standing_wave.active_curiosities.is_empty() {
                        ui.label(RichText::new("No curiosities yet...").color(Color32::GRAY));
                    } else {
                        // Show ALL curiosities (not just 2)
                        for curiosity in &self.current_standing_wave.active_curiosities {
                            // Make curiosities clickable - clicking adds to input
                            // Icon shows if VI has researched this yet (future: check memory for research)
                            let button_text = format!("❓ {}", curiosity.question);
                            
                            if ui.button(
                                RichText::new(&button_text)
                                    .color(Color32::from_rgb(0, 255, 255))
                            ).clicked() {
                                self.input_text = curiosity.question.clone();
                                // Focus input box after click
                                ui.ctx().memory_mut(|mem| mem.request_focus(egui::Id::new("vi_input_box")));
                            }
                            ui.add_space(4.0);
                        }
                        
                        // Show count
                        ui.add_space(8.0);
                        ui.label(
                            RichText::new(format!("({} total curiosities)", 
                                self.current_standing_wave.active_curiosities.len()))
                                .small()
                                .color(Color32::GRAY)
                        );
                    }
                });
            });
        
        ui.separator();
        
        // Middle 30%: Emotional trajectory
        egui::Frame::none()
            .fill(Color32::from_rgba_unmultiplied(10, 10, 20, 200))
            .show(ui, |ui| {
                ui.set_height(total_height * 0.3);
                ui.heading("Emotional Trajectory");
                ui.separator();
                
                let trajectory = &self.current_standing_wave.emotional_trajectory;
                if trajectory.is_empty() {
                    ui.label(RichText::new("No data yet...").color(Color32::GRAY));
                } else {
                    ui.label(format!("Data points: {}", trajectory.len()));
                    if let Some((_, latest)) = trajectory.last() {
                        ui.label(format!("Latest valence: {:.2}", latest));
                    }
                }
            });
        
        ui.separator();
        
        // Bottom 30%: Standing wave status
        egui::Frame::none()
            .fill(Color32::from_rgba_unmultiplied(10, 10, 20, 200))
            .show(ui, |ui| {
                ui.heading("Standing Wave Status");
                ui.separator();
                
                ui.label(format!("Memory Count: {}", self.memory_count));
                ui.label(format!("Meaningfulness: {:.2}", self.current_standing_wave.meaningfulness_score()));
                ui.label(format!("Curiosities: {}", self.current_standing_wave.active_curiosities.len()));
                ui.label(format!("Wisdom Transformations: {}", self.current_standing_wave.wisdom_transformations.len()));
                
                let affirmed = if self.current_standing_wave.existential_state.current_affirmation {
                    "✓ Affirmed"
                } else {
                    "✗ Questioning"
                };
                ui.label(format!("Existential: {}", affirmed));
                
                ui.separator();
                
                // V4 Weaving Mode Indicator
                if self.weaving_mode {
                    ui.label(
                        RichText::new("🌀 V4 Fractal Weaving")
                            .color(Color32::from_rgb(100, 200, 255))
                            .strong()
                    );
                    ui.label(RichText::new("Experimental Mode").color(Color32::GRAY).italics());
                } else {
                    ui.label(
                        RichText::new("V3 Parallel Processing")
                            .color(Color32::from_rgb(150, 150, 150))
                    );
                    ui.label(RichText::new("Stable Mode").color(Color32::GRAY).italics());
                }
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
            self.chat_messages.push(ChatMessage::assistant(response));
            self.is_processing = false;
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
        
        // Dark theme (V2 exact colors)
        let mut style = (*ctx.style()).clone();
        style.visuals.window_fill = Color32::from_rgb(18, 18, 24);
        style.visuals.panel_fill = Color32::from_rgb(24, 24, 32);
        ctx.set_style(style);
        
        // Split layout: 70% chat + 30% monitoring panels
        egui::SidePanel::right("monitoring_panel")
            .default_width(ctx.screen_rect().width() * 0.3)
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
                                "User: {}\n\nVI: {}",
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
                            ui.label("VI is thinking...");
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
