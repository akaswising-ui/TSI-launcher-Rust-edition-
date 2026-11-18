use crate::GpuRuntimeState;
use eframe::egui::{self, Align, Color32, FontId, Layout, RichText, Stroke, Vec2};
use serde::{Deserialize, Serialize};

const BG: Color32 = Color32::from_rgb(11, 15, 22);
const PANEL: Color32 = Color32::from_rgb(18, 25, 35);
const PANEL_ALT: Color32 = Color32::from_rgb(24, 33, 45);
const EDGE: Color32 = Color32::from_rgb(42, 55, 69);
const TEXT: Color32 = Color32::from_rgb(229, 237, 243);
const MUTED: Color32 = Color32::from_rgb(132, 149, 164);
const TEAL: Color32 = Color32::from_rgb(70, 226, 199);
const BLUE: Color32 = Color32::from_rgb(92, 160, 255);
const AMBER: Color32 = Color32::from_rgb(246, 185, 88);
const ROSE: Color32 = Color32::from_rgb(246, 121, 142);

#[derive(Clone, Copy, PartialEq, Eq)]
enum Page {
    Home,
    Game,
    Versions,
    Worlds,
    Mods,
    Packs,
    Servers,
    Controls,
    Accounts,
    Settings,
    Downloads,
}

impl Page {
    const ALL: [(Self, &'static str, &'static str); 11] = [
        (Self::Home, "Home", "⌂"),
        (Self::Game, "Game", "▶"),
        (Self::Versions, "Versions", "◷"),
        (Self::Worlds, "Worlds", "▦"),
        (Self::Mods, "Native Mods", "◇"),
        (Self::Packs, "Resource Packs", "▧"),
        (Self::Servers, "Servers", "◎"),
        (Self::Controls, "Touch Controls", "✣"),
        (Self::Accounts, "Accounts", "◉"),
        (Self::Downloads, "Downloads", "⇩"),
        (Self::Settings, "Settings", "⚙"),
    ];

    fn title(self) -> &'static str {
        Self::ALL
            .iter()
            .find(|(page, _, _)| *page == self)
            .map(|(_, title, _)| *title)
            .unwrap_or("Home")
    }
}

#[derive(Clone, Serialize, Deserialize)]
struct ServerEntry {
    address: String,
}

#[derive(Clone, Serialize, Deserialize)]
struct SavedState {
    profile_name: String,
    profiles: Vec<String>,
    servers: Vec<ServerEntry>,
    mod_urls: Vec<String>,
    pack_names: Vec<String>,
    touch_scheme: String,
    button_scale: f32,
    button_opacity: f32,
    right_handed: bool,
    auto_sprint: bool,
    show_sprint: bool,
    movement_360: bool,
    vibration: bool,
    resolution_scale: f32,
    vsync: bool,
    fps_limit: u32,
}

impl Default for SavedState {
    fn default() -> Self {
        Self {
            profile_name: "Player".to_owned(),
            profiles: vec!["Player".to_owned()],
            servers: Vec::new(),
            mod_urls: Vec::new(),
            pack_names: Vec::new(),
            touch_scheme: "Joystick · Crosshair".to_owned(),
            button_scale: 1.0,
            button_opacity: 0.82,
            right_handed: true,
            auto_sprint: false,
            show_sprint: true,
            movement_360: false,
            vibration: true,
            resolution_scale: 100.0,
            vsync: true,
            fps_limit: 60,
        }
    }
}

pub struct LauncherApp {
    page: Page,
    saved: SavedState,
    draft_profile: String,
    draft_server: String,
    draft_mod_url: String,
    draft_pack: String,
    notice: String,
    gpu_runtime: GpuRuntimeState,
}

impl LauncherApp {
    pub fn new(cc: &eframe::CreationContext<'_>, gpu_runtime: GpuRuntimeState) -> Self {
        configure_theme(&cc.egui_ctx);
        let saved = cc
            .storage
            .and_then(|storage| eframe::get_value::<SavedState>(storage, "tsi_orbit_state"))
            .unwrap_or_default();

        Self {
            page: Page::Home,
            draft_profile: String::new(),
            draft_server: String::new(),
            draft_mod_url: String::new(),
            draft_pack: String::new(),
            saved,
            notice: "Rust UI initialized · game runtime is a separate integration phase".to_owned(),
            gpu_runtime,
        }
    }

    fn navigate(&mut self, page: Page) {
        self.page = page;
    }

    fn top_bar(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label(RichText::new("TSI").size(17.0).strong().color(TEAL));
            ui.label(RichText::new("/").size(15.0).color(MUTED));
            ui.label(RichText::new("ORBIT").size(17.0).strong().color(TEXT));
            ui.add_space(12.0);
            ui.label(RichText::new(self.page.title()).size(14.0).color(MUTED));
            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                status_chip(ui, "RUST NATIVE", TEAL);
                ui.add_space(8.0);
                status_chip(ui, "ANDROID · ARM64 TARGET", BLUE);
            });
        });
    }

    fn sidebar(&mut self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            ui.add_space(8.0);
            ui.horizontal(|ui| {
                brand_mark(ui);
                ui.vertical(|ui| {
                    ui.label(RichText::new("TSI ORBIT").size(16.0).strong().color(TEXT));
                    ui.label(
                        RichText::new("RUST EDITION")
                            .size(10.0)
                            .color(TEAL)
                            .strong(),
                    );
                });
            });
            ui.add_space(20.0);
            ui.label(RichText::new("WORKSPACE").size(10.0).strong().color(MUTED));
            ui.add_space(7.0);

            for (page, title, icon) in Page::ALL {
                let selected = self.page == page;
                let response = ui.add_sized(
                    [ui.available_width(), 39.0],
                    egui::Button::new(
                        RichText::new(format!("{icon}   {title}"))
                            .size(13.0)
                            .color(if selected { TEAL } else { TEXT }),
                    )
                    .fill(if selected {
                        Color32::from_rgb(24, 62, 64)
                    } else {
                        PANEL
                    })
                    .stroke(Stroke::new(1.0, if selected { TEAL } else { PANEL })),
                );
                if response.clicked() {
                    self.navigate(page);
                }
            }

            ui.add_space(18.0);
            panel(ui, |ui| {
                ui.label(
                    RichText::new("ACTIVE PROFILE")
                        .size(10.0)
                        .color(MUTED)
                        .strong(),
                );
                ui.label(
                    RichText::new(&self.saved.profile_name)
                        .size(14.0)
                        .strong()
                        .color(TEXT),
                );
                ui.label(RichText::new("Local profile").size(11.0).color(MUTED));
            });
            ui.with_layout(Layout::bottom_up(Align::Min), |ui| {
                ui.label(
                    RichText::new("Independent project · no Mojang affiliation")
                        .size(9.0)
                        .color(MUTED),
                );
                ui.label(
                    RichText::new("BUILD 0.1.0 · UI PREVIEW")
                        .size(10.0)
                        .color(TEAL),
                );
            });
        });
    }

    fn mobile_nav(&mut self, ui: &mut egui::Ui) {
        ui.horizontal_wrapped(|ui| {
            for (page, title, icon) in Page::ALL {
                let selected = self.page == page;
                if ui
                    .add(
                        egui::Button::new(
                            RichText::new(format!("{icon} {title}"))
                                .size(11.0)
                                .color(if selected { TEAL } else { TEXT }),
                        )
                        .fill(if selected {
                            Color32::from_rgb(24, 62, 64)
                        } else {
                            PANEL
                        })
                        .stroke(Stroke::new(1.0, if selected { TEAL } else { EDGE })),
                    )
                    .clicked()
                {
                    self.navigate(page);
                }
            }
        });
    }

    fn home_page(&mut self, ui: &mut egui::Ui) {
        ui.label(
            RichText::new("YOUR GAME SPACE")
                .size(11.0)
                .strong()
                .color(TEAL),
        );
        ui.heading(
            RichText::new("A cleaner way to launch.")
                .size(28.0)
                .strong()
                .color(TEXT),
        );
        ui.label(
            RichText::new(
                "Manage profiles, worlds, packs, mods and servers from one Rust-native hub.",
            )
            .size(13.0)
            .color(MUTED),
        );
        ui.add_space(12.0);

        hero_card(ui, |ui| {
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    status_chip(ui, "ENGINE NOT CONNECTED", AMBER);
                    ui.add_space(8.0);
                    ui.label(
                        RichText::new("TSI ORBIT CLIENT")
                            .size(23.0)
                            .strong()
                            .color(TEXT),
                    );
                    ui.label(
                        RichText::new(
                            "A new native Rust game runtime is planned as a separate component.",
                        )
                        .size(12.0)
                        .color(MUTED),
                    );
                    ui.add_space(12.0);
                    ui.horizontal(|ui| {
                        let response = ui.add_enabled(
                            false,
                            egui::Button::new(
                                RichText::new("▶  PLAY").strong().color(Color32::BLACK),
                            )
                            .fill(TEAL),
                        );
                        response
                            .on_hover_text("Play activates after a game runtime is integrated.");
                        if ui.button("Open Versions").clicked() {
                            self.navigate(Page::Versions);
                        }
                    });
                });
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    orbit_art(ui, Vec2::new(150.0, 130.0));
                });
            });
        });

        ui.add_space(16.0);
        ui.horizontal_wrapped(|ui| {
            metric_card(
                ui,
                "PROFILES",
                self.saved.profiles.len().to_string(),
                "Local player identities",
                TEAL,
            );
            metric_card(
                ui,
                "SERVERS",
                self.saved.servers.len().to_string(),
                "Saved addresses",
                BLUE,
            );
            metric_card(
                ui,
                "NATIVE MOD URLS",
                self.saved.mod_urls.len().to_string(),
                "Queued for integration",
                AMBER,
            );
            metric_card(
                ui,
                "RESOURCE PACKS",
                self.saved.pack_names.len().to_string(),
                "Local catalogue entries",
                ROSE,
            );
        });

        ui.add_space(12.0);
        ui.horizontal_top(|ui| {
            ui.vertical(|ui| {
                section_title(ui, "QUICK ACCESS", "Jump into a workspace");
                ui.horizontal_wrapped(|ui| {
                    quick_action(ui, "Versions", "Build channels and install state", || {
                        self.navigate(Page::Versions)
                    });
                    quick_action(ui, "Worlds", "World library and backups", || {
                        self.navigate(Page::Worlds)
                    });
                    quick_action(ui, "Mods", "Native Rust add-ons", || {
                        self.navigate(Page::Mods)
                    });
                    quick_action(ui, "Controls", "Touch layout editor", || {
                        self.navigate(Page::Controls)
                    });
                });
            });
        });

        ui.add_space(12.0);
        panel(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label(RichText::new("PROJECT STATUS").strong().color(TEXT));
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    status_chip(ui, "EARLY FOUNDATION", AMBER);
                });
            });
            ui.label(RichText::new("The launcher UI and preference model are live. Engine installation, resource verification, account sign-in and in-game rendering still need their own tested Rust modules.")
                .size(12.0).color(MUTED));
        });
    }

    fn game_page(&mut self, ui: &mut egui::Ui) {
        section_title(ui, "GAME", "Client runtime");
        panel(ui, |ui| {
            status_chip(ui, "NOT INSTALLED", AMBER);
            ui.add_space(8.0);
            ui.label(
                RichText::new("Runtime integration is not part of this UI foundation yet.")
                    .size(15.0)
                    .strong()
                    .color(TEXT),
            );
            ui.label(RichText::new("The Horizon feature model separates launcher responsibilities from the Rust game engine. TSI Orbit will keep that boundary explicit rather than showing a fake Play result.")
                .size(12.0).color(MUTED));
            ui.add_space(8.0);
            ui.label(RichText::new("Target engine line: Java Edition 26.3 compatibility (to be verified against the engine source and licence before integration).")
                .size(11.0).color(BLUE));
        });
        section_title(ui, "RUN CONFIGURATION", "Planned launch inputs");
        info_row(ui, "Profile", &self.saved.profile_name, "Local identity");
        info_row(
            ui,
            "Graphics",
            "Adapter default",
            "Renderer selection not connected",
        );
        info_row(ui, "Input", &self.saved.touch_scheme, "Saved touch preset");
        info_row(
            ui,
            "Game files",
            "Not present",
            "No copyrighted game data is bundled",
        );
    }

    fn versions_page(&mut self, ui: &mut egui::Ui) {
        section_title(ui, "VERSIONS", "Builds and runtime channels");
        panel(ui, |ui| {
            ui.label(
                RichText::new("No client build is installed yet.")
                    .size(15.0)
                    .strong()
                    .color(TEXT),
            );
            ui.label(RichText::new("This page will list only verified TSI engine builds with checksums and matching Android ABI. Minecraft Java files must come from an authorised copy/source.")
                .size(12.0).color(MUTED));
        });
        section_title(ui, "CHANNELS", "Independent update tracks");
        channel_card(
            ui,
            "STABLE",
            "Signed, tested releases",
            "Not published",
            TEAL,
        );
        channel_card(
            ui,
            "NIGHTLY",
            "Development snapshots",
            "Not published",
            BLUE,
        );
        channel_card(
            ui,
            "COMPATIBILITY",
            "Game version support",
            "Requires engine test matrix",
            AMBER,
        );
    }

    fn worlds_page(&mut self, ui: &mut egui::Ui) {
        section_title(ui, "WORLDS", "World library");
        panel(ui, |ui| {
            ui.label(
                RichText::new("No world folders are indexed.")
                    .size(15.0)
                    .strong()
                    .color(TEXT),
            );
            ui.label(RichText::new("The next storage module will use Android's document picker for user-selected import/export. It will not scan unrelated phone folders.")
                .size(12.0).color(MUTED));
            ui.add_space(8.0);
            ui.label(
                RichText::new(
                    "Backup flow planned: select world → create archive → verify → export.",
                )
                .size(11.0)
                .color(TEAL),
            );
        });
    }

    fn mods_page(&mut self, ui: &mut egui::Ui) {
        section_title(ui, "NATIVE MODS", "Rust extension packages");
        panel(ui, |ui| {
            ui.label(
                RichText::new("Add a direct HTTPS .hmod source")
                    .strong()
                    .color(TEXT),
            );
            ui.horizontal(|ui| {
                ui.add(egui::TextEdit::singleline(&mut self.draft_mod_url)
                    .hint_text("https://host.example/mod.hmod")
                    .desired_width(ui.available_width() - 92.0));
                if ui.button("Queue URL").clicked() {
                    let candidate = self.draft_mod_url.trim();
                    if candidate.starts_with("https://") && candidate.ends_with(".hmod") {
                        self.saved.mod_urls.push(candidate.to_owned());
                        self.draft_mod_url.clear();
                        self.notice = "Mod source queued locally. Signature, checksum and sandbox checks are not implemented yet.".to_owned();
                    } else {
                        self.notice = "Enter a direct HTTPS URL ending in .hmod.".to_owned();
                    }
                }
            });
            ui.label(
                RichText::new("Queueing only saves the URL. It does not download or execute code.")
                    .size(10.0)
                    .color(MUTED),
            );
        });
        section_title(ui, "QUEUED SOURCES", "Local list");
        if self.saved.mod_urls.is_empty() {
            empty_state(
                ui,
                "No native mods queued",
                "Java .jar mods are not native Rust mods and will not be shown as compatible.",
            );
        } else {
            let mut remove_index = None;
            for (index, url) in self.saved.mod_urls.iter().enumerate() {
                ui.horizontal(|ui| {
                    ui.label(RichText::new("◇").color(TEAL));
                    ui.label(RichText::new(url).color(TEXT).size(11.0));
                    if ui.small_button("Remove").clicked() {
                        remove_index = Some(index);
                    }
                });
            }
            if let Some(index) = remove_index {
                self.saved.mod_urls.remove(index);
            }
        }
    }

    fn packs_page(&mut self, ui: &mut egui::Ui) {
        section_title(ui, "RESOURCE PACKS", "Verified visual content");
        panel(ui, |ui| {
            ui.label(
                RichText::new("Create a local catalogue entry")
                    .strong()
                    .color(TEXT),
            );
            ui.horizontal(|ui| {
                ui.add(egui::TextEdit::singleline(&mut self.draft_pack)
                    .hint_text("Pack name")
                    .desired_width(ui.available_width() - 92.0));
                if ui.button("Add entry").clicked() {
                    let name = self.draft_pack.trim();
                    if !name.is_empty() {
                        self.saved.pack_names.push(name.to_owned());
                        self.draft_pack.clear();
                        self.notice = "Pack entry saved locally. Import and checksum verification are still pending.".to_owned();
                    }
                }
            });
        });
        if self.saved.pack_names.is_empty() {
            empty_state(
                ui,
                "No resource packs listed",
                "The future pack manager will verify every downloaded file against a pinned checksum.",
            );
        } else {
            let mut remove_index = None;
            for (index, name) in self.saved.pack_names.iter().enumerate() {
                ui.horizontal(|ui| {
                    ui.label(RichText::new("▧").color(BLUE));
                    ui.label(RichText::new(name).color(TEXT));
                    if ui.small_button("Remove").clicked() {
                        remove_index = Some(index);
                    }
                });
            }
            if let Some(index) = remove_index {
                self.saved.pack_names.remove(index);
            }
        }
    }

    fn servers_page(&mut self, ui: &mut egui::Ui) {
        section_title(ui, "SERVERS", "Multiplayer address book");
        panel(ui, |ui| {
            ui.label(RichText::new("Save a server address").strong().color(TEXT));
            ui.horizontal(|ui| {
                ui.add(
                    egui::TextEdit::singleline(&mut self.draft_server)
                        .hint_text("play.example.net:25565")
                        .desired_width(ui.available_width() - 92.0),
                );
                if ui.button("Add server").clicked() {
                    let address = self.draft_server.trim();
                    if valid_server_address(address) {
                        self.saved.servers.push(ServerEntry {
                            address: address.to_owned(),
                        });
                        self.draft_server.clear();
                        self.notice =
                            "Server address saved. Network protocol connection is not wired yet."
                                .to_owned();
                    } else {
                        self.notice = "Use a hostname or IP address, optionally followed by :port."
                            .to_owned();
                    }
                }
            });
        });
        section_title(ui, "SAVED SERVERS", "Your list stays on this device");
        if self.saved.servers.is_empty() {
            empty_state(
                ui,
                "No servers saved",
                "Add an address above. Joining becomes available when the Rust network client is integrated.",
            );
        } else {
            let mut remove_index = None;
            for (index, server) in self.saved.servers.iter().enumerate() {
                panel(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.label(RichText::new("◎").size(18.0).color(TEAL));
                        ui.vertical(|ui| {
                            ui.label(RichText::new(&server.address).strong().color(TEXT));
                            ui.label(
                                RichText::new("Saved · not connected")
                                    .size(10.0)
                                    .color(MUTED),
                            );
                        });
                        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                            if ui.small_button("Remove").clicked() {
                                remove_index = Some(index);
                            }
                            ui.add_enabled(false, egui::Button::new("Join"));
                        });
                    });
                });
            }
            if let Some(index) = remove_index {
                self.saved.servers.remove(index);
            }
        }
    }

    fn controls_page(&mut self, ui: &mut egui::Ui) {
        section_title(ui, "TOUCH CONTROLS", "Configure your on-screen layout");
        panel(ui, |ui| {
            ui.label(RichText::new("Control scheme").strong().color(TEXT));
            for scheme in [
                "Joystick · Tap",
                "Joystick · Crosshair",
                "D-pad · Tap",
                "D-pad · Crosshair",
            ] {
                ui.radio_value(&mut self.saved.touch_scheme, scheme.to_owned(), scheme);
            }
        });
        panel(ui, |ui| {
            ui.horizontal(|ui| {
                ui.checkbox(&mut self.saved.right_handed, "Right-handed layout");
                ui.checkbox(&mut self.saved.auto_sprint, "Auto Sprint");
                ui.checkbox(&mut self.saved.show_sprint, "Show Sprint button");
            });
            ui.checkbox(&mut self.saved.movement_360, "360° analog movement");
            ui.checkbox(&mut self.saved.vibration, "Phone vibration");
            ui.add(
                egui::Slider::new(&mut self.saved.button_scale, 0.70..=1.50).text("Button scale"),
            );
            ui.add(
                egui::Slider::new(&mut self.saved.button_opacity, 0.20..=1.00)
                    .text("Button opacity"),
            );
        });
        panel(ui, |ui| {
            ui.label(RichText::new("Live preview").strong().color(TEXT));
            touch_preview(
                ui,
                &self.saved.touch_scheme,
                self.saved.button_scale,
                self.saved.button_opacity,
            );
            ui.label(RichText::new("The preference editor is interactive; game input injection is not connected until the engine module is added.")
                .size(10.0).color(MUTED));
        });
    }

    fn accounts_page(&mut self, ui: &mut egui::Ui) {
        section_title(ui, "ACCOUNTS", "Player identities");
        panel(ui, |ui| {
            ui.label(
                RichText::new("Active local profile")
                    .size(11.0)
                    .color(MUTED),
            );
            egui::ComboBox::from_id_salt("profile_select")
                .selected_text(&self.saved.profile_name)
                .show_ui(ui, |ui| {
                    for name in &self.saved.profiles {
                        ui.selectable_value(&mut self.saved.profile_name, name.clone(), name);
                    }
                });
            ui.add_space(8.0);
            ui.horizontal(|ui| {
                ui.add(
                    egui::TextEdit::singleline(&mut self.draft_profile)
                        .hint_text("New offline name")
                        .desired_width(ui.available_width() - 92.0),
                );
                if ui.button("Add profile").clicked() {
                    let name = self.draft_profile.trim();
                    if !name.is_empty()
                        && !self.saved.profiles.iter().any(|existing| existing == name)
                    {
                        self.saved.profiles.push(name.to_owned());
                        self.saved.profile_name = name.to_owned();
                        self.draft_profile.clear();
                        self.notice = "Offline profile saved.".to_owned();
                    } else {
                        self.notice = "Enter a unique profile name.".to_owned();
                    }
                }
            });
        });
        panel(ui, |ui| {
            status_chip(ui, "MICROSOFT AUTH · NOT CONNECTED", AMBER);
            ui.label(RichText::new("Production Microsoft sign-in needs an approved OAuth application, device-code flow, token validation and secure token storage. No password field is collected here.")
                .size(12.0).color(MUTED));
            ui.add_enabled(false, egui::Button::new("Connect Microsoft account"));
        });
    }

    fn downloads_page(&mut self, ui: &mut egui::Ui) {
        section_title(ui, "DOWNLOADS", "Verified transfer queue");
        panel(ui, |ui| {
            status_chip(ui, "NO ACTIVE TRANSFERS", BLUE);
            ui.label(
                RichText::new("The download service is not connected yet.")
                    .size(15.0)
                    .strong()
                    .color(TEXT),
            );
            ui.label(RichText::new("The planned pipeline verifies HTTPS source, expected file size and SHA-256 before a resource or client build is marked installed. Interrupted downloads must resume from verified byte ranges.")
                .size(12.0).color(MUTED));
        });
    }

    fn settings_page(&mut self, ui: &mut egui::Ui) {
        section_title(ui, "SETTINGS", "Rendering and application preferences");
        panel(ui, |ui| {
            ui.label(RichText::new("Renderer preferences").strong().color(TEXT));
            ui.label(
                RichText::new("The launcher UI uses the selected wgpu backend. The separate 3D game renderer is not connected yet.")
                    .size(11.0)
                    .color(MUTED),
            );
            ui.add(
                egui::Slider::new(&mut self.saved.resolution_scale, 50.0..=100.0)
                    .text("Game resolution scale preference"),
            );
            ui.checkbox(&mut self.saved.vsync, "VSync preference");
            ui.add(
                egui::Slider::new(&mut self.saved.fps_limit, 30..=240).text("Game frame cap preference"),
            );
            ui.label(
                RichText::new("Resolution scale, VSync and frame cap are saved for the game-renderer phase; they do not change this launcher UI's swapchain.")
                    .size(10.0)
                    .color(MUTED),
            );
        });

        let gpu = self.gpu_runtime.snapshot();
        panel(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label(RichText::new("GPU / VULKAN DIAGNOSTICS").strong().color(TEXT));
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    status_chip(
                        ui,
                        &gpu.status,
                        if gpu.error.is_some() { AMBER } else { TEAL },
                    );
                });
            });

            ui.add_space(6.0);
            ui.label(RichText::new(format!("Selected GPU: {}", gpu.selected_gpu)).strong().color(TEXT));
            ui.label(RichText::new(format!("Active backend: {}", gpu.selected_backend)).color(TEAL));
            if let Some(error) = &gpu.error {
                ui.label(RichText::new(error).size(10.0).color(ROSE));
            }

            if let Some(selected) = gpu.adapters.iter().find(|adapter| adapter.selected) {
                ui.horizontal_wrapped(|ui| {
                    status_chip(
                        ui,
                        if selected.is_mali { "ARM MALI DETECTED" } else { "GPU DETECTED" },
                        if selected.is_mali { BLUE } else { TEAL },
                    );
                    status_chip(ui, &format!("VENDOR {}", selected.vendor_label), MUTED);
                    status_chip(ui, &format!("DEVICE {}", selected.device_type), MUTED);
                });

                ui.label(
                    RichText::new(format!("Driver: {} {}", selected.driver, selected.driver_info).trim().to_owned())
                        .size(11.0)
                        .color(MUTED),
                );
                ui.label(
                    RichText::new(format!(
                        "Texture formats: {} · ASTC {} · ETC2 {} · BC {}",
                        selected.texture_compression,
                        yes_no(selected.supports_astc),
                        yes_no(selected.supports_etc2),
                        yes_no(selected.supports_bc),
                    ))
                    .size(11.0)
                    .color(TEXT),
                );
                ui.label(
                    RichText::new(format!(
                        "Limits: max 2D texture {} · max buffer {} MiB · storage binding {} MiB",
                        selected.max_texture_dimension_2d,
                        selected.max_buffer_size / (1024 * 1024),
                        selected.max_storage_buffer_binding_size / (1024 * 1024),
                    ))
                    .size(10.0)
                    .color(MUTED),
                );

                if selected.is_mali {
                    ui.label(
                        RichText::new(format!("Mali compatibility profile: {}", selected.mali_profile))
                            .size(10.0)
                            .color(BLUE),
                    );
                    ui.label(
                        RichText::new("Vulkan is used only when the Mali driver exposes a surface-compatible adapter and wgpu can open a device. Otherwise the selector tries OpenGL ES.")
                            .size(10.0)
                            .color(MUTED),
                    );
                }
            }

            if gpu.adapters.is_empty() {
                ui.label(
                    RichText::new("Adapter details will appear after the platform renderer selects a compatible GPU.")
                        .size(11.0)
                        .color(MUTED),
                );
            } else {
                ui.add_space(6.0);
                ui.label(RichText::new("Detected adapters").size(10.0).strong().color(MUTED));
                for adapter in &gpu.adapters {
                    ui.horizontal(|ui| {
                        let marker = if adapter.selected { "●" } else { "○" };
                        ui.label(RichText::new(marker).color(if adapter.selected { TEAL } else { MUTED }));
                        ui.label(RichText::new(format!("{} · {}", adapter.name, adapter.backend)).size(10.0).color(TEXT));
                        if adapter.selected {
                            status_chip(ui, "ACTIVE", TEAL);
                        } else if !adapter.surface_compatible {
                            status_chip(ui, "NO SURFACE", AMBER);
                        }
                    });
                }
            }
        });

        panel(ui, |ui| {
            ui.label(RichText::new("Privacy and storage").strong().color(TEXT));
            ui.label(RichText::new("Profiles, server addresses and UI preferences are stored in the app's local settings. No telemetry service is configured.")
                .size(11.0).color(MUTED));
        });
    }

    fn render_current_page(&mut self, ui: &mut egui::Ui) {
        match self.page {
            Page::Home => self.home_page(ui),
            Page::Game => self.game_page(ui),
            Page::Versions => self.versions_page(ui),
            Page::Worlds => self.worlds_page(ui),
            Page::Mods => self.mods_page(ui),
            Page::Packs => self.packs_page(ui),
            Page::Servers => self.servers_page(ui),
            Page::Controls => self.controls_page(ui),
            Page::Accounts => self.accounts_page(ui),
            Page::Downloads => self.downloads_page(ui),
            Page::Settings => self.settings_page(ui),
        }
    }
}

impl eframe::App for LauncherApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let width = ui.available_width();
        let compact = width < 760.0;

        egui::Panel::top("top_bar")
            .exact_size(48.0)
            .frame(
                egui::Frame::new()
                    .fill(PANEL)
                    .inner_margin(egui::Margin::symmetric(16, 8)),
            )
            .show(ui, |ui| self.top_bar(ui));

        if compact {
            egui::Panel::bottom("mobile_navigation")
                .exact_size(62.0)
                .frame(
                    egui::Frame::new()
                        .fill(PANEL)
                        .inner_margin(egui::Margin::symmetric(8, 6)),
                )
                .show(ui, |ui| self.mobile_nav(ui));
        } else {
            egui::Panel::left("sidebar")
                .exact_size(204.0)
                .resizable(false)
                .frame(
                    egui::Frame::new()
                        .fill(PANEL)
                        .inner_margin(egui::Margin::same(14)),
                )
                .show(ui, |ui| self.sidebar(ui));
        }

        egui::CentralPanel::default()
            .frame(
                egui::Frame::new()
                    .fill(BG)
                    .inner_margin(egui::Margin::same(if compact { 12 } else { 22 })),
            )
            .show(ui, |ui| {
                egui::ScrollArea::vertical()
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        self.render_current_page(ui);
                        ui.add_space(24.0);
                        ui.separator();
                        ui.label(RichText::new(&self.notice).size(10.0).color(MUTED));
                        ui.label(
                            RichText::new("TSI Orbit is an independent project. Minecraft is a trademark of Mojang Synergies AB.")
                                .size(9.0)
                                .color(MUTED),
                        );
                    });
            });
    }

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, "tsi_orbit_state", &self.saved);
    }
}

fn configure_theme(ctx: &egui::Context) {
    let mut style = (*ctx.style_of(egui::Theme::Dark)).clone();
    style.visuals = egui::Visuals::dark();
    style.visuals.panel_fill = BG;
    style.visuals.window_fill = PANEL;
    style.visuals.override_text_color = Some(TEXT);
    style.visuals.widgets.inactive.bg_fill = PANEL_ALT;
    style.visuals.widgets.inactive.fg_stroke.color = TEXT;
    style.visuals.widgets.hovered.bg_fill = Color32::from_rgb(34, 65, 73);
    style.visuals.widgets.active.bg_fill = Color32::from_rgb(34, 97, 94);
    style.visuals.selection.bg_fill = Color32::from_rgb(31, 112, 107);
    style.spacing.item_spacing = Vec2::new(10.0, 9.0);
    ctx.set_style_of(egui::Theme::Dark, style);
}

fn panel(ui: &mut egui::Ui, add_contents: impl FnOnce(&mut egui::Ui)) {
    egui::Frame::new()
        .fill(PANEL)
        .stroke(Stroke::new(1.0, EDGE))
        .corner_radius(egui::CornerRadius::same(14))
        .inner_margin(egui::Margin::same(14))
        .show(ui, add_contents);
    ui.add_space(7.0);
}

fn hero_card(ui: &mut egui::Ui, add_contents: impl FnOnce(&mut egui::Ui)) {
    egui::Frame::new()
        .fill(Color32::from_rgb(20, 43, 51))
        .stroke(Stroke::new(1.0, Color32::from_rgb(45, 105, 107)))
        .corner_radius(egui::CornerRadius::same(18))
        .inner_margin(egui::Margin::same(18))
        .show(ui, add_contents);
}

fn brand_mark(ui: &mut egui::Ui) {
    let (rect, _) = ui.allocate_exact_size(Vec2::splat(38.0), egui::Sense::hover());
    let painter = ui.painter();
    painter.circle_filled(rect.center(), 18.0, Color32::from_rgb(21, 73, 76));
    painter.circle_stroke(rect.center(), 15.0, Stroke::new(2.0, TEAL));
    let arc_points: Vec<egui::Pos2> = (0..=20)
        .map(|step| {
            let angle = 0.35_f32 + (5.2_f32 - 0.35_f32) * step as f32 / 20.0;
            rect.center() + Vec2::new(angle.cos() * 9.0, angle.sin() * 9.0)
        })
        .collect();
    painter.add(egui::Shape::line(arc_points, Stroke::new(2.2, BLUE)));
    painter.circle_filled(rect.center(), 3.2, TEXT);
}

fn orbit_art(ui: &mut egui::Ui, size: Vec2) {
    let (rect, _) = ui.allocate_exact_size(size, egui::Sense::hover());
    let center = rect.center();
    let painter = ui.painter();
    painter.circle_filled(center, 48.0, Color32::from_rgb(23, 83, 82));
    painter.circle_stroke(
        center,
        54.0,
        Stroke::new(1.3, Color32::from_rgb(54, 136, 133)),
    );
    painter.circle_stroke(center, 37.0, Stroke::new(2.0, TEAL));
    painter.circle_filled(center + Vec2::new(31.0, -26.0), 6.0, AMBER);
    painter.circle_filled(center + Vec2::new(-39.0, 20.0), 4.0, BLUE);
    painter.circle_filled(center, 12.0, Color32::from_rgb(9, 24, 30));
    painter.circle_stroke(center, 12.0, Stroke::new(2.0, TEXT));
}

fn status_chip(ui: &mut egui::Ui, label: &str, color: Color32) {
    egui::Frame::new()
        .fill(Color32::from_rgba_unmultiplied(
            color.r(),
            color.g(),
            color.b(),
            28,
        ))
        .stroke(Stroke::new(1.0, color))
        .corner_radius(egui::CornerRadius::same(20))
        .inner_margin(egui::Margin::symmetric(8, 4))
        .show(ui, |ui| {
            ui.label(RichText::new(label).size(9.0).strong().color(color));
        });
}

fn section_title(ui: &mut egui::Ui, eyebrow: &str, title: &str) {
    ui.add_space(4.0);
    ui.label(RichText::new(eyebrow).size(10.0).strong().color(TEAL));
    ui.label(RichText::new(title).size(20.0).strong().color(TEXT));
    ui.add_space(6.0);
}

fn metric_card(ui: &mut egui::Ui, label: &str, value: String, caption: &str, accent: Color32) {
    let width = (ui.available_width() / 2.0 - 8.0).max(130.0);
    egui::Frame::new()
        .fill(PANEL)
        .stroke(Stroke::new(1.0, EDGE))
        .corner_radius(egui::CornerRadius::same(12))
        .inner_margin(egui::Margin::same(12))
        .show(ui, |ui| {
            ui.set_min_width(width);
            ui.label(RichText::new(label).size(9.0).strong().color(MUTED));
            ui.label(RichText::new(value).size(23.0).strong().color(accent));
            ui.label(RichText::new(caption).size(10.0).color(MUTED));
        });
}

fn quick_action(ui: &mut egui::Ui, title: &str, subtitle: &str, action: impl FnOnce()) {
    let response = ui.add(
        egui::Button::new(
            egui::RichText::new(format!("{title}\n{subtitle}"))
                .size(11.0)
                .color(TEXT),
        )
        .fill(PANEL)
        .stroke(Stroke::new(1.0, EDGE)),
    );
    if response.clicked() {
        action();
    }
}

fn info_row(ui: &mut egui::Ui, label: &str, value: &str, description: &str) {
    panel(ui, |ui| {
        ui.horizontal(|ui| {
            ui.label(RichText::new(label).strong().color(TEXT));
            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                ui.label(RichText::new(description).size(10.0).color(MUTED));
                ui.label(RichText::new(value).size(11.0).color(TEAL));
            });
        });
    });
}

fn channel_card(
    ui: &mut egui::Ui,
    title: &str,
    description: &str,
    availability: &str,
    accent: Color32,
) {
    panel(ui, |ui| {
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.label(RichText::new(title).strong().color(accent));
                ui.label(RichText::new(description).size(11.0).color(MUTED));
            });
            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                ui.label(RichText::new(availability).size(10.0).color(MUTED));
            });
        });
    });
}

fn empty_state(ui: &mut egui::Ui, title: &str, description: &str) {
    panel(ui, |ui| {
        ui.vertical_centered(|ui| {
            ui.add_space(10.0);
            ui.label(RichText::new("◌").size(28.0).color(TEAL));
            ui.label(RichText::new(title).strong().color(TEXT));
            ui.label(RichText::new(description).size(11.0).color(MUTED));
            ui.add_space(10.0);
        });
    });
}

fn touch_preview(ui: &mut egui::Ui, scheme: &str, scale: f32, opacity: f32) {
    let (rect, _) =
        ui.allocate_exact_size(Vec2::new(ui.available_width(), 150.0), egui::Sense::hover());
    let painter = ui.painter();
    painter.rect_filled(
        rect,
        egui::CornerRadius::same(10),
        Color32::from_rgb(12, 19, 27),
    );
    painter.rect_stroke(
        rect,
        egui::CornerRadius::same(10),
        Stroke::new(1.0, EDGE),
        egui::StrokeKind::Inside,
    );
    let fill = Color32::from_rgba_unmultiplied(76, 211, 194, (opacity * 255.0) as u8);
    let r = 22.0 * scale;
    let left = rect.left() + 70.0;
    let center_y = rect.center().y;
    if scheme.starts_with("Joystick") {
        painter.circle_filled(
            egui::pos2(left, center_y),
            r * 1.6,
            Color32::from_rgba_unmultiplied(61, 117, 137, 80),
        );
        painter.circle_stroke(egui::pos2(left, center_y), r * 1.6, Stroke::new(1.5, fill));
        painter.circle_filled(egui::pos2(left, center_y), r * 0.65, fill);
    } else {
        painter.rect_stroke(
            egui::Rect::from_center_size(egui::pos2(left, center_y), Vec2::splat(r * 2.6)),
            6.0,
            Stroke::new(1.5, fill),
            egui::StrokeKind::Inside,
        );
        painter.line_segment(
            [
                egui::pos2(left - r, center_y),
                egui::pos2(left + r, center_y),
            ],
            Stroke::new(1.3, fill),
        );
        painter.line_segment(
            [
                egui::pos2(left, center_y - r),
                egui::pos2(left, center_y + r),
            ],
            Stroke::new(1.3, fill),
        );
    }
    let aim = if scheme.ends_with("Crosshair") {
        "AIM"
    } else {
        "TAP"
    };
    painter.circle_filled(egui::pos2(rect.right() - 74.0, center_y), r * 1.2, fill);
    painter.text(
        egui::pos2(rect.right() - 74.0, center_y),
        egui::Align2::CENTER_CENTER,
        aim,
        FontId::proportional(10.0),
        Color32::from_rgb(10, 30, 33),
    );
    painter.text(
        egui::pos2(rect.center().x, rect.bottom() - 12.0),
        egui::Align2::CENTER_CENTER,
        scheme,
        FontId::proportional(10.0),
        MUTED,
    );
}

fn valid_server_address(value: &str) -> bool {
    if value.is_empty() || value.contains(char::is_whitespace) {
        return false;
    }
    if value.starts_with(':') || value.ends_with(':') {
        return false;
    }
    value
        .chars()
        .all(|ch| ch.is_ascii_alphanumeric() || matches!(ch, '.' | '-' | ':' | '[' | ']'))
}

fn yes_no(value: bool) -> &'static str {
    if value { "supported" } else { "not reported" }
}
