//! Shared egui UI for RSpeek (desktop and web WASM).
//!
//! Pure presentation layer: NO networking code is allowed here.
//! Reads and mutates [`ClientState`].

use egui::{Color32, Panel, RichText, Ui};
use rspeek_client_core::ClientState;
use rspeek_shared::feature::Feature;

pub struct RSpeekUi {
    pub state: ClientState,
    pub message_draft: String,
}

impl Default for RSpeekUi {
    fn default() -> Self {
        Self {
            state: ClientState::new(),
            message_draft: String::new(),
        }
    }
}

impl RSpeekUi {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn update(&mut self, ui: &mut Ui) {
        self.render_top_bar(ui);
        self.render_server_list(ui);
        self.render_channel_list(ui);
        self.render_main_area(ui);
    }

    fn render_top_bar(&mut self, ui: &mut Ui) {
        Panel::top("top_bar").show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.heading(
                    RichText::new("RSpeek")
                        .color(Color32::from_rgb(88, 101, 242))
                        .strong(),
                );
                ui.separator();

                let status_text = match &self.state.connection_status {
                    rspeek_client_core::ConnectionStatus::Disconnected => {
                        RichText::new("Offline").color(Color32::from_rgb(237, 66, 69))
                    }
                    rspeek_client_core::ConnectionStatus::Connecting { endpoint } => {
                        RichText::new(format!("Connecting to {endpoint}..."))
                            .color(Color32::from_rgb(254, 231, 92))
                    }
                    rspeek_client_core::ConnectionStatus::Connected { endpoint } => {
                        RichText::new(format!("Connected ({endpoint})"))
                            .color(Color32::from_rgb(87, 242, 135))
                    }
                    rspeek_client_core::ConnectionStatus::Reconnecting { attempt } => {
                        RichText::new(format!("Reconnecting (attempt {attempt})..."))
                            .color(Color32::from_rgb(254, 231, 92))
                    }
                };
                ui.label(status_text);

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if let Some(ref session) = self.state.session {
                        ui.label(RichText::new(&session.username).strong());
                        for feature in &session.active_features {
                            let badge = match feature {
                                Feature::Video1080p60 => "1080p60",
                                Feature::ScreenShare1080p60 => "Screen 60fps",
                                Feature::Stickers => "Stickers",
                                Feature::CustomEmojiAnywhere => "Emojis",
                                Feature::LargerUploads => "Uploads+",
                                Feature::AnimatedAvatar => "Gif Avatar",
                                Feature::ProfileBanner => "Banner",
                                Feature::HighBitrateAudio => "HD Audio",
                            };
                            ui.colored_label(Color32::from_rgb(235, 69, 158), format!("[{badge}]"));
                        }
                    } else {
                        ui.label("Guest / Not logged in");
                    }
                });
            });
        });
    }

    fn render_server_list(&mut self, ui: &mut Ui) {
        Panel::left("servers_panel")
            .resizable(false)
            .exact_size(60.0)
            .show(ui, |ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(8.0);
                    if ui.button(RichText::new("🏠").size(24.0)).clicked() {
                        self.state.selected_server = None;
                    }
                    ui.separator();

                    for server in &self.state.servers {
                        let selected = self.state.selected_server == Some(server.id);
                        let initial = server.name.chars().next().unwrap_or('S').to_string();
                        let text = if selected {
                            RichText::new(&initial)
                                .size(20.0)
                                .color(Color32::WHITE)
                                .strong()
                        } else {
                            RichText::new(&initial).size(18.0)
                        };
                        if ui.button(text).clicked() {
                            self.state.selected_server = Some(server.id);
                        }
                    }
                });
            });
    }

    fn render_channel_list(&mut self, ui: &mut Ui) {
        Panel::left("channels_panel")
            .resizable(true)
            .default_size(180.0)
            .show(ui, |ui| {
                ui.heading("Channels");
                ui.separator();

                let channels = self
                    .state
                    .servers
                    .iter()
                    .find(|s| Some(s.id) == self.state.selected_server)
                    .map(|s| s.channels.as_slice())
                    .unwrap_or(&[]);

                if channels.is_empty() {
                    ui.label("No server selected");
                } else {
                    for channel in channels {
                        let icon = if channel.is_voice { "🔊" } else { "#" };
                        let label = format!("{icon} {}", channel.name);
                        let selected = self.state.selected_channel == Some(channel.id);
                        if ui.selectable_label(selected, label).clicked() {
                            self.state.selected_channel = Some(channel.id);
                        }
                    }
                }
            });
    }

    fn render_main_area(&mut self, ui: &mut Ui) {
        egui::CentralPanel::default().show(ui, |ui| {
            ui.vertical(|ui| {
                // Messages backlog area
                egui::ScrollArea::vertical()
                    .auto_shrink([false; 2])
                    .max_height(ui.available_height() - 40.0)
                    .show(ui, |ui| {
                        if self.state.messages.is_empty() {
                            ui.label("No messages yet. Send a message to start chatting!");
                        } else {
                            for msg in &self.state.messages {
                                ui.horizontal(|ui| {
                                    ui.label(
                                        RichText::new(&msg.author_name)
                                            .color(Color32::from_rgb(88, 101, 242))
                                            .strong(),
                                    );
                                    ui.label(&msg.content);
                                });
                            }
                        }
                    });

                ui.separator();

                // Chat input row
                ui.horizontal(|ui| {
                    let response = ui.text_edit_singleline(&mut self.message_draft);
                    let enter_pressed =
                        response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter));
                    if (ui.button("Send").clicked() || enter_pressed)
                        && !self.message_draft.trim().is_empty()
                    {
                        let text = self.message_draft.trim().to_string();
                        self.state.messages.push(rspeek_client_core::ChatMessage {
                            id: self.state.messages.len() as u64 + 1,
                            author_name: self
                                .state
                                .session
                                .as_ref()
                                .map(|s| s.username.clone())
                                .unwrap_or_else(|| "You".to_string()),
                            content: text,
                            timestamp_ms: 0,
                        });
                        self.message_draft.clear();
                        response.request_focus();
                    }
                });
            });
        });
    }
}
