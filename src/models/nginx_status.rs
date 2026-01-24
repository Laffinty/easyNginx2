/// Nginx 运行状态
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NginxStatus {
    Stopped,
    Starting,
    Running,
    Stopping,
    Reloading,
}

impl NginxStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            NginxStatus::Stopped => "Stopped",
            NginxStatus::Starting => "Starting",
            NginxStatus::Running => "Running",
            NginxStatus::Stopping => "Stopping",
            NginxStatus::Reloading => "Reloading",
        }
    }

    pub fn display_name(&self, language: &crate::core::LanguageCode) -> &'static str {
        match (self, language) {
            (NginxStatus::Stopped, crate::core::LanguageCode::English) => "Stopped",
            (NginxStatus::Stopped, crate::core::LanguageCode::SimplifiedChinese) => "已停止",
            (NginxStatus::Starting, crate::core::LanguageCode::English) => "Starting",
            (NginxStatus::Starting, crate::core::LanguageCode::SimplifiedChinese) => "启动中",
            (NginxStatus::Running, crate::core::LanguageCode::English) => "Running",
            (NginxStatus::Running, crate::core::LanguageCode::SimplifiedChinese) => "运行中",
            (NginxStatus::Stopping, crate::core::LanguageCode::English) => "Stopping",
            (NginxStatus::Stopping, crate::core::LanguageCode::SimplifiedChinese) => "停止中",
            (NginxStatus::Reloading, crate::core::LanguageCode::English) => "Reloading",
            (NginxStatus::Reloading, crate::core::LanguageCode::SimplifiedChinese) => "重载中",
        }
    }

    pub fn is_running(&self) -> bool {
        matches!(self, NginxStatus::Running | NginxStatus::Starting | NginxStatus::Reloading)
    }

    pub fn is_stopped(&self) -> bool {
        matches!(self, NginxStatus::Stopped)
    }
}

impl Default for NginxStatus {
    fn default() -> Self {
        NginxStatus::Stopped
    }
}
