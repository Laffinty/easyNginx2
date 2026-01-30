// MIT License
// 
// Copyright (c) 2026 Laffinty
// 
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
// 
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
// 
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use eframe::egui;
use crate::model::l18n::{Language, LanguageChangeRequest};
use std::sync::Arc;
use crate::MessageBus;

// ==============================================================================
// Constants - UI Configuration
// =============================================================================-

/// Site list table column definitions
const COLUMN_CONFIG: [(Option<f32>, &str); 5] = [
    (Some(200.0), "site_list_site"),    // Site name
    (Some(100.0), "site_list_type"),    // Type (Static/PHP/Proxy)
    (Some(100.0), "site_list_port"),    // Port number
    (None,        "site_list_domain"),  // Domain (flexible width)
    (Some(80.0),  "site_list_https"),   // HTTPS status
];

const SPACING: f32 = 16.0;
const HEADER_HEIGHT: f32 = 32.0;
const ROW_HEIGHT: f32 = 40.0;
const MIN_DOMAIN_WIDTH: f32 = 120.0;
const ROW_PADDING_LEFT: f32 = 8.0;
const CONTEXT_MENU_WIDTH: f32 = 120.0;
const CONTEXT_MENU_BUTTON_HEIGHT: f32 = 28.0;
const FONT_SIZE: f32 = 14.0;
const HEADER_FONT_SIZE: f32 = 15.0;

// Color constants
const COLOR_SELECTED: egui::Color32 = egui::Color32::from_rgb(200, 230, 255);
const COLOR_HOVER: egui::Color32 = egui::Color32::from_rgb(240, 248, 255);
const COLOR_TRANSPARENT: egui::Color32 = egui::Color32::TRANSPARENT;

// ==============================================================================
// About Dialog Component
// ==============================================================================

/// AboutDialog - A reusable about dialog component
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AboutDialog {
    is_open: bool,
}

impl Default for AboutDialog {
    fn default() -> Self {
        Self { is_open: false }
    }
}

impl AboutDialog {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn open(&mut self) {
        self.is_open = true;
    }
    
    pub fn close(&mut self) {
        self.is_open = false;
    }
    
    pub fn is_open(&self) -> bool {
        self.is_open
    }
    
    /// Render the about dialog window
    pub fn ui(&mut self, ctx: &egui::Context, language: Language) {
        if !self.is_open {
            return;
        }
        
        let window_title = self.translate("about_title", language);
        
        let response = egui::Window::new(window_title)
            .collapsible(false)
            .resizable(false)
            .fixed_size([420.0, 320.0])
            .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)
            .frame(
                egui::Frame::window(&ctx.style())
                    .inner_margin(egui::Margin::symmetric(24.0, 20.0))
            )
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    Self::render_app_icon(ui);
                    ui.add_space(16.0);
                    self.render_app_info(ui, language);
                    ui.add_space(8.0);
                    ui.separator();
                    ui.add_space(16.0);
                    self.render_details(ui, language);
                    ui.add_space(20.0);
                    self.render_ok_button(ui, language);
                });
            });
        
        // Close dialog when clicking outside or pressing Escape
        let should_close = response.is_none() 
            || ctx.input(|i| i.key_pressed(egui::Key::Escape));
        if should_close {
            self.close();
        }
    }
    
    fn render_app_icon(ui: &mut egui::Ui) {
        let icon_size = 80.0;
        let (rect, _response) = ui.allocate_exact_size(
            egui::vec2(icon_size, icon_size),
            egui::Sense::hover()
        );
        
        if ui.is_rect_visible(rect) {
            let painter = ui.painter();
            painter.rect_filled(rect, 16.0, egui::Color32::from_rgb(76, 175, 80));
            painter.text(
                rect.center(),
                egui::Align2::CENTER_CENTER,
                "N",
                egui::FontId::proportional(48.0),
                egui::Color32::WHITE
            );
        }
    }
    
    fn render_app_info(&self, ui: &mut egui::Ui, language: Language) {
        ui.label(
            egui::RichText::new(self.translate("about_app_name", language))
                .size(24.0)
                .strong()
        );
        
        ui.add_space(4.0);
        
        ui.label(
            egui::RichText::new(self.translate("about_version", language))
                .size(14.0)
                .color(ui.visuals().weak_text_color())
        );
        
        ui.add_space(8.0);
        
        ui.label(
            egui::RichText::new(self.translate("about_description", language))
                .size(13.0)
        );
    }
    
    fn render_details(&self, ui: &mut egui::Ui, language: Language) {
        let label_color = ui.visuals().weak_text_color();
        
        ui.horizontal(|ui| {
            ui.label(egui::RichText::new(self.translate("about_author_label", language)).size(12.0).color(label_color));
            ui.label(egui::RichText::new(self.translate("about_author", language)).size(12.0));
        });
        
        ui.add_space(4.0);
        
        ui.horizontal(|ui| {
            ui.label(egui::RichText::new(self.translate("about_license_label", language)).size(12.0).color(label_color));
            ui.label(egui::RichText::new(self.translate("about_license", language)).size(12.0));
        });
        
        ui.add_space(4.0);
        
        ui.horizontal(|ui| {
            ui.label(egui::RichText::new(self.translate("about_website_label", language)).size(12.0).color(label_color));
            ui.hyperlink_to(
                egui::RichText::new(self.translate("about_website", language)).size(12.0),
                "https://github.com/laffinty/easyNginx"
            );
        });
        
        ui.add_space(4.0);
        
        ui.label(
            egui::RichText::new(self.translate("about_copyright", language))
                .size(11.0)
                .color(label_color)
        );
    }
    
    fn render_ok_button(&self, ui: &mut egui::Ui, language: Language) {
        ui.vertical_centered(|ui| {
            if ui.add_sized(
                [100.0, 32.0],
                egui::Button::new(
                    egui::RichText::new(self.translate("about_ok", language)).size(13.0)
                ).rounding(6.0)
            ).clicked() {
                // Window will close due to the check at the start of ui()
            }
        });
    }
    
    fn translate(&self, key: &str, language: Language) -> String {
        about_translate(key, language)
    }
}

// About dialog translations
fn about_translate(key: &str, language: Language) -> String {
    match (key, language) {
        // English
        ("about_title", Language::English) => "About".into(),
        ("about_app_name", Language::English) => "easyNginx".into(),
        ("about_version", Language::English) => "Version 1.0.0".into(),
        ("about_description", Language::English) => "A simple and intuitive Nginx management tool".into(),
        ("about_author_label", Language::English) => "Author:".into(),
        ("about_author", Language::English) => "Laffinty".into(),
        ("about_license_label", Language::English) => "License:".into(),
        ("about_license", Language::English) => "MIT License".into(),
        ("about_website_label", Language::English) => "Website:".into(),
        ("about_website", Language::English) => "GitHub".into(),
        ("about_copyright", Language::English) => "© 2026 Laffinty. All rights reserved.".into(),
        ("about_ok", Language::English) => "OK".into(),
        
        // Chinese Simplified
        ("about_title", Language::ChineseSimplified) => "关于".into(),
        ("about_app_name", Language::ChineseSimplified) => "easyNginx".into(),
        ("about_version", Language::ChineseSimplified) => "版本 1.0.0".into(),
        ("about_description", Language::ChineseSimplified) => "简单直观的 Nginx 管理工具".into(),
        ("about_author_label", Language::ChineseSimplified) => "作者：".into(),
        ("about_author", Language::ChineseSimplified) => "Laffinty".into(),
        ("about_license_label", Language::ChineseSimplified) => "许可证：".into(),
        ("about_license", Language::ChineseSimplified) => "MIT 许可证".into(),
        ("about_website_label", Language::ChineseSimplified) => "网站：".into(),
        ("about_website", Language::ChineseSimplified) => "GitHub".into(),
        ("about_copyright", Language::ChineseSimplified) => "© 2026 Laffinty. 保留所有权利。".into(),
        ("about_ok", Language::ChineseSimplified) => "确定".into(),
        
        _ => key.into(),
    }
}

// ==============================================================================
// Site List Components
// ==============================================================================

/// Represents a site configuration entry
#[derive(Clone, Debug, PartialEq)]
struct SiteListItem {
    name: String,
    site_type: SiteType,
    port: String,
    domain: String,
    enable_https: bool,
    enable_http_redirect: bool,
}

#[derive(Clone, Debug, PartialEq)]
enum SiteType {
    Static,
    Php,
    Proxy,
}

impl SiteType {
    fn as_str(&self, language: Language) -> &'static str {
        match (self, language) {
            (SiteType::Static, Language::English) => "Static",
            (SiteType::Php, Language::English) => "PHP",
            (SiteType::Proxy, Language::English) => "Proxy",
            (SiteType::Static, Language::ChineseSimplified) => "静态",
            (SiteType::Php, Language::ChineseSimplified) => "PHP",
            (SiteType::Proxy, Language::ChineseSimplified) => "代理",
        }
    }
}

/// Site list panel component
struct SiteListPanel {
    sites: Vec<SiteListItem>,
    selected_site: Option<String>,
    show_context_menu: bool,
    context_menu_pos: egui::Pos2,
    current_language: Language,
}

impl SiteListPanel {
    pub fn new(language: Language) -> Self {
        let sites = vec![
            SiteListItem {
                name: "example-static".into(),
                site_type: SiteType::Static,
                port: "80".into(),
                domain: "static.example.com".into(),
                enable_https: false,
                enable_http_redirect: false,
            },
            SiteListItem {
                name: "example-php".into(),
                site_type: SiteType::Php,
                port: "8080".into(),
                domain: "php.example.com".into(),
                enable_https: true,
                enable_http_redirect: true,
            },
            SiteListItem {
                name: "example-proxy".into(),
                site_type: SiteType::Proxy,
                port: "3000".into(),
                domain: "proxy.example.com".into(),
                enable_https: false,
                enable_http_redirect: false,
            },
        ];
        
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
        egui::ScrollArea::vertical()
            .auto_shrink([false, false])
            .show(ui, |ui| {
                let dynamic_width = self.calculate_dynamic_width(ui.available_width());
                
                self.render_header(ui, dynamic_width);
                ui.separator();
                self.render_rows(ui, ctx, dynamic_width);
            });
    }
    
    fn calculate_dynamic_width(&self, available_width: f32) -> f32 {
        let fixed_width: f32 = COLUMN_CONFIG.iter()
            .filter_map(|(w, _)| *w)
            .sum();
        let spacing_total = SPACING * (COLUMN_CONFIG.len() - 1) as f32;
        
        if fixed_width + spacing_total < available_width {
            available_width - fixed_width - spacing_total - ROW_PADDING_LEFT * 2.0
        } else {
            MIN_DOMAIN_WIDTH
        }
    }
    
    fn render_header(&self, ui: &mut egui::Ui, dynamic_width: f32) {
        let rect = ui.available_rect_before_wrap();
        let rect = rect.with_max_y(rect.min.y + HEADER_HEIGHT);
        ui.advance_cursor_after_rect(rect);
        
        let painter = ui.painter();
        let start_x = rect.left() + ROW_PADDING_LEFT;
        let center_y = rect.center().y;
        let mut x = start_x;
        
        for (col_width, key) in &COLUMN_CONFIG {
            let width = col_width.unwrap_or(dynamic_width);
            let text = self.translate(key);
            
            Self::draw_centered_text(
                painter,
                &text,
                x,
                center_y,
                width,
                ui.visuals().strong_text_color(),
                HEADER_FONT_SIZE,
            );
            
            x += width + SPACING;
        }
    }
    
    fn render_rows(&mut self, ui: &mut egui::Ui, ctx: &egui::Context, dynamic_width: f32) {
        let mut context_menu_action: Option<(String, egui::Pos2)> = None;
        let mut double_click_action: Option<String> = None;
        
        // Clone sites to avoid borrow issues
        let sites: Vec<_> = self.sites.clone();
        
        for site in &sites {
            let selected = self.selected_site.as_ref() == Some(&site.name);
            let row_rect = self.allocate_row_rect(ui);
            let row_response = ui.interact(row_rect, ui.id().with(&site.name), egui::Sense::click());
            
            // Draw background
            self.draw_row_background(ui, row_rect, selected, row_response.hovered());
            
            // Draw content
            self.draw_row_content(ui, row_rect, site, selected, dynamic_width);
            
            // Handle interactions
            if row_response.clicked() {
                self.selected_site = Some(site.name.clone());
            }
            if row_response.double_clicked() {
                self.selected_site = Some(site.name.clone());
                double_click_action = Some(site.name.clone());
            }
            if row_response.secondary_clicked() {
                self.selected_site = Some(site.name.clone());
                let pos = row_response.interact_pointer_pos()
                    .unwrap_or_else(|| row_rect.center());
                context_menu_action = Some((site.name.clone(), pos));
            }
        }
        
        // Process actions after iteration
        if let Some(name) = double_click_action {
            self.edit_site(&name);
        }
        if let Some((name, pos)) = context_menu_action {
            self.show_context_menu_at(ctx, ui, &name, pos);
        }
        
        // Render context menu if open
        if self.show_context_menu {
            self.render_context_menu(ctx, ui);
        }
    }
    
    fn allocate_row_rect(&self, ui: &mut egui::Ui) -> egui::Rect {
        let width = ui.available_width();
        let pos = ui.cursor().min;
        let rect = egui::Rect::from_min_size(pos, egui::vec2(width, ROW_HEIGHT));
        ui.advance_cursor_after_rect(rect);
        rect
    }
    
    fn draw_row_background(&self, ui: &egui::Ui, rect: egui::Rect, selected: bool, hovered: bool) {
        let color = if selected {
            COLOR_SELECTED
        } else if hovered {
            COLOR_HOVER
        } else {
            COLOR_TRANSPARENT
        };
        
        if color != COLOR_TRANSPARENT {
            ui.painter().rect_filled(rect, 4.0, color);
        }
    }
    
    fn draw_row_content(&self, ui: &egui::Ui, rect: egui::Rect, site: &SiteListItem, selected: bool, dynamic_width: f32) {
        let painter = ui.painter();
        let start_x = rect.left() + ROW_PADDING_LEFT;
        let center_y = rect.center().y;
        let mut x = start_x;
        let text_color = if selected {
            ui.visuals().strong_text_color()
        } else {
            ui.visuals().text_color()
        };
        
        for (i, (col_width, _)) in COLUMN_CONFIG.iter().enumerate() {
            let width = col_width.unwrap_or(dynamic_width);
            let text = self.get_column_text(site, i);
            
            Self::draw_centered_text(
                painter,
                &text,
                x,
                center_y,
                width,
                text_color,
                FONT_SIZE,
            );
            
            x += width + SPACING;
        }
    }
    
    fn get_column_text(&self, site: &SiteListItem, column_index: usize) -> String {
        match column_index {
            0 => site.name.clone(),
            1 => site.site_type.as_str(self.current_language).into(),
            2 => {
                if site.enable_https && site.enable_http_redirect {
                    format!("{}/80(redirect)", site.port)
                } else {
                    site.port.clone()
                }
            }
            3 => site.domain.clone(),
            4 => {
                if site.enable_https {
                    self.translate("site_list_https_yes")
                } else {
                    self.translate("site_list_https_no")
                }
            }
            _ => String::new(),
        }
    }
    
    fn draw_centered_text(
        painter: &egui::Painter,
        text: &str,
        x: f32,
        center_y: f32,
        max_width: f32,
        color: egui::Color32,
        font_size: f32,
    ) {
        let font_id = egui::FontId::proportional(font_size);
        
        // Measure text
        let galley = painter.layout(text.into(), font_id.clone(), color, f32::INFINITY);
        let text_width = galley.size().x.min(max_width);
        let offset = (max_width - text_width) / 2.0;
        
        // Recreate with proper wrap width
        let galley = painter.layout(text.into(), font_id, color, max_width);
        let text_height = galley.size().y;
        
        let pos = egui::pos2(x + offset, center_y - text_height / 2.0);
        painter.galley(pos, galley, color);
    }
    
    fn show_context_menu_at(&mut self, ctx: &egui::Context, _ui: &egui::Ui, site_name: &str, pos: egui::Pos2) {
        self.selected_site = Some(site_name.into());
        self.show_context_menu = true;
        
        // Ensure menu doesn't go off screen
        let screen_rect = ctx.screen_rect();
        let menu_width = CONTEXT_MENU_WIDTH;
        let menu_height = CONTEXT_MENU_BUTTON_HEIGHT * 2.0 + 8.0; // 2 buttons + padding
        
        let mut adjusted_pos = pos;
        if pos.x + menu_width > screen_rect.max.x {
            adjusted_pos.x = screen_rect.max.x - menu_width - 10.0;
        }
        if pos.y + menu_height > screen_rect.max.y {
            adjusted_pos.y = screen_rect.max.y - menu_height - 10.0;
        }
        
        self.context_menu_pos = adjusted_pos;
    }
    
    fn render_context_menu(&mut self, ctx: &egui::Context, ui: &egui::Ui) {
        if let Some(site) = self.selected_site.clone() {
            egui::Window::new("site_context_menu")
                .title_bar(false)
                .resizable(false)
                .fixed_pos(self.context_menu_pos)
                .show(ctx, |ui| {
                    ui.set_min_width(CONTEXT_MENU_WIDTH);
                    
                    ui.vertical(|ui| {
                        ui.set_width(CONTEXT_MENU_WIDTH);
                        
                        if self.menu_button(ui, "site_list_edit") {
                            self.show_context_menu = false;
                            self.edit_site(&site);
                        }
                        
                        if self.menu_button(ui, "site_list_delete") {
                            self.show_context_menu = false;
                            self.delete_site(&site);
                        }
                    });
                });
            
            // Close menu when clicking outside
            let clicked_outside = ui.input(|i| i.pointer.any_click())
                && !ctx.is_pointer_over_area();
            if clicked_outside {
                self.show_context_menu = false;
            }
        } else {
            self.show_context_menu = false;
        }
    }
    
    fn menu_button(&self, ui: &mut egui::Ui, key: &str) -> bool {
        ui.add_sized(
            [CONTEXT_MENU_WIDTH, CONTEXT_MENU_BUTTON_HEIGHT],
            egui::Button::new(self.translate(key))
        ).clicked()
    }
    
    fn edit_site(&self, site: &str) {
        println!("Edit site: {}", site);
        // TODO: Implement edit functionality
    }
    
    fn delete_site(&mut self, site: &str) {
        println!("Delete site: {}", site);
        self.sites.retain(|s| s.name != site);
        if self.selected_site.as_deref() == Some(site) {
            self.selected_site = None;
        }
    }
    
    fn translate(&self, key: &str) -> String {
        site_list_translate(key, self.current_language)
    }
}

// Site list translations
fn site_list_translate(key: &str, language: Language) -> String {
    match (key, language) {
        ("site_list_site", Language::English) => "Site".into(),
        ("site_list_type", Language::English) => "Type".into(),
        ("site_list_port", Language::English) => "Port".into(),
        ("site_list_domain", Language::English) => "Domain".into(),
        ("site_list_https", Language::English) => "HTTPS".into(),
        ("site_list_https_yes", Language::English) => "Yes".into(),
        ("site_list_https_no", Language::English) => "No".into(),
        ("site_list_edit", Language::English) => "Edit".into(),
        ("site_list_delete", Language::English) => "Delete".into(),
        ("site_list_site", Language::ChineseSimplified) => "站点".into(),
        ("site_list_type", Language::ChineseSimplified) => "类型".into(),
        ("site_list_port", Language::ChineseSimplified) => "端口".into(),
        ("site_list_domain", Language::ChineseSimplified) => "域名".into(),
        ("site_list_https", Language::ChineseSimplified) => "HTTPS".into(),
        ("site_list_https_yes", Language::ChineseSimplified) => "是".into(),
        ("site_list_https_no", Language::ChineseSimplified) => "否".into(),
        ("site_list_edit", Language::ChineseSimplified) => "编辑".into(),
        ("site_list_delete", Language::ChineseSimplified) => "删除".into(),
        _ => key.into(),
    }
}

// ==============================================================================
// Main Window
// ==============================================================================

pub struct MainWindow {
    site_list_panel: SiteListPanel,
    about_dialog: AboutDialog,
    current_language: Language,
    bus: Option<Arc<MessageBus>>,
}

impl MainWindow {
    pub fn new(bus: Option<Arc<MessageBus>>) -> Self {
        let language = Language::ChineseSimplified;
        Self {
            site_list_panel: SiteListPanel::new(language),
            about_dialog: AboutDialog::new(),
            current_language: language,
            bus,
        }
    }
    
    pub fn set_language(&mut self, language: Language) {
        self.current_language = language;
        self.site_list_panel.set_language(language);
    }
    
    fn get_translation(&self, key: &str) -> String {
        main_window_translate(key, self.current_language)
    }
}

impl eframe::App for MainWindow {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("menu_bar")
            .exact_height(36.0)
            .show(ctx, |ui| {
                ui.style_mut().visuals.widgets.noninteractive.bg_fill = COLOR_TRANSPARENT;
                ui.style_mut().visuals.widgets.inactive.bg_fill = COLOR_TRANSPARENT;
                self.render_menu_bar(ui);
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            self.site_list_panel.ui(ctx, ui);
        });

        egui::TopBottomPanel::bottom("status_bar").show(ctx, |ui| {
            self.render_status_bar(ui);
        });

        self.about_dialog.ui(ctx, self.current_language);
    }
}

impl MainWindow {
    fn render_menu_bar(&mut self, ui: &mut egui::Ui) {
        ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
            self.render_file_menu(ui);
            self.render_operation_menu(ui);
            self.render_language_menu(ui);
            self.render_help_menu(ui);
        });
    }
    
    fn render_file_menu(&mut self, ui: &mut egui::Ui) {
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
    }
    
    fn render_operation_menu(&mut self, ui: &mut egui::Ui) {
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
    }
    
    fn render_language_menu(&mut self, ui: &mut egui::Ui) {
        ui.menu_button(self.get_translation("menu_language"), |ui| {
            let is_english = self.current_language == Language::English;
            let is_chinese = self.current_language == Language::ChineseSimplified;
            
            if ui.radio(is_english, self.get_translation("menu_english")).clicked() {
                self.change_language(Language::English);
                ui.close_menu();
            }
            if ui.radio(is_chinese, self.get_translation("menu_chinese")).clicked() {
                self.change_language(Language::ChineseSimplified);
                ui.close_menu();
            }
        });
    }
    
    fn render_help_menu(&mut self, ui: &mut egui::Ui) {
        ui.menu_button(self.get_translation("menu_help"), |ui| {
            if ui.button(self.get_translation("menu_about")).clicked() {
                ui.close_menu();
                self.about_dialog.open();
            }
        });
    }
    
    fn change_language(&mut self, language: Language) {
        self.set_language(language);
        if let Some(bus) = &self.bus {
            let bus_clone = bus.clone();
            tokio::spawn(async move {
                let _ = bus_clone.publish(LanguageChangeRequest::new(language)).await;
            });
        }
    }
    
    fn render_status_bar(&self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label(self.get_translation("status_nginx_stopped"));
            ui.separator();
            
            let stats = self.calculate_site_stats();
            let text = self.get_translation("status_sites")
                .replace("{total}", &stats.total.to_string())
                .replace("{static}", &stats.static_count.to_string())
                .replace("{php}", &stats.php_count.to_string())
                .replace("{proxy}", &stats.proxy_count.to_string());
            ui.label(text);
            
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.label("easyNginx v1.0.0");
            });
        });
    }
    
    fn calculate_site_stats(&self) -> SiteStats {
        SiteStats {
            total: self.site_list_panel.sites.len(),
            static_count: self.count_sites_by_type(&SiteType::Static),
            php_count: self.count_sites_by_type(&SiteType::Php),
            proxy_count: self.count_sites_by_type(&SiteType::Proxy),
        }
    }
    
    fn count_sites_by_type(&self, site_type: &SiteType) -> usize {
        self.site_list_panel.sites.iter()
            .filter(|s| &s.site_type == site_type)
            .count()
    }
}

struct SiteStats {
    total: usize,
    static_count: usize,
    php_count: usize,
    proxy_count: usize,
}

// Main window translations
fn main_window_translate(key: &str, language: Language) -> String {
    match (key, language) {
        // English
        ("menu_file", Language::English) => "File".into(),
        ("menu_operation", Language::English) => "Operation".into(),
        ("menu_language", Language::English) => "Language".into(),
        ("menu_help", Language::English) => "Help".into(),
        ("menu_takeover_nginx", Language::English) => "Takeover Nginx".into(),
        ("menu_startup_on_boot", Language::English) => "Startup on Boot".into(),
        ("menu_new_proxy", Language::English) => "New Proxy".into(),
        ("menu_new_php", Language::English) => "New PHP".into(),
        ("menu_new_static", Language::English) => "New Static".into(),
        ("menu_exit", Language::English) => "Exit".into(),
        ("menu_start_nginx", Language::English) => "Start Nginx".into(),
        ("menu_stop_nginx", Language::English) => "Stop Nginx".into(),
        ("menu_reload_config", Language::English) => "Reload Config".into(),
        ("menu_refresh_sites", Language::English) => "Refresh Sites".into(),
        ("menu_test_config", Language::English) => "Test Config".into(),
        ("menu_backup_config", Language::English) => "Backup Config".into(),
        ("menu_english", Language::English) => "English".into(),
        ("menu_chinese", Language::English) => "Chinese".into(),
        ("menu_about", Language::English) => "About".into(),
        ("status_nginx_stopped", Language::English) => "Nginx: Stopped".into(),
        ("status_sites", Language::English) => "Sites: Total {total}, Static {static}, PHP {php}, Proxy {proxy}".into(),
        
        // Chinese Simplified
        ("menu_file", Language::ChineseSimplified) => "文件".into(),
        ("menu_operation", Language::ChineseSimplified) => "操作".into(),
        ("menu_language", Language::ChineseSimplified) => "语言".into(),
        ("menu_help", Language::ChineseSimplified) => "帮助".into(),
        ("menu_takeover_nginx", Language::ChineseSimplified) => "接管 Nginx".into(),
        ("menu_startup_on_boot", Language::ChineseSimplified) => "开机启动".into(),
        ("menu_new_proxy", Language::ChineseSimplified) => "新建代理".into(),
        ("menu_new_php", Language::ChineseSimplified) => "新建 PHP".into(),
        ("menu_new_static", Language::ChineseSimplified) => "新建静态".into(),
        ("menu_exit", Language::ChineseSimplified) => "退出".into(),
        ("menu_start_nginx", Language::ChineseSimplified) => "启动 Nginx".into(),
        ("menu_stop_nginx", Language::ChineseSimplified) => "停止 Nginx".into(),
        ("menu_reload_config", Language::ChineseSimplified) => "重载配置".into(),
        ("menu_refresh_sites", Language::ChineseSimplified) => "刷新站点".into(),
        ("menu_test_config", Language::ChineseSimplified) => "测试配置".into(),
        ("menu_backup_config", Language::ChineseSimplified) => "备份配置".into(),
        ("menu_english", Language::ChineseSimplified) => "English".into(),
        ("menu_chinese", Language::ChineseSimplified) => "中文".into(),
        ("menu_about", Language::ChineseSimplified) => "关于".into(),
        ("status_nginx_stopped", Language::ChineseSimplified) => "Nginx: 已停止".into(),
        ("status_sites", Language::ChineseSimplified) => "站点: 总计 {total}, 静态 {static}, PHP {php}, 代理 {proxy}".into(),
        
        _ => key.into(),
    }
}

pub fn create_main_window(bus: Option<Arc<MessageBus>>) -> Box<dyn eframe::App> {
    Box::new(MainWindow::new(bus))
}
