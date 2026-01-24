use crate::core::LanguageManager;
use crate::models::{NginxStatus, SiteConfig, SiteListItem, SiteType};
use crate::ui::site_list::SiteListPanel;
use eframe::egui;
use std::sync::{Arc, RwLock};

#[cfg(windows)]
use windows::Win32::UI::WindowsAndMessaging::{MessageBoxW, MB_ICONINFORMATION, MB_OK};

pub struct MainWindow {
    language_manager: Arc<RwLock<LanguageManager>>,
    nginx_status: Arc<RwLock<NginxStatus>>,
    sites: Arc<RwLock<Vec<SiteListItem>>>,
    site_list_panel: SiteListPanel,
    show_about: bool,
    show_language_menu: bool,
}

impl MainWindow {
    pub fn new(language_manager: Arc<RwLock<LanguageManager>>) -> Self {
        let nginx_status = Arc::new(RwLock::new(NginxStatus::Stopped));
        let sites = Arc::new(RwLock::new(Vec::new()));

        // 添加一些示例数据
        {
            let mut sites_lock = sites.write().unwrap();
            sites_lock.push(SiteListItem::from_config(&SiteConfig::new_static(
                "example-static".to_string(),
                "static.example.com".to_string(),
                "/var/www/static".to_string(),
            )));
            sites_lock.push(SiteListItem::from_config(&SiteConfig::new_php(
                "example-php".to_string(),
                "php.example.com".to_string(),
                "/var/www/php".to_string(),
                "8.1".to_string(),
            )));
            sites_lock.push(SiteListItem::from_config(&SiteConfig::new_proxy(
                "example-proxy".to_string(),
                "proxy.example.com".to_string(),
                "http://localhost:3000".to_string(),
            )));
        }

        Self {
            language_manager: language_manager.clone(),
            nginx_status: nginx_status.clone(),
            sites: sites.clone(),
            site_list_panel: SiteListPanel::new(language_manager.clone(), sites, nginx_status),
            show_about: false,
            show_language_menu: false,
        }
    }

    pub fn ui(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // 顶部菜单栏 - 增加高度，移除背景色
        egui::TopBottomPanel::top("menu_bar")
            .exact_height(36.0)  // 增加菜单栏高度
            .show(ctx, |ui| {
                // 移除菜单栏背景色
                ui.style_mut().visuals.widgets.noninteractive.bg_fill = egui::Color32::TRANSPARENT;
                ui.style_mut().visuals.widgets.inactive.bg_fill = egui::Color32::TRANSPARENT;
                self.render_menu_bar(ui);
            });

        // 中央内容区域
        egui::CentralPanel::default().show(ctx, |ui| {
            self.site_list_panel.ui(ui);
        });

        // 底部状态栏
        egui::TopBottomPanel::bottom("status_bar").show(ctx, |ui| {
            self.render_status_bar(ui);
        });

        // 关于对话框 - 使用Windows标准MessageBox
        if self.show_about {
            self.show_about = false; // 立即关闭标志
            
            #[cfg(windows)]
            unsafe {
                // 转换字符串为宽字节
                let title = self.translate("about_title");
                let content = self.translate("about_content");
                
                let title_wide: Vec<u16> = title.encode_utf16().chain(std::iter::once(0)).collect();
                let content_wide: Vec<u16> = content.encode_utf16().chain(std::iter::once(0)).collect();
                
                // 显示Windows标准消息框 (使用NULL作为父窗口)
                MessageBoxW(
                    windows::Win32::Foundation::HWND(std::ptr::null_mut()), // NULL handle
                    windows::core::PCWSTR(content_wide.as_ptr()),
                    windows::core::PCWSTR(title_wide.as_ptr()),
                    MB_OK | MB_ICONINFORMATION,
                );
            }
            
            #[cfg(not(windows))]
            {
                // 非Windows平台仍使用egui内建对话框
                egui::Window::new(self.translate("about_title"))
                    .collapsible(false)
                    .resizable(false)
                    .show(ctx, |ui| {
                        ui.label(self.translate("about_content"));
                        if ui.button("OK").clicked() {
                            self.show_about = false;
                        }
                    });
            }
        }
    }

    fn render_menu_bar(&mut self, ui: &mut egui::Ui) {
        // 设置垂直居中布局
        ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
            // 文件菜单
            ui.menu_button(self.translate("file_menu"), |ui| {
                if ui.button(self.translate("takeover_nginx")).clicked() {
                    ui.close_menu();
                    // TODO: 接管 Nginx
                }

                if ui.button(self.translate("startup_on_boot")).clicked() {
                    ui.close_menu();
                    // TODO: 切换开机自启
                }

                ui.separator();

                if ui.button(self.translate("new_proxy")).clicked() {
                    ui.close_menu();
                    // TODO: 新建代理站点
                }

                if ui.button(self.translate("new_php")).clicked() {
                    ui.close_menu();
                    // TODO: 新建 PHP 站点
                }

                if ui.button(self.translate("new_static")).clicked() {
                    ui.close_menu();
                    // TODO: 新建静态站点
                }

                ui.separator();

                if ui.button(self.translate("exit")).clicked() {
                    ui.close_menu();
                    std::process::exit(0);
                }
            });

            // 操作菜单
            ui.menu_button(self.translate("operation_menu"), |ui| {
                if ui.button(self.translate("start_nginx")).clicked() {
                    ui.close_menu();
                    // TODO: 启动 Nginx
                    if let Ok(mut status) = self.nginx_status.write() {
                        *status = NginxStatus::Starting;
                    }
                }

                if ui.button(self.translate("stop_nginx")).clicked() {
                    ui.close_menu();
                    // TODO: 停止 Nginx
                    if let Ok(mut status) = self.nginx_status.write() {
                        *status = NginxStatus::Stopping;
                    }
                }

                if ui.button(self.translate("reload_config")).clicked() {
                    ui.close_menu();
                    // TODO: 重载配置
                    if let Ok(mut status) = self.nginx_status.write() {
                        *status = NginxStatus::Reloading;
                    }
                }

                ui.separator();

                if ui.button(self.translate("refresh_sites")).clicked() {
                    ui.close_menu();
                    // TODO: 刷新站点
                }

                ui.separator();

                if ui.button(self.translate("test_config")).clicked() {
                    ui.close_menu();
                    // TODO: 测试配置
                }

                if ui.button(self.translate("backup_config")).clicked() {
                    ui.close_menu();
                    // TODO: 备份配置
                }
            });

            // 语言菜单
            ui.menu_button(self.translate("language_menu"), |ui| {
                let current_lang = self.language_manager.read().unwrap().current_language();
                let supported_langs = self
                    .language_manager
                    .read()
                    .unwrap()
                    .supported_languages();

                for (lang_code, display_name) in supported_langs {
                    let selected = lang_code == current_lang;
                    if ui.radio(selected, display_name).clicked() {
                        ui.close_menu();
                        self.language_manager.write().unwrap().set_language(lang_code);
                        // 重新加载 UI（egui 会自动重绘）
                    }
                }
            });

            // 帮助菜单
            ui.menu_button(self.translate("help_menu"), |ui| {
                if ui.button(self.translate("about")).clicked() {
                    ui.close_menu();
                    self.show_about = true;
                }
            });
        });
    }

    fn render_status_bar(&self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            let status = self.nginx_status.read().unwrap();
            let lang = self.language_manager.read().unwrap().current_language();
            let status_text = format!("Nginx: {}", status.display_name(&lang));
            ui.label(status_text);

            ui.separator();

            // 站点统计
            let sites = self.sites.read().unwrap();
            let total = sites.len();
            let static_count = sites.iter().filter(|s| s.site_type == SiteType::Static).count();
            let php_count = sites.iter().filter(|s| s.site_type == SiteType::Php).count();
            let proxy_count = sites.iter().filter(|s| s.site_type == SiteType::Proxy).count();

            let args = serde_json::json!({
                "total": total,
                "static": static_count,
                "php": php_count,
                "proxy": proxy_count
            });
            let stats_text = self
                .language_manager
                .read()
                .unwrap()
                .get_with_args("total_sites", &args);
            ui.label(stats_text);

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.label(format!("easyNginx v{}", env!("CARGO_PKG_VERSION")));
            });
        });
    }

    fn translate(&self, key: &str) -> String {
        self.language_manager.read().unwrap().get(key)
    }
}
