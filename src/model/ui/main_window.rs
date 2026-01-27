use eframe::egui;

#[derive(Clone)]
struct SiteListItem {
    site_name: String,
    site_type: String,
    listen_port: String,
    server_name: String,
    enable_https: bool,
    enable_http_redirect: bool,
}

struct SiteListPanel {
    sites: Vec<SiteListItem>,
    selected_site: Option<String>,
    show_context_menu: bool,
    context_menu_pos: egui::Pos2,
}

impl SiteListPanel {
    pub fn new() -> Self {
        let mut sites = Vec::new();
        
        sites.push(SiteListItem {
            site_name: "example-static".to_string(),
            site_type: "Static".to_string(),
            listen_port: "80".to_string(),
            server_name: "static.example.com".to_string(),
            enable_https: false,
            enable_http_redirect: false,
        });
        
        sites.push(SiteListItem {
            site_name: "example-php".to_string(),
            site_type: "PHP".to_string(),
            listen_port: "8080".to_string(),
            server_name: "php.example.com".to_string(),
            enable_https: true,
            enable_http_redirect: true,
        });
        
        sites.push(SiteListItem {
            site_name: "example-proxy".to_string(),
            site_type: "Proxy".to_string(),
            listen_port: "3000".to_string(),
            server_name: "proxy.example.com".to_string(),
            enable_https: false,
            enable_http_redirect: false,
        });
        
        Self {
            sites,
            selected_site: None,
            show_context_menu: false,
            context_menu_pos: egui::Pos2::ZERO,
        }
    }
    
    pub fn ui(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        let col_widths: [Option<f32>; 5] = [
            Some(200.0), // site
            Some(100.0), // type
            Some(100.0), // port
            None,        // domain
            Some(60.0),  // https
        ];

        let spacing = 16.0;
        let header_height = 32.0;
        let row_height = 36.0;

        egui::ScrollArea::vertical()
            .auto_shrink([false, false])
            .show(ui, |ui| {
                ui.spacing_mut().item_spacing.x = spacing;

                let available_width = ui.available_width().max(400.0);
                let fixed_width: f32 = col_widths.iter().filter_map(|&w| w).sum();
                let spacing_total = spacing * (col_widths.len() - 1) as f32;
                let dynamic_width = if fixed_width + spacing_total < available_width {
                    available_width - fixed_width - spacing_total
                } else {
                    120.0
                };

                ui.horizontal(|ui| {
                    ui.set_height(header_height);
                    for (i, col) in col_widths.iter().enumerate() {
                        let w = col.unwrap_or(dynamic_width);
                        let text = match i {
                            0 => "Site",
                            1 => "Type",
                            2 => "Port",
                            3 => "Domain",
                            4 => "HTTPS",
                            _ => "",
                        };

                        ui.add_sized(
                            egui::vec2(w, header_height),
                            egui::Label::new(egui::RichText::new(text).strong()),
                        );

                        if i + 1 < col_widths.len() {
                            ui.add_space(spacing);
                        }
                    }
                });

                ui.separator();

                let mut click_actions = Vec::new();
                let mut double_click_actions = Vec::new();
                let mut right_click_actions = Vec::new();
                let mut context_menu_pos = None;
                
                for site in &self.sites {
                    let selected = self
                        .selected_site
                        .as_ref()
                        .map_or(false, |s| s == &site.site_name);

                    let site_name = site.site_name.clone();
                    
                    let _row_size = egui::vec2(available_width, row_height);
                    ui.horizontal(|ui| {
                        ui.set_height(row_height);
                        ui.spacing_mut().item_spacing.x = spacing;

                        for (i, col) in col_widths.iter().enumerate() {
                            let w = col.unwrap_or(dynamic_width);

                            match i {
                                0 => {
                                    let response = ui.add_sized(
                                        egui::vec2(w, row_height),
                                        egui::SelectableLabel::new(selected, &site_name),
                                    );
                                    if response.clicked() {
                                        click_actions.push(site_name.clone());
                                    }
                                    if response.double_clicked() {
                                        double_click_actions.push(site_name.clone());
                                    }
                                    if response.secondary_clicked() {
                                        right_click_actions.push(site_name.clone());
                                        context_menu_pos = ui.input(|i| i.pointer.hover_pos());
                                    }
                                }

                                1 => {
                                    ui.add_sized(
                                        egui::vec2(w, row_height),
                                        egui::Label::new(&site.site_type),
                                    )
                                    .on_hover_text(&site.site_type);
                                }

                                2 => {
                                    let text = if site.enable_https && site.enable_http_redirect {
                                        format!("{}/80(redirect)", site.listen_port)
                                    } else {
                                        site.listen_port.clone()
                                    };
                                    ui.add_sized(
                                        egui::vec2(w, row_height),
                                        egui::Label::new(text),
                                    );
                                }

                                3 => {
                                    ui.add_sized(
                                        egui::vec2(w, row_height),
                                        egui::Label::new(&site.server_name),
                                    )
                                    .on_hover_text(&site.server_name);
                                }

                                4 => {
                                    ui.add_sized(
                                        egui::vec2(w, row_height),
                                        egui::Label::new(if site.enable_https { "Yes" } else { "No" }),
                                    );
                                }
                                _ => {}
                            }

                            if i + 1 < col_widths.len() {
                                ui.add_space(spacing);
                            }
                        }
                    });
                }
                
                // Process actions after loop to avoid borrow conflicts
                for site_name in click_actions {
                    self.selected_site = Some(site_name);
                }
                for site_name in double_click_actions {
                    self.edit_site(&site_name);
                }
                for site_name in right_click_actions {
                    self.selected_site = Some(site_name);
                    self.show_context_menu = true;
                    if let Some(pos) = context_menu_pos {
                        self.context_menu_pos = pos;
                    }
                }

                if self.show_context_menu {
                    if let Some(site) = self.selected_site.clone() {
                        egui::Window::new("site_context_menu")
                            .title_bar(false)
                            .resizable(false)
                            .fixed_pos(self.context_menu_pos)
                            .show(ctx, |ui| {
                                ui.set_min_width(120.0);

                                if ui.button("Edit").clicked() {
                                    self.show_context_menu = false;
                                    self.edit_site(&site);
                                }

                                if ui.button("Delete").clicked() {
                                    self.show_context_menu = false;
                                    self.delete_site(&site);
                                }
                            });

                        if ui.input(|i| i.pointer.any_click())
                            && !ctx.is_pointer_over_area()
                        {
                            self.show_context_menu = false;
                        }
                    }
                }
            });
    }
    
    fn edit_site(&mut self, site: &str) {
        println!("Edit site: {}", site);
    }
    
    fn delete_site(&mut self, site: &str) {
        println!("Delete site: {}", site);
        self.sites.retain(|s| s.site_name != site);
        if self.selected_site.as_deref() == Some(site) {
            self.selected_site = None;
        }
    }
}

pub struct MainWindow {
    site_list_panel: SiteListPanel,
    show_about: bool,
}

impl MainWindow {
    pub fn new() -> Self {
        Self {
            site_list_panel: SiteListPanel::new(),
            show_about: false,
        }
    }
}

impl eframe::App for MainWindow {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("menu_bar")
            .exact_height(36.0)
            .show(ctx, |ui| {
                ui.style_mut().visuals.widgets.noninteractive.bg_fill = egui::Color32::TRANSPARENT;
                ui.style_mut().visuals.widgets.inactive.bg_fill = egui::Color32::TRANSPARENT;
                self.render_menu_bar(ui);
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            self.site_list_panel.ui(ctx, ui);
        });

        egui::TopBottomPanel::bottom("status_bar").show(ctx, |ui| {
            self.render_status_bar(ui);
        });

        if self.show_about {
            self.show_about = false;
            
            egui::Window::new("About")
                .collapsible(false)
                .resizable(false)
                .show(ctx, |ui| {
                    ui.label("easyNginx");
                    ui.label("Version 1.0.0");
                    ui.label("A simple Nginx management tool");
                    if ui.button("OK").clicked() {
                        // 对话框会自动关闭
                    }
                });
        }
    }
}

impl MainWindow {
    fn render_menu_bar(&mut self, ui: &mut egui::Ui) {
        ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
            ui.menu_button("File", |ui| {
                if ui.button("Takeover Nginx").clicked() {
                    ui.close_menu();
                }

                if ui.button("Startup on Boot").clicked() {
                    ui.close_menu();
                }

                ui.separator();

                if ui.button("New Proxy").clicked() {
                    ui.close_menu();
                }

                if ui.button("New PHP").clicked() {
                    ui.close_menu();
                }

                if ui.button("New Static").clicked() {
                    ui.close_menu();
                }

                ui.separator();

                if ui.button("Exit").clicked() {
                    ui.close_menu();
                    std::process::exit(0);
                }
            });

            ui.menu_button("Operation", |ui| {
                if ui.button("Start Nginx").clicked() {
                    ui.close_menu();
                }

                if ui.button("Stop Nginx").clicked() {
                    ui.close_menu();
                }

                if ui.button("Reload Config").clicked() {
                    ui.close_menu();
                }

                ui.separator();

                if ui.button("Refresh Sites").clicked() {
                    ui.close_menu();
                }

                ui.separator();

                if ui.button("Test Config").clicked() {
                    ui.close_menu();
                }

                if ui.button("Backup Config").clicked() {
                    ui.close_menu();
                }
            });

            ui.menu_button("Language", |ui| {
                if ui.radio(false, "English").clicked() {
                    ui.close_menu();
                }

                if ui.radio(true, "中文").clicked() {
                    ui.close_menu();
                }
            });

            ui.menu_button("Help", |ui| {
                if ui.button("About").clicked() {
                    ui.close_menu();
                    self.show_about = true;
                }
            });
        });
    }

    fn render_status_bar(&self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            let status_text = format!("Nginx: Stopped");
            ui.label(status_text);

            ui.separator();

            let total = self.site_list_panel.sites.len();
            let static_count = self.site_list_panel.sites.iter().filter(|s| s.site_type == "Static").count();
            let php_count = self.site_list_panel.sites.iter().filter(|s| s.site_type == "PHP").count();
            let proxy_count = self.site_list_panel.sites.iter().filter(|s| s.site_type == "Proxy").count();

            let stats_text = format!("Sites: Total {}, Static {}, PHP {}, Proxy {}", total, static_count, php_count, proxy_count);
            ui.label(stats_text);

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.label("easyNginx v1.0.0");
            });
        });
    }
}
