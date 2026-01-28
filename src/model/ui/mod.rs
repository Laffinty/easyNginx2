pub mod main_window;

use async_trait::async_trait;
use std::sync::Arc;
use std::error::Error;
use crate::{MessageEnvelope, MessageBus, Module, module_init};
use std::any::TypeId;
use tokio::sync::RwLock;
use std::sync::atomic::{AtomicBool, Ordering};
use eframe::egui;

#[derive(Clone)]
pub struct UiModule {
    name: &'static str,
    bus: Arc<RwLock<Option<Arc<MessageBus>>>>,
    is_running: Arc<AtomicBool>,
}

impl UiModule {
    pub fn new() -> Self {
        Self {
            name: "ui",
            bus: Arc::new(RwLock::new(None)),
            is_running: Arc::new(AtomicBool::new(false)),
        }
    }
}

impl Default for UiModule {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Module for UiModule {
    fn name(&self) -> &'static str {
        self.name
    }
    
    async fn initialize(&mut self, bus: Arc<MessageBus>) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        eprintln!("[UI Module] === INITIALIZATION START ===");
        
        *self.bus.write().await = Some(bus.clone());
        
        let is_running = self.is_running.clone();
        
        eprintln!("[UI Module] Starting GUI in spawn_blocking...");
        self.is_running.store(true, Ordering::SeqCst);
        
        let bus_for_exit = bus.clone();
        
        // Test if we can create a simple window without eframe first
        eprintln!("[UI Module] Testing basic console output...");
        
        // Spawn the GUI thread
        let gui_handle = tokio::task::spawn_blocking(move || {
            eprintln!("[GUI] === GUI THREAD START ===");
            
            // Test if we can print to stderr
            eprintln!("[GUI] Testing stderr output...");
            
            // Try a very simple eframe setup
            eprintln!("[GUI] Creating minimal eframe window...");
            
            // Create native options with Windows-specific any_thread support
            let mut native_options = eframe::NativeOptions {
                viewport: egui::ViewportBuilder::default()
                    .with_title("easyNginx Test")
                    .with_inner_size([1000.0, 700.0])
                    .with_resizable(true),
                ..Default::default()
            };
            
            // Enable any_thread support for Windows to allow creation on non-main thread
            #[cfg(windows)]
            {
                use winit::platform::windows::EventLoopBuilderExtWindows;
                native_options.event_loop_builder = Some(Box::new(|builder| {
                    builder.with_any_thread(true);
                }));
            }
            
            eprintln!("[GUI] Calling eframe::run_native...");
            
            // Use the original MainWindow
            let bus_for_window = bus_for_exit.clone();
            let result = eframe::run_native(
                "easyNginx",
                native_options,
                Box::new(|cc| {
                    eprintln!("[GUI] Creating MainWindow instance...");
                    
                    // 配置中文字体支持
                    eprintln!("[GUI] Configuring Chinese font support...");
                    
                    // 使用Windows系统内置的微软雅黑字体
                    eprintln!("[GUI] Using Windows system Microsoft YaHei font");
                    
                    let mut fonts = egui::FontDefinitions::default();
                    
                    // 添加微软雅黑字体（使用系统路径）
                    fonts.font_data.insert(
                        "microsoft_yahei".to_owned(),
                        egui::FontData::from_static(include_bytes!("C:/Windows/Fonts/msyh.ttc"))
                    );
                    
                    // 设置默认字体
                    let proportional_fonts = fonts.families.get_mut(&egui::FontFamily::Proportional)
                        .unwrap();
                    proportional_fonts.insert(0, "microsoft_yahei".to_owned());
                    
                    let monospace_fonts = fonts.families.get_mut(&egui::FontFamily::Monospace)
                        .unwrap();
                    monospace_fonts.insert(0, "microsoft_yahei".to_owned());
                    
                    // 应用字体配置
                    cc.egui_ctx.set_fonts(fonts);
                    eprintln!("[GUI] Font configuration applied");
                    
                    let window = main_window::create_main_window(Some(bus_for_window));
                    eprintln!("[GUI] MainWindow created successfully");
                    window
                }),
            );
            
            eprintln!("[GUI] eframe::run_native returned: {:?}", result);
            
            match result {
                Ok(()) => eprintln!("[GUI] GUI window closed normally"),
                Err(e) => {
                    eprintln!("[GUI] GUI error: {:?}", e);
                    // Try to print more details about the error
                    if let Some(cause) = e.source() {
                        eprintln!("[GUI] Error cause: {:?}", cause);
                    }
                }
            }
            
            is_running.store(false, Ordering::SeqCst);
            eprintln!("[GUI] === GUI THREAD END ===");
            
            // Signal exit
            match tokio::runtime::Handle::try_current() {
                Ok(rt) => {
                    rt.block_on(async {
                        eprintln!("[GUI] Signaling exit via existing runtime");
                        bus_for_exit.signal_exit().await;
                    });
                }
                Err(_) => {
                    let rt = tokio::runtime::Builder::new_current_thread()
                        .build()
                        .expect("Failed to build tokio runtime");
                    rt.block_on(async {
                        eprintln!("[GUI] Signaling exit via new runtime");
                        bus_for_exit.signal_exit().await;
                    });
                }
            }
        });
        
        // Wait a bit and check if the GUI task is running
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
        
        if gui_handle.is_finished() {
            eprintln!("[UI Module] WARNING: GUI task finished prematurely!");
            match gui_handle.await {
                Ok(_) => eprintln!("[UI Module] GUI task completed successfully"),
                Err(e) => eprintln!("[UI Module] GUI task panicked: {:?}", e),
            }
        } else {
            eprintln!("[UI Module] GUI task is still running");
        }
        
        eprintln!("[UI Module] === INITIALIZATION COMPLETE ===");
        Ok(())
    }
    
    async fn process_message(&self, envelope: MessageEnvelope) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        if envelope.message_type == TypeId::of::<crate::SystemMessage>() {
            if let Some(msg) = envelope.payload.as_any().downcast_ref::<crate::SystemMessage>() {
                println!("[UI Module] Received system message: {} - {}", msg.source, msg.content);
            }
        }
        
        Ok(())
    }
    
    async fn shutdown(&mut self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        println!("[UI Module] Shutting down...");
        
        self.is_running.store(false, Ordering::SeqCst);
        
        let mut attempts = 0;
        while self.is_running.load(Ordering::SeqCst) && attempts < 20 {
            tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
            attempts += 1;
        }
        
        println!("[UI Module] Shutdown complete");
        Ok(())
    }
}

module_init!(UiModule, "ui");
