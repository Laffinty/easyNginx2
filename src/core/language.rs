use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// 支持的语言
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LanguageCode {
    #[serde(rename = "en")]
    English,
    #[serde(rename = "zh-CN")]
    SimplifiedChinese,
}

impl Default for LanguageCode {
    fn default() -> Self {
        LanguageCode::English
    }
}

impl LanguageCode {
    pub fn as_str(&self) -> &'static str {
        match self {
            LanguageCode::English => "en",
            LanguageCode::SimplifiedChinese => "zh-CN",
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            LanguageCode::English => "English",
            LanguageCode::SimplifiedChinese => "简体中文",
        }
    }
}

/// 翻译条目
pub type Translations = HashMap<String, String>;

/// 语言管理器
pub struct LanguageManager {
    current_language: LanguageCode,
    translations: HashMap<LanguageCode, Translations>,
}

impl Default for LanguageManager {
    fn default() -> Self {
        Self::new()
    }
}

impl LanguageManager {
    pub fn new() -> Self {
        let mut manager = Self {
            current_language: LanguageCode::default(),
            translations: HashMap::new(),
        };

        // 加载内置的默认翻译
        manager.load_builtin_translations();
        manager
    }

    /// 从JSON文件加载翻译
    pub fn load_from_file<P: AsRef<Path>>(&mut self, path: P, language: LanguageCode) -> Result<()> {
        let content = fs::read_to_string(path.as_ref())
            .with_context(|| format!("Failed to read translation file: {:?}", path.as_ref()))?;

        let translations: Translations = serde_json::from_str(&content)
            .with_context(|| format!("Failed to parse JSON from {:?}", path.as_ref()))?;

        self.translations.insert(language, translations);
        Ok(())
    }

    /// 加载内置的默认翻译
    fn load_builtin_translations(&mut self) {
        // 英文翻译
        let en_translations: Translations = [
            ("file_menu".to_string(), "File".to_string()),
            ("takeover_nginx".to_string(), "Takeover Nginx".to_string()),
            ("startup_on_boot".to_string(), "Start on Boot".to_string()),
            ("new_proxy".to_string(), "New Proxy Site".to_string()),
            ("new_php".to_string(), "New PHP Site".to_string()),
            ("new_static".to_string(), "New Static Site".to_string()),
            ("exit".to_string(), "Exit".to_string()),
            ("operation_menu".to_string(), "Operation".to_string()),
            ("start_nginx".to_string(), "Start Nginx".to_string()),
            ("stop_nginx".to_string(), "Stop Nginx".to_string()),
            ("reload_config".to_string(), "Reload Config".to_string()),
            ("refresh_sites".to_string(), "Refresh Sites".to_string()),
            ("test_config".to_string(), "Test Config".to_string()),
            ("backup_config".to_string(), "Backup Config".to_string()),
            ("language_menu".to_string(), "Language".to_string()),
            ("help_menu".to_string(), "Help".to_string()),
            ("about".to_string(), "About".to_string()),
            ("site".to_string(), "Site".to_string()),
            ("type".to_string(), "Type".to_string()),
            ("port".to_string(), "Port".to_string()),
            ("domain".to_string(), "Domain".to_string()),
            ("static_site".to_string(), "Static".to_string()),
            ("php_site".to_string(), "PHP".to_string()),
            ("proxy_site".to_string(), "Proxy".to_string()),
            ("yes".to_string(), "Yes".to_string()),
            ("no".to_string(), "No".to_string()),
            ("redirect".to_string(), "redirect".to_string()),
            ("total_sites".to_string(), "Total: {total} (Static: {static}, PHP: {php}, Proxy: {proxy})".to_string()),
            ("edit".to_string(), "Edit".to_string()),
            ("delete".to_string(), "Delete".to_string()),
            ("confirm_delete".to_string(), "Confirm Delete".to_string()),
            ("delete_confirm_message".to_string(), "Are you sure you want to delete site '{name}'?".to_string()),
            ("operation_success".to_string(), "Success".to_string()),
            ("operation_failed".to_string(), "Failed".to_string()),
            ("error".to_string(), "Error".to_string()),
            ("preview_title".to_string(), "Config Preview".to_string()),
            ("update_preview".to_string(), "Update Preview".to_string()),
            ("copy_config".to_string(), "Copy Config".to_string()),
            ("about_title".to_string(), "About easyNginx".to_string()),
            ("about_content".to_string(), "easyNginx v1.0.0\n\nA professional Nginx management tool for Windows.\n\nMIT License".to_string()),
            ("show_main_window".to_string(), "Show Main Window".to_string()),
            ("confirm_exit".to_string(), "Confirm Exit".to_string()),
            ("exit_confirm_message".to_string(), "Are you sure you want to exit?".to_string()),
            ("takeover_completed".to_string(), "Takeover Completed".to_string()),
            ("takeover_restart_message".to_string(), "Nginx paths have been updated.".to_string()),
            ("please_restart_application".to_string(), "Please restart the application.".to_string()),
        ]
        .iter()
        .cloned()
        .collect();

        // 简体中文翻译
        let zh_translations: Translations = [
            ("file_menu".to_string(), "文件".to_string()),
            ("takeover_nginx".to_string(), "接管 Nginx".to_string()),
            ("startup_on_boot".to_string(), "开机自启".to_string()),
            ("new_proxy".to_string(), "新建代理站点".to_string()),
            ("new_php".to_string(), "新建 PHP 站点".to_string()),
            ("new_static".to_string(), "新建静态站点".to_string()),
            ("exit".to_string(), "退出".to_string()),
            ("operation_menu".to_string(), "操作".to_string()),
            ("start_nginx".to_string(), "启动 Nginx".to_string()),
            ("stop_nginx".to_string(), "停止 Nginx".to_string()),
            ("reload_config".to_string(), "重载配置".to_string()),
            ("refresh_sites".to_string(), "刷新站点".to_string()),
            ("test_config".to_string(), "测试配置".to_string()),
            ("backup_config".to_string(), "备份配置".to_string()),
            ("language_menu".to_string(), "语言".to_string()),
            ("help_menu".to_string(), "帮助".to_string()),
            ("about".to_string(), "关于".to_string()),
            ("site".to_string(), "站点".to_string()),
            ("type".to_string(), "类型".to_string()),
            ("port".to_string(), "端口".to_string()),
            ("domain".to_string(), "域名".to_string()),
            ("static_site".to_string(), "静态".to_string()),
            ("php_site".to_string(), "PHP".to_string()),
            ("proxy_site".to_string(), "代理".to_string()),
            ("yes".to_string(), "是".to_string()),
            ("no".to_string(), "否".to_string()),
            ("redirect".to_string(), "重定向".to_string()),
            ("total_sites".to_string(), "总计: {total} (静态: {static}, PHP: {php}, 代理: {proxy})".to_string()),
            ("edit".to_string(), "编辑".to_string()),
            ("delete".to_string(), "删除".to_string()),
            ("confirm_delete".to_string(), "确认删除".to_string()),
            ("delete_confirm_message".to_string(), "确定要删除站点 '{name}' 吗？".to_string()),
            ("operation_success".to_string(), "操作成功".to_string()),
            ("operation_failed".to_string(), "操作失败".to_string()),
            ("error".to_string(), "错误".to_string()),
            ("preview_title".to_string(), "配置预览".to_string()),
            ("update_preview".to_string(), "更新预览".to_string()),
            ("copy_config".to_string(), "复制配置".to_string()),
            ("about_title".to_string(), "关于 easyNginx".to_string()),
            ("about_content".to_string(), "easyNginx v1.0.0\n\n专业的 Windows Nginx 管理工具。\n\nMIT 许可证".to_string()),
            ("show_main_window".to_string(), "显示主窗口".to_string()),
            ("confirm_exit".to_string(), "确认退出".to_string()),
            ("exit_confirm_message".to_string(), "确定要退出程序吗？".to_string()),
            ("takeover_completed".to_string(), "接管完成".to_string()),
            ("takeover_restart_message".to_string(), "Nginx 路径已更新。".to_string()),
            ("please_restart_application".to_string(), "请重启应用程序。".to_string()),
        ]
        .iter()
        .cloned()
        .collect();

        self.translations.insert(LanguageCode::English, en_translations);
        self.translations.insert(LanguageCode::SimplifiedChinese, zh_translations);
    }

    /// 设置当前语言
    pub fn set_language(&mut self, language: LanguageCode) {
        self.current_language = language;
    }

    /// 获取当前语言
    pub fn current_language(&self) -> LanguageCode {
        self.current_language
    }

    /// 获取所有支持的语言
    pub fn supported_languages(&self) -> Vec<(LanguageCode, &'static str)> {
        vec![
            (LanguageCode::English, LanguageCode::English.display_name()),
            (LanguageCode::SimplifiedChinese, LanguageCode::SimplifiedChinese.display_name()),
        ]
    }

    /// 获取翻译文本
    pub fn get(&self, key: &str) -> String {
        self.get_simple(key)
    }

    /// 获取翻译文本并替换参数
    pub fn get_with_args<T: serde::Serialize>(&self, key: &str, args: &T) -> String {
        let translations = self.translations.get(&self.current_language).unwrap();

        let template = translations
            .get(key)
            .cloned()
            .unwrap_or_else(|| format!("[{}]", key));

        // 简单替换参数
        let mut result = template;
        if let Ok(args_map) = serde_json::to_value(args) {
            if let serde_json::Value::Object(map) = args_map {
                for (key, value) in map {
                    if let Some(str_val) = value.as_str() {
                        result = result.replace(&format!("{{{}}}", key), str_val);
                    } else if let Some(num_val) = value.as_u64() {
                        result = result.replace(&format!("{{{}}}", key), &num_val.to_string());
                    } else if let Some(num_val) = value.as_i64() {
                        result = result.replace(&format!("{{{}}}", key), &num_val.to_string());
                    }
                }
            }
        }

        result
    }

    /// 获取格式化后的翻译
    pub fn t(&self, key: &str) -> String {
        self.get(key)
    }

    /// 获取翻译文本（无参数版本）
    pub fn get_simple(&self, key: &str) -> String {
        self.get_with_args::<()>(key, &())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_language_manager() {
        let mut manager = LanguageManager::new();

        // 测试默认语言
        assert_eq!(manager.current_language(), LanguageCode::English);
        assert_eq!(manager.get("file_menu"), "File");

        // 测试切换语言
        manager.set_language(LanguageCode::SimplifiedChinese);
        assert_eq!(manager.current_language(), LanguageCode::SimplifiedChinese);
        assert_eq!(manager.get("file_menu"), "文件");

        // 测试参数替换
        let args = serde_json::json!({
            "total": 10,
            "static": 3,
            "php": 4,
            "proxy": 3
        });
        let result = manager.get_with_args("total_sites", &args);
        assert!(result.contains("10"));
        assert!(result.contains("3"));
        assert!(result.contains("4"));
    }
}
