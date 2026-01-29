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

use async_trait::async_trait;
use std::any::{Any, TypeId};
use std::sync::Arc;
use tokio::sync::RwLock;
use crate::{MessageEnvelope, MessageBus, Module, module_init};

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum Language {
    English,
    ChineseSimplified,
}

impl Default for Language {
    fn default() -> Self {
        Language::ChineseSimplified
    }
}

#[derive(Clone, Debug)]
pub struct TranslationRequest {
    pub key: String,
    pub language: Language,
}

impl TranslationRequest {
    pub fn new(key: &str, language: Language) -> Self {
        Self {
            key: key.to_string(),
            language,
        }
    }
}

impl crate::Message for TranslationRequest {
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn message_type(&self) -> TypeId {
        TypeId::of::<TranslationRequest>()
    }
    
    fn clone_box(&self) -> Box<dyn crate::Message> {
        Box::new(self.clone())
    }
}

#[derive(Clone, Debug)]
pub struct TranslationResponse {
    pub key: String,
    pub translation: String,
    pub language: Language,
}

impl TranslationResponse {
    pub fn new(key: &str, translation: &str, language: Language) -> Self {
        Self {
            key: key.to_string(),
            translation: translation.to_string(),
            language,
        }
    }
}

impl crate::Message for TranslationResponse {
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn message_type(&self) -> TypeId {
        TypeId::of::<TranslationResponse>()
    }
    
    fn clone_box(&self) -> Box<dyn crate::Message> {
        Box::new(self.clone())
    }
}

#[derive(Clone, Debug)]
pub struct LanguageChangeRequest {
    pub language: Language,
}

impl LanguageChangeRequest {
    pub fn new(language: Language) -> Self {
        Self {
            language,
        }
    }
}

impl crate::Message for LanguageChangeRequest {
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn message_type(&self) -> TypeId {
        TypeId::of::<LanguageChangeRequest>()
    }
    
    fn clone_box(&self) -> Box<dyn crate::Message> {
        Box::new(self.clone())
    }
}

pub struct I18nModule {
    name: &'static str,
    bus: Arc<RwLock<Option<Arc<MessageBus>>>>,
    current_language: Arc<RwLock<Language>>,
    translations: Arc<RwLock<HashMap<(String, Language), String>>>,
}

use std::collections::HashMap;

impl I18nModule {
    pub fn new() -> Self {
        let mut translations = HashMap::new();
        
        // English translations
        translations.insert(("menu_file".to_string(), Language::English), "File".to_string());
        translations.insert(("menu_operation".to_string(), Language::English), "Operation".to_string());
        translations.insert(("menu_language".to_string(), Language::English), "Language".to_string());
        translations.insert(("menu_help".to_string(), Language::English), "Help".to_string());
        translations.insert(("menu_takeover_nginx".to_string(), Language::English), "Takeover Nginx".to_string());
        translations.insert(("menu_startup_on_boot".to_string(), Language::English), "Startup on Boot".to_string());
        translations.insert(("menu_new_proxy".to_string(), Language::English), "New Proxy".to_string());
        translations.insert(("menu_new_php".to_string(), Language::English), "New PHP".to_string());
        translations.insert(("menu_new_static".to_string(), Language::English), "New Static".to_string());
        translations.insert(("menu_exit".to_string(), Language::English), "Exit".to_string());
        translations.insert(("menu_start_nginx".to_string(), Language::English), "Start Nginx".to_string());
        translations.insert(("menu_stop_nginx".to_string(), Language::English), "Stop Nginx".to_string());
        translations.insert(("menu_reload_config".to_string(), Language::English), "Reload Config".to_string());
        translations.insert(("menu_refresh_sites".to_string(), Language::English), "Refresh Sites".to_string());
        translations.insert(("menu_test_config".to_string(), Language::English), "Test Config".to_string());
        translations.insert(("menu_backup_config".to_string(), Language::English), "Backup Config".to_string());
        translations.insert(("menu_english".to_string(), Language::English), "English".to_string());
        translations.insert(("menu_chinese".to_string(), Language::English), "Chinese".to_string());
        translations.insert(("menu_about".to_string(), Language::English), "About".to_string());
        translations.insert(("site_list_site".to_string(), Language::English), "Site".to_string());
        translations.insert(("site_list_type".to_string(), Language::English), "Type".to_string());
        translations.insert(("site_list_port".to_string(), Language::English), "Port".to_string());
        translations.insert(("site_list_domain".to_string(), Language::English), "Domain".to_string());
        translations.insert(("site_list_https".to_string(), Language::English), "HTTPS".to_string());
        translations.insert(("site_list_edit".to_string(), Language::English), "Edit".to_string());
        translations.insert(("site_list_delete".to_string(), Language::English), "Delete".to_string());
        translations.insert(("status_nginx_stopped".to_string(), Language::English), "Nginx: Stopped".to_string());
        translations.insert(("status_nginx_running".to_string(), Language::English), "Nginx: Running".to_string());
        translations.insert(("status_sites".to_string(), Language::English), "Sites: Total {total}, Static {static}, PHP {php}, Proxy {proxy}".to_string());
        translations.insert(("about_title".to_string(), Language::English), "About".to_string());
        translations.insert(("about_app_name".to_string(), Language::English), "easyNginx".to_string());
        translations.insert(("about_version".to_string(), Language::English), "Version 1.0.0".to_string());
        translations.insert(("about_description".to_string(), Language::English), "A simple Nginx management tool".to_string());
        translations.insert(("about_ok".to_string(), Language::English), "OK".to_string());
        
        // Chinese Simplified translations
        translations.insert(("menu_file".to_string(), Language::ChineseSimplified), "文件".to_string());
        translations.insert(("menu_operation".to_string(), Language::ChineseSimplified), "操作".to_string());
        translations.insert(("menu_language".to_string(), Language::ChineseSimplified), "语言".to_string());
        translations.insert(("menu_help".to_string(), Language::ChineseSimplified), "帮助".to_string());
        translations.insert(("menu_takeover_nginx".to_string(), Language::ChineseSimplified), "接管 Nginx".to_string());
        translations.insert(("menu_startup_on_boot".to_string(), Language::ChineseSimplified), "开机启动".to_string());
        translations.insert(("menu_new_proxy".to_string(), Language::ChineseSimplified), "新建代理".to_string());
        translations.insert(("menu_new_php".to_string(), Language::ChineseSimplified), "新建 PHP".to_string());
        translations.insert(("menu_new_static".to_string(), Language::ChineseSimplified), "新建静态".to_string());
        translations.insert(("menu_exit".to_string(), Language::ChineseSimplified), "退出".to_string());
        translations.insert(("menu_start_nginx".to_string(), Language::ChineseSimplified), "启动 Nginx".to_string());
        translations.insert(("menu_stop_nginx".to_string(), Language::ChineseSimplified), "停止 Nginx".to_string());
        translations.insert(("menu_reload_config".to_string(), Language::ChineseSimplified), "重载配置".to_string());
        translations.insert(("menu_refresh_sites".to_string(), Language::ChineseSimplified), "刷新站点".to_string());
        translations.insert(("menu_test_config".to_string(), Language::ChineseSimplified), "测试配置".to_string());
        translations.insert(("menu_backup_config".to_string(), Language::ChineseSimplified), "备份配置".to_string());
        translations.insert(("menu_english".to_string(), Language::ChineseSimplified), "English".to_string());
        translations.insert(("menu_chinese".to_string(), Language::ChineseSimplified), "中文".to_string());
        translations.insert(("menu_about".to_string(), Language::ChineseSimplified), "关于".to_string());
        translations.insert(("site_list_site".to_string(), Language::ChineseSimplified), "站点".to_string());
        translations.insert(("site_list_type".to_string(), Language::ChineseSimplified), "类型".to_string());
        translations.insert(("site_list_port".to_string(), Language::ChineseSimplified), "端口".to_string());
        translations.insert(("site_list_domain".to_string(), Language::ChineseSimplified), "域名".to_string());
        translations.insert(("site_list_https".to_string(), Language::ChineseSimplified), "HTTPS".to_string());
        translations.insert(("site_list_edit".to_string(), Language::ChineseSimplified), "编辑".to_string());
        translations.insert(("site_list_delete".to_string(), Language::ChineseSimplified), "删除".to_string());
        translations.insert(("status_nginx_stopped".to_string(), Language::ChineseSimplified), "Nginx: 已停止".to_string());
        translations.insert(("status_nginx_running".to_string(), Language::ChineseSimplified), "Nginx: 运行中".to_string());
        translations.insert(("status_sites".to_string(), Language::ChineseSimplified), "站点: 总计 {total}, 静态 {static}, PHP {php}, 代理 {proxy}".to_string());
        translations.insert(("about_title".to_string(), Language::ChineseSimplified), "关于".to_string());
        translations.insert(("about_app_name".to_string(), Language::ChineseSimplified), "easyNginx".to_string());
        translations.insert(("about_version".to_string(), Language::ChineseSimplified), "版本 1.0.0".to_string());
        translations.insert(("about_description".to_string(), Language::ChineseSimplified), "简单的 Nginx 管理工具".to_string());
        translations.insert(("about_ok".to_string(), Language::ChineseSimplified), "确定".to_string());
        
        Self {
            name: "l18n",
            bus: Arc::new(RwLock::new(None)),
            current_language: Arc::new(RwLock::new(Language::ChineseSimplified)),
            translations: Arc::new(RwLock::new(translations)),
        }
    }
    
    async fn translate(&self, key: &str, language: Option<Language>) -> String {
        let lang = match language {
            Some(l) => l,
            None => *self.current_language.read().await,
        };
        let translations = self.translations.read().await;
        
        if let Some(translation) = translations.get(&(key.to_string(), lang)) {
            translation.clone()
        } else {
            key.to_string()
        }
    }
    
    async fn set_language(&self, language: Language) {
        *self.current_language.write().await = language;
    }
    
    async fn get_current_language(&self) -> Language {
        *self.current_language.read().await
    }
}

impl Default for I18nModule {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Module for I18nModule {
    fn name(&self) -> &'static str {
        self.name
    }
    
    async fn initialize(&mut self, bus: Arc<MessageBus>) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        *self.bus.write().await = Some(bus.clone());
        
        // Register message types
        let translation_request_type = bus.register_message_type::<TranslationRequest>().await;
        let language_change_request_type = bus.register_message_type::<LanguageChangeRequest>().await;
        
        // Subscribe to messages
        bus.subscribe(translation_request_type, self.name().to_string()).await;
        bus.subscribe(language_change_request_type, self.name().to_string()).await;
        
        Ok(())
    }
    
    async fn process_message(&self, envelope: MessageEnvelope) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        if envelope.message_type == TypeId::of::<TranslationRequest>() {
            if let Some(msg) = envelope.payload.as_any().downcast_ref::<TranslationRequest>() {
                let translation = self.translate(&msg.key, Some(msg.language)).await;
                let response = TranslationResponse::new(&msg.key, &translation, msg.language);
                
                if let Some(bus) = &*self.bus.read().await {
                    bus.publish(response).await?;
                }
            }
        } else if envelope.message_type == TypeId::of::<LanguageChangeRequest>() {
            if let Some(msg) = envelope.payload.as_any().downcast_ref::<LanguageChangeRequest>() {
                self.set_language(msg.language).await;
                println!("[I18n] Language changed to: {:?}", msg.language);
            }
        }
        
        Ok(())
    }
    
    async fn shutdown(&mut self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        Ok(())
    }
}

module_init!(I18nModule, "l18n");
