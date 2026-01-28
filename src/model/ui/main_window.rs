use eframe::egui;
use crate::model::l18n::{Language, LanguageChangeRequest};
use std::sync::Arc;
use crate::MessageBus;

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
    current_language: Language,
}

impl SiteListPanel {
    pub fn new(language: Language) -> Self {
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
            current_language: language,
        }
    }
    
    pub fn set_language(&mut self, language: Language) {
        self.current_language = language;
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
                            0 => self.get_translation("site_list_site"),
                            1 => self.get_translation("site_list_type"),
                            2 => self.get_translation("site_list_port"),
                            3 => self.get_translation("site_list_domain"),
                            4 => self.get_translation("site_list_https"),
                            _ => "".to_string(),
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

                                if ui.button(self.get_translation("site_list_edit")).clicked() {
                                    self.show_context_menu = false;
                                    self.edit_site(&site);
                                }

                                if ui.button(self.get_translation("site_list_delete")).clicked() {
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
    
    fn get_translation(&self, key: &str) -> String {
        match (key, self.current_language) {
            ("site_list_site", Language::English) => "Site".to_string(),
            ("site_list_type", Language::English) => "Type".to_string(),
            ("site_list_port", Language::English) => "Port".to_string(),
            ("site_list_domain", Language::English) => "Domain".to_string(),
            ("site_list_https", Language::English) => "HTTPS".to_string(),
            ("site_list_edit", Language::English) => "Edit".to_string(),
            ("site_list_delete", Language::English) => "Delete".to_string(),
            ("site_list_site", Language::ChineseSimplified) => "站点".to_string(),
            ("site_list_type", Language::ChineseSimplified) => "类型".to_string(),
            ("site_list_port", Language::ChineseSimplified) => "端口".to_string(),
            ("site_list_domain", Language::ChineseSimplified) => "域名".to_string(),
            ("site_list_https", Language::ChineseSimplified) => "HTTPS".to_string(),
            ("site_list_edit", Language::ChineseSimplified) => "编辑".to_string(),
            ("site_list_delete", Language::ChineseSimplified) => "删除".to_string(),
            _ => key.to_string(),
        }
    }
}

pub struct MainWindow {
    site_list_panel: SiteListPanel,
    show_about: bool,
    current_language: Language,
    bus: Option<Arc<MessageBus>>,
}

impl MainWindow {
    pub fn new(bus: Option<Arc<MessageBus>>) -> Self {
        let language = Language::ChineseSimplified;
        Self {
            site_list_panel: SiteListPanel::new(language),
            show_about: false,
            current_language: language,
            bus,
        }
    }
    
    pub fn set_language(&mut self, language: Language) {
        self.current_language = language;
        self.site_list_panel.set_language(language);
    }
    
    pub fn get_translation(&self, key: &str) -> String {
        match (key, self.current_language) {
            ("menu_file", Language::English) => "File".to_string(),
            ("menu_operation", Language::English) => "Operation".to_string(),
            ("menu_language", Language::English) => "Language".to_string(),
            ("menu_help", Language::English) => "Help".to_string(),
            ("menu_takeover_nginx", Language::English) => "Takeover Nginx".to_string(),
            ("menu_startup_on_boot", Language::English) => "Startup on Boot".to_string(),
            ("menu_new_proxy", Language::English) => "New Proxy".to_string(),
            ("menu_new_php", Language::English) => "New PHP".to_string(),
            ("menu_new_static", Language::English) => "New Static".to_string(),
            ("menu_exit", Language::English) => "Exit".to_string(),
            ("menu_start_nginx", Language::English) => "Start Nginx".to_string(),
            ("menu_stop_nginx", Language::English) => "Stop Nginx".to_string(),
            ("menu_reload_config", Language::English) => "Reload Config".to_string(),
            ("menu_refresh_sites", Language::English) => "Refresh Sites".to_string(),
            ("menu_test_config", Language::English) => "Test Config".to_string(),
            ("menu_backup_config", Language::English) => "Backup Config".to_string(),
            ("menu_english", Language::English) => "English".to_string(),
            ("menu_chinese", Language::English) => "Chinese".to_string(),
            ("menu_about", Language::English) => "About".to_string(),
            ("status_nginx_stopped", Language::English) => "Nginx: Stopped".to_string(),
            ("status_sites", Language::English) => "Sites: Total {}, Static {}, PHP {}, Proxy {}".to_string(),
            ("about_title", Language::English) => "About".to_string(),
            ("about_app_name", Language::English) => "easyNginx".to_string(),
            ("about_version", Language::English) => "Version 1.0.0".to_string(),
            ("about_description", Language::English) => "A simple Nginx management tool".to_string(),
            ("about_ok", Language::English) => "OK".to_string(),
            ("menu_file", Language::ChineseSimplified) => "文件".to_string(),
            ("menu_operation", Language::ChineseSimplified) => "操作".to_string(),
            ("menu_language", Language::ChineseSimplified) => "语言".to_string(),
            ("menu_help", Language::ChineseSimplified) => "帮助".to_string(),
            ("menu_takeover_nginx", Language::ChineseSimplified) => "接管 Nginx".to_string(),
            ("menu_startup_on_boot", Language::ChineseSimplified) => "开机启动".to_string(),
            ("menu_new_proxy", Language::ChineseSimplified) => "新建代理".to_string(),
            ("menu_new_php", Language::ChineseSimplified) => "新建 PHP".to_string(),
            ("menu_new_static", Language::ChineseSimplified) => "新建静态".to_string(),
            ("menu_exit", Language::ChineseSimplified) => "退出".to_string(),
            ("menu_start_nginx", Language::ChineseSimplified) => "启动 Nginx".to_string(),
            ("menu_stop_nginx", Language::ChineseSimplified) => "停止 Nginx".to_string(),
            ("menu_reload_config", Language::ChineseSimplified) => "重载配置".to_string(),
            ("menu_refresh_sites", Language::ChineseSimplified) => "刷新站点".to_string(),
            ("menu_test_config", Language::ChineseSimplified) => "测试配置".to_string(),
            ("menu_backup_config", Language::ChineseSimplified) => "备份配置".to_string(),
            ("menu_english", Language::ChineseSimplified) => "English".to_string(),
            ("menu_chinese", Language::ChineseSimplified) => "中文".to_string(),
            ("menu_about", Language::ChineseSimplified) => "关于".to_string(),
            ("status_nginx_stopped", Language::ChineseSimplified) => "Nginx: 已停止".to_string(),
            ("status_sites", Language::ChineseSimplified) => "站点: 总计 {}, 静态 {}, PHP {}, 代理 {}".to_string(),
            ("about_title", Language::ChineseSimplified) => "关于".to_string(),
            ("about_app_name", Language::ChineseSimplified) => "easyNginx".to_string(),
            ("about_version", Language::ChineseSimplified) => "版本 1.0.0".to_string(),
            ("about_description", Language::ChineseSimplified) => "简单的 Nginx 管理工具".to_string(),
            ("about_ok", Language::ChineseSimplified) => "确定".to_string(),
            _ => key.to_string(),
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
            
            egui::Window::new(self.get_translation("about_title"))
                .collapsible(false)
                .resizable(false)
                .show(ctx, |ui| {
                    ui.label(self.get_translation("about_app_name"));
                    ui.label(self.get_translation("about_version"));
                    ui.label(self.get_translation("about_description"));
                    if ui.button(self.get_translation("about_ok")).clicked() {
                        // 对话框会自动关闭
                    }
                });
        }
    }
}

impl MainWindow {
    fn render_menu_bar(&mut self, ui: &mut egui::Ui) {
        ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
            ui.menu_button(self.get_translation("menu_file"), |ui| {
                if ui.button(self.get_translation("menu_takeover_nginx")).clicked() {
                    ui.close_menu();
                }

                if ui.button(self.get_translation("menu_startup_on_boot")).clicked() {
                    ui.close_menu();
                }

                ui.separator();

                if ui.button(self.get_translation("menu_new_proxy")).clicked() {
                    ui.close_menu();
                }

                if ui.button(self.get_translation("menu_new_php")).clicked() {
                    ui.close_menu();
                }

                if ui.button(self.get_translation("menu_new_static")).clicked() {
                    ui.close_menu();
                }

                ui.separator();

                if ui.button(self.get_translation("menu_exit")).clicked() {
                    ui.close_menu();
                    std::process::exit(0);
                }
            });

            ui.menu_button(self.get_translation("menu_operation"), |ui| {
                if ui.button(self.get_translation("menu_start_nginx")).clicked() {
                    ui.close_menu();
                }

                if ui.button(self.get_translation("menu_stop_nginx")).clicked() {
                    ui.close_menu();
                }

                if ui.button(self.get_translation("menu_reload_config")).clicked() {
                    ui.close_menu();
                }

                ui.separator();

                if ui.button(self.get_translation("menu_refresh_sites")).clicked() {
                    ui.close_menu();
                }

                ui.separator();

                if ui.button(self.get_translation("menu_test_config")).clicked() {
                    ui.close_menu();
                }

                if ui.button(self.get_translation("menu_backup_config")).clicked() {
                    ui.close_menu();
                }
            });

            ui.menu_button(self.get_translation("menu_language"), |ui| {
                if ui.radio(self.current_language == Language::English, self.get_translation("menu_english")).clicked() {
                    self.set_language(Language::English);
                    if let Some(bus) = &self.bus {
                        // 使用tokio::spawn在异步任务中发送消息
                        let bus_clone = bus.clone();
                        tokio::spawn(async move {
                            let _ = bus_clone.publish(LanguageChangeRequest::new(Language::English)).await;
                        });
                    }
                    ui.close_menu();
                }

                if ui.radio(self.current_language == Language::ChineseSimplified, self.get_translation("menu_chinese")).clicked() {
                    self.set_language(Language::ChineseSimplified);
                    if let Some(bus) = &self.bus {
                        // 使用tokio::spawn在异步任务中发送消息
                        let bus_clone = bus.clone();
                        tokio::spawn(async move {
                            let _ = bus_clone.publish(LanguageChangeRequest::new(Language::ChineseSimplified)).await;
                        });
                    }
                    ui.close_menu();
                }
            });

            ui.menu_button(self.get_translation("menu_help"), |ui| {
                if ui.button(self.get_translation("menu_about")).clicked() {
                    ui.close_menu();
                    self.show_about = true;
                }
            });
        });
    }

    fn render_status_bar(&self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            let status_text = self.get_translation("status_nginx_stopped");
            ui.label(status_text);

            ui.separator();

            let total = self.site_list_panel.sites.len();
            let static_count = self.site_list_panel.sites.iter().filter(|s| s.site_type == "Static").count();
            let php_count = self.site_list_panel.sites.iter().filter(|s| s.site_type == "PHP").count();
            let proxy_count = self.site_list_panel.sites.iter().filter(|s| s.site_type == "Proxy").count();

            let stats_template = self.get_translation("status_sites");
            let stats_text = stats_template
                .replace("{total}", &total.to_string())
                .replace("{static}", &static_count.to_string())
                .replace("{php}", &php_count.to_string())
                .replace("{proxy}", &proxy_count.to_string());
            ui.label(stats_text);

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.label("easyNginx v1.0.0");
            });
        });
    }
}

pub fn create_main_window(bus: Option<Arc<MessageBus>>) -> Box<dyn eframe::App> {
    // 创建应用实例
    let window = MainWindow::new(bus);
    
    Box::new(window)
}
