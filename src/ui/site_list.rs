use crate::core::LanguageManager;
use crate::models::{NginxStatus, SiteListItem};
use eframe::egui;
use std::sync::{Arc, RwLock};

pub struct SiteListPanel {
    language_manager: Arc<RwLock<LanguageManager>>,
    sites: Arc<RwLock<Vec<SiteListItem>>>,
    nginx_status: Arc<RwLock<NginxStatus>>,
    selected_site: Option<String>,
    show_context_menu: bool,
    context_menu_pos: egui::Pos2,
}

impl SiteListPanel {
    pub fn new(
        language_manager: Arc<RwLock<LanguageManager>>,
        sites: Arc<RwLock<Vec<SiteListItem>>>,
        nginx_status: Arc<RwLock<NginxStatus>>,
    ) -> Self {
        Self {
            language_manager,
            sites,
            nginx_status,
            selected_site: None,
            show_context_menu: false,
            context_menu_pos: egui::Pos2::ZERO,
        }
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) {
        // 列宽定义（None = 吃剩余宽度）
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

                // ===== 宽度计算 =====
                let available_width = ui.available_width().max(400.0);
                let fixed_width: f32 = col_widths.iter().filter_map(|&w| w).sum();
                let spacing_total = spacing * (col_widths.len() - 1) as f32;
                let dynamic_width = if fixed_width + spacing_total < available_width {
                    available_width - fixed_width - spacing_total
                } else {
                    120.0
                };

                // ===== 表头 =====
                ui.horizontal(|ui| {
                    ui.set_height(header_height);
                    for (i, col) in col_widths.iter().enumerate() {
                        let w = col.unwrap_or(dynamic_width);
                        let text = match i {
                            0 => self.translate("site"),
                            1 => self.translate("type"),
                            2 => self.translate("port"),
                            3 => self.translate("domain"),
                            4 => "HTTPS".to_string(),
                            _ => "".to_string(),
                        };

                        ui.add_sized(
                            egui::vec2(w, header_height),
                            egui::Label::new(egui::RichText::new(text).strong())
                                .wrap_mode(egui::TextWrapMode::Extend),
                        );

                        if i + 1 < col_widths.len() {
                            ui.add_space(spacing);
                        }
                    }
                });

                ui.separator();

                let lang = self.language_manager.read().unwrap().current_language();
                let redirect_text = self.translate("redirect");
                let yes_text = self.translate("yes");
                let no_text = self.translate("no");

                // ===== 拷贝数据，避免 UI 中持锁 =====
                let sites_data: Vec<_> = {
                    let sites = self.sites.read().unwrap();
                    sites
                        .iter()
                        .map(|s| {
                            (
                                s.site_name.clone(),
                                s.site_type.display_name(&lang).to_string(),
                                s.listen_port.to_string(),
                                s.server_name.clone(),
                                s.enable_https,
                                s.enable_http_redirect,
                            )
                        })
                        .collect()
                };

                // ===== 内容行 =====
                for (site, ty, port, domain, https, redirect) in sites_data {
                    let selected = self
                        .selected_site
                        .as_ref()
                        .map_or(false, |s| s == &site);

                    let row_size = egui::vec2(available_width, row_height);
                    let (_id, row_rect) = ui.allocate_space(row_size);

                    let response = ui.interact(
                        row_rect,
                        ui.make_persistent_id(&site),
                        egui::Sense::click(),
                    );

                    ui.allocate_new_ui(egui::UiBuilder::new().max_rect(row_rect), |ui| {
                        ui.set_height(row_height);
                        ui.spacing_mut().item_spacing.x = spacing;

                        ui.horizontal(|ui| {
                            for (i, col) in col_widths.iter().enumerate() {
                                let w = col.unwrap_or(dynamic_width);

                                match i {
                                    // site
                                    0 => {
                                        ui.add_sized(
                                            egui::vec2(w, row_height),
                                            egui::SelectableLabel::new(selected, &site),
                                        );
                                    }

                                    // type（不换行）
                                    1 => {
                                        ui.add_sized(
                                            egui::vec2(w, row_height),
                                            egui::Label::new(&ty).wrap_mode(egui::TextWrapMode::Extend),
                                        )
                                        .on_hover_text(&ty);
                                    }

                                    // port（不换行）
                                    2 => {
                                        let text = if https && redirect {
                                            format!("{}/80({})", port, redirect_text)
                                        } else {
                                            port.clone()
                                        };
                                        ui.add_sized(
                                            egui::vec2(w, row_height),
                                            egui::Label::new(text).wrap_mode(egui::TextWrapMode::Extend),
                                        );
                                    }

                                    // domain（✅ 允许换行）
                                    3 => {
                                        ui.add_sized(
                                            egui::vec2(w, row_height),
                                            egui::Label::new(&domain).wrap_mode(egui::TextWrapMode::Wrap),
                                        )
                                        .on_hover_text(&domain);
                                    }

                                    // https（不换行）
                                    4 => {
                                        ui.add_sized(
                                            egui::vec2(w, row_height),
                                            egui::Label::new(if https { &yes_text } else { &no_text })
                                                .wrap_mode(egui::TextWrapMode::Extend),
                                        );
                                    }
                                    _ => {}
                                }

                                if i + 1 < col_widths.len() {
                                    ui.add_space(spacing);
                                }
                            }
                        });
                    });

                    // ===== 行级事件 =====
                    if response.clicked() {
                        self.selected_site = Some(site.clone());
                    }

                    if response.double_clicked() {
                        self.edit_site(&site);
                        return;
                    }

                    if response.secondary_clicked() {
                        self.selected_site = Some(site.clone());
                        self.show_context_menu = true;
                        self.context_menu_pos = ui
                            .input(|i| i.pointer.hover_pos().unwrap_or(row_rect.left_top()));
                    }
                }

                // ===== 右键菜单 =====
                if self.show_context_menu {
                    if let Some(site) = self.selected_site.clone() {
                        egui::Window::new("site_context_menu")
                            .title_bar(false)
                            .resizable(false)
                            .fixed_pos(self.context_menu_pos)
                            .show(ui.ctx(), |ui| {
                                ui.set_min_width(120.0);

                                if ui.button(self.translate("edit")).clicked() {
                                    self.show_context_menu = false;
                                    self.edit_site(&site);
                                }

                                if ui.button(self.translate("delete")).clicked() {
                                    self.show_context_menu = false;
                                    self.delete_site(&site);
                                }
                            });

                        if ui.input(|i| i.pointer.any_click())
                            && !ui.ctx().is_pointer_over_area()
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
        let mut sites = self.sites.write().unwrap();
        sites.retain(|s| s.site_name != site);
        if self.selected_site.as_deref() == Some(site) {
            self.selected_site = None;
        }
    }

    fn translate(&self, key: &str) -> String {
        self.language_manager.read().unwrap().get(key)
    }
}
