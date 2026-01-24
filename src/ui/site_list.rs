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
            context_menu_pos: egui::Pos2::default(),
        }
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) {
        // 工具栏 - 精简版，移除标题和右侧按钮
        ui.horizontal(|_ui| {
            // 留空，保持布局整洁
        });

        // 站点列表 - 平铺式布局，每一行横向填充
        egui::ScrollArea::both().show(ui, |ui| {
            // 表头 - 平铺式布局
            ui.horizontal(|ui| {
                ui.set_height(32.0);
                
                // 重置水平间距，使用自定义间距
                ui.spacing_mut().item_spacing.x = 16.0; // 设置元素间间距
                
                // 站点名称 - 优先宽度
                ui.strong(self.translate("site"));
                ui.add_space(120.0); // 增加额外间距，让表头与数据对齐
                
                // 类型 - 固定宽度
                ui.strong(self.translate("type"));
                ui.add_space(80.0);
                
                // 端口 - 固定宽度
                ui.strong(self.translate("port"));
                ui.add_space(80.0);
                
                // 域名 - 自适应剩余空间
                ui.strong(self.translate("domain"));
                ui.add_space(120.0);
                
                // HTTPS - 固定宽度
                ui.strong("HTTPS");
            });



            let lang = self.language_manager.read().unwrap().current_language();
            let redirect_text = self.translate("redirect");
            let yes_text = self.translate("yes");
            let no_text = self.translate("no");

            // 克隆站点数据，避免在循环中持锁
            let sites_data: Vec<(String, String, String, String, bool, bool)> = {
                let sites = self.sites.read().unwrap();
                sites
                    .iter()
                    .map(|s| {
                        let type_text = s.site_type.display_name(&lang);
                        (
                            s.site_name.clone(),
                            type_text.to_string(),  // 转换为String
                            s.listen_port.to_string(),
                            s.server_name.clone(),
                            s.enable_https,
                            s.enable_http_redirect,
                        )
                    })
                    .collect()
            };
            
            let row_height = 36.0;
            
            for (site_name, type_text, port_text, server_name, enable_https, enable_http_redirect) in sites_data {
                let is_selected = self
                    .selected_site
                    .as_ref()
                    .map_or(false, |s| s == &site_name);

                let site_name_clone1 = site_name.clone();
                let site_name_clone2 = site_name.clone();
                let site_name_for_edit = site_name.clone();

                // 每一行 - 平铺式布局
                let response = ui.horizontal(|ui| {
                    ui.set_height(row_height);
                    
                    // 重置水平间距，使用自定义间距（与表头一致）
                    ui.spacing_mut().item_spacing.x = 16.0;
                    
                    // 站点名称 - 可点击选择
                    let site_response = ui
                        .selectable_label(is_selected, &site_name)
                        .on_hover_text(&site_name);
                    
                    if site_response.clicked() {
                        self.selected_site = Some(site_name_clone1);
                    }
                    
                    ui.add_space(120.0); // 与表头间距对齐
                    
                    // 类型
                    ui.label(&type_text);
                    ui.add_space(120.0); // 与表头间距对齐
                    
                    // 端口 - 处理重定向显示
                    let display_port = if enable_https && enable_http_redirect {
                        format!("{}/80({})", port_text, redirect_text)
                    } else {
                        port_text.clone()
                    };
                    ui.label(display_port);
                    ui.add_space(120.0); // 与表头间距对齐
                    
                    // 域名 - 自适应剩余空间
                    ui.label(&server_name);
                    ui.add_space(160.0); // 与表头间距对齐
                    
                    // HTTPS
                    let https_text = if enable_https { &yes_text } else { &no_text };
                    ui.label(https_text);
                });
                
                // 右键点击检测 - 在response上检测
                if response.response.secondary_clicked() {
                    self.selected_site = Some(site_name_clone2);
                    self.show_context_menu = true;
                    self.context_menu_pos = ui.input(|i| i.pointer.hover_pos().unwrap_or_default());
                }

                // 双击编辑
                if response.response.double_clicked() {
                    self.edit_site(&site_name_for_edit);
                    return; // 退出函数避免继续使用已drop的数据
                }
            }

            // 右键菜单
            if self.show_context_menu {
                if let Some(site_name) = self.selected_site.clone() {
                    egui::Window::new("context_menu")
                        .title_bar(false)
                        .resizable(false)
                        .fixed_pos(self.context_menu_pos)
                        .show(ui.ctx(), |ui| {
                            ui.set_min_width(120.0);

                            let site_name_clone = site_name.clone();
                            if ui.button(self.translate("edit")).clicked() {
                                self.show_context_menu = false;
                                self.edit_site(&site_name_clone);
                            }

                            let site_name_clone = site_name.clone();
                            if ui.button(self.translate("delete")).clicked() {
                                self.show_context_menu = false;
                                self.delete_site(&site_name_clone);
                            }
                        });

                    // 点击其他地方关闭菜单
                    if ui.input(|i| i.pointer.any_click()) && !ui.ctx().is_pointer_over_area() {
                        self.show_context_menu = false;
                    }
                }
            }
        });
    }

    fn edit_site(&mut self, site_name: &str) {
        // TODO: 打开编辑对话框
        println!("Edit site: {}", site_name);
    }

    fn delete_site(&mut self, site_name: &str) {
        // 确认删除
        // TODO: 显示确认对话框
        println!("Delete site: {}", site_name);

        // 从列表中移除
        let mut sites = self.sites.write().unwrap();
        sites.retain(|s| s.site_name != site_name);

        // 清除选择
        if self.selected_site.as_ref() == Some(&site_name.to_string()) {
            self.selected_site = None;
        }
    }

    fn translate(&self, key: &str) -> String {
        self.language_manager.read().unwrap().get(key)
    }
}
