use serde::{Deserialize, Serialize};

/// 站点类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SiteType {
    #[serde(rename = "static")]
    Static,
    #[serde(rename = "php")]
    Php,
    #[serde(rename = "proxy")]
    Proxy,
}

impl SiteType {
    pub fn as_str(&self) -> &'static str {
        match self {
            SiteType::Static => "static",
            SiteType::Php => "php",
            SiteType::Proxy => "proxy",
        }
    }

    pub fn display_name(&self, language: &crate::core::LanguageCode) -> &'static str {
        match (self, language) {
            (SiteType::Static, crate::core::LanguageCode::English) => "Static",
            (SiteType::Static, crate::core::LanguageCode::SimplifiedChinese) => "静态",
            (SiteType::Php, crate::core::LanguageCode::English) => "PHP",
            (SiteType::Php, crate::core::LanguageCode::SimplifiedChinese) => "PHP",
            (SiteType::Proxy, crate::core::LanguageCode::English) => "Proxy",
            (SiteType::Proxy, crate::core::LanguageCode::SimplifiedChinese) => "代理",
        }
    }
}

/// 站点基本配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SiteConfig {
    pub site_name: String,
    pub site_type: SiteType,
    pub server_name: String,
    pub listen_port: u16,
    pub root_path: String,
    pub enable_https: bool,
    pub enable_http_redirect: bool,
    /// 静态站点: 无额外配置
    /// PHP站点: PHP版本
    /// 代理站点: 代理地址
    pub extra_config: Option<String>,
}

impl SiteConfig {
    pub fn new_static(site_name: String, server_name: String, root_path: String) -> Self {
        Self {
            site_name,
            site_type: SiteType::Static,
            server_name,
            listen_port: 80,
            root_path,
            enable_https: false,
            enable_http_redirect: false,
            extra_config: None,
        }
    }

    pub fn new_php(site_name: String, server_name: String, root_path: String, php_version: String) -> Self {
        Self {
            site_name,
            site_type: SiteType::Php,
            server_name,
            listen_port: 80,
            root_path,
            enable_https: false,
            enable_http_redirect: false,
            extra_config: Some(php_version),
        }
    }

    pub fn new_proxy(site_name: String, server_name: String, proxy_url: String) -> Self {
        Self {
            site_name,
            site_type: SiteType::Proxy,
            server_name,
            listen_port: 80,
            root_path: String::new(),
            enable_https: false,
            enable_http_redirect: false,
            extra_config: Some(proxy_url),
        }
    }
}

/// 站点列表项（用于显示）
#[derive(Debug, Clone)]
pub struct SiteListItem {
    pub site_name: String,
    pub site_type: SiteType,
    pub server_name: String,
    pub listen_port: u16,
    pub enable_https: bool,
    pub enable_http_redirect: bool,
}

impl SiteListItem {
    pub fn from_config(config: &SiteConfig) -> Self {
        Self {
            site_name: config.site_name.clone(),
            site_type: config.site_type,
            server_name: config.server_name.clone(),
            listen_port: config.listen_port,
            enable_https: config.enable_https,
            enable_http_redirect: config.enable_http_redirect,
        }
    }

    pub fn get_display_name(&self) -> String {
        self.site_name.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_site_type_display() {
        let lang = crate::core::LanguageCode::English;
        assert_eq!(SiteType::Static.display_name(&lang), "Static");
        assert_eq!(SiteType::Php.display_name(&lang), "PHP");
        assert_eq!(SiteType::Proxy.display_name(&lang), "Proxy");
    }

    #[test]
    fn test_site_config_creation() {
        let config = SiteConfig::new_static(
            "test".to_string(),
            "example.com".to_string(),
            "/var/www".to_string(),
        );
        assert_eq!(config.site_type, SiteType::Static);
        assert_eq!(config.listen_port, 80);
    }
}
