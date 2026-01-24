mod core;
mod models;
mod ui;

use crate::core::LanguageManager;
use crate::ui::MainWindow;
use eframe::egui;
use std::sync::{Arc, RwLock};
use tracing::{error, info, warn};
use tracing_subscriber;

/// 设置中文字体 - 优化版，使用更美观的字体配置
fn setup_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();

    // 按优先级顺序使用更美观的中文字体
    // 优先级: 微软雅黑 > 黑体 > 宋体
    let preferred_fonts = [
        ("msyh".to_string(), "C:/Windows/Fonts/msyh.ttc"),      // 微软雅黑（更现代）
        ("simhei".to_string(), "C:/Windows/Fonts/simhei.ttf"),  // 黑体
        ("simsun".to_string(), "C:/Windows/Fonts/simsun.ttc"),  // 宋体
    ];

    let mut font_loaded = false;

    for (font_name, font_path) in &preferred_fonts {
        if std::path::Path::new(font_path).exists() {
            info!("Loading font: {} from {}", font_name, font_path);

            match std::fs::read(font_path) {
                Ok(font_data) => {
                    // 添加主字体 - 增大字体锐度，调整渲染参数
                    fonts.font_data.insert(
                        font_name.clone(),
                        egui::FontData::from_owned(font_data).tweak(egui::FontTweak {
                            scale: 1.1,                    // 稍微放大，使字体更清晰锐利
                            y_offset_factor: 0.0,          // 垂直偏移
                            y_offset: 0.0,
                            baseline_offset_factor: 0.0,   // 基线偏移
                        }),
                    );

                    // 配置 Proportional 字体族（UI 文本）
                    fonts
                        .families
                        .entry(egui::FontFamily::Proportional)
                        .or_default()
                        .insert(0, font_name.clone());

                    // 配置 Monospace 字体族（代码等宽文本）
                    // 优先使用 Consolas 等宽字体，如果没有则使用主字体
                    fonts.font_data.insert(
                        "consolas".to_string(),
                        egui::FontData::from_static(include_bytes!("C:/Windows/Fonts/consola.ttf")));

                    fonts
                        .families
                        .entry(egui::FontFamily::Monospace)
                        .or_default()
                        .insert(0, "consolas".to_string());
                    fonts
                        .families
                        .entry(egui::FontFamily::Monospace)
                        .or_default()
                        .push(font_name.clone());

                    font_loaded = true;
                    info!("Font loaded successfully: {}", font_name);
                    break; // 成功加载后跳出
                }
                Err(e) => {
                    warn!("Failed to read font {}: {}", font_path, e);
                }
            }
        } else {
            info!("Font not found: {}", font_path);
        }
    }

    if !font_loaded {
        warn!("No Chinese fonts found, using egui default fonts");
    }

    // 应用字体配置
    ctx.set_fonts(fonts);
    info!("Font configuration applied");
}

/// 设置 UI 样式 - 让界面更美观现代
fn setup_style(ctx: &egui::Context) {
    let mut style = (*ctx.style()).clone();

    // 调整字体大小 - 增大字体，提高可读性
    style.text_styles = [
        (egui::TextStyle::Heading, egui::FontId::new(26.0, egui::FontFamily::Proportional)),
        (egui::TextStyle::Body, egui::FontId::new(16.0, egui::FontFamily::Proportional)),
        (egui::TextStyle::Monospace, egui::FontId::new(15.0, egui::FontFamily::Monospace)),
        (egui::TextStyle::Button, egui::FontId::new(16.0, egui::FontFamily::Proportional)),
        (egui::TextStyle::Small, egui::FontId::new(14.0, egui::FontFamily::Proportional)),
    ]
    .into();

    // 应用样式
    ctx.set_style(style);
    info!("UI style applied");
}

/// easyNginx 应用程序
struct EasyNginxApp {
    language_manager: Arc<RwLock<LanguageManager>>,
    main_window: MainWindow,
}

impl EasyNginxApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // 配置字体（必须在创建任何UI之前）
        setup_fonts(&cc.egui_ctx);
        setup_style(&cc.egui_ctx);

        // 初始化语言管理器
        let language_manager = Arc::new(RwLock::new(LanguageManager::new()));

        // 创建主窗口
        let main_window = MainWindow::new(language_manager.clone());

        Self {
            language_manager,
            main_window,
        }
    }
}

impl eframe::App for EasyNginxApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        // 渲染主窗口
        self.main_window.ui(ctx, frame);

        // 请求连续渲染（响应UI更新）
        ctx.request_repaint_after(std::time::Duration::from_millis(100));
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        info!("easyNginx application exiting");
    }
}

fn main() -> anyhow::Result<()> {
    // 初始化日志
    tracing_subscriber::fmt::init();

    info!("Starting easyNginx application...");

    // 配置原生选项
    let native_options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_title("easyNginx")
            .with_inner_size([1000.0, 700.0])
            .with_min_inner_size([800.0, 600.0])
            .with_icon(load_icon()),
        ..Default::default()
    };

    // 运行应用程序
    eframe::run_native(
        "easyNginx",
        native_options,
        Box::new(|cc| Ok(Box::new(EasyNginxApp::new(cc)))),
    )
    .map_err(|e| {
        error!("Failed to run application: {}", e);
        anyhow::anyhow!("Application error: {}", e)
    })
}

/// 加载应用程序图标
fn load_icon() -> egui::IconData {
    // 使用内置的默认图标（绿色背景，白色 N 字母）
    // 16x16 的简单图标
    let rgba = vec![
        // 第1行
        0, 128, 0, 255, 0, 128, 0, 255, 0, 128, 0, 255, 0, 128, 0, 255, 0, 128, 0, 255, 0, 128, 0,
        255, 0, 128, 0, 255, 0, 128, 0, 255, 0, 128, 0, 255, 0, 128, 0, 255, 0, 128, 0, 255, 0,
        128, 0, 255, 0, 128, 0, 255, 0, 128, 0, 255, 0, 128, 0, 255, 0, 128, 0, 255,
        // 第2行
        0, 128, 0, 255, 0, 128, 0, 255, 0, 128, 0, 255, 0, 128, 0, 255, 0, 128, 0, 255, 0, 128,
        0, 255, 0, 128, 0, 255, 0, 128, 0, 255, 0, 128, 0, 255, 0, 128, 0, 255, 0, 128, 0, 255,
        0, 128, 0, 255, 0, 128, 0, 255, 0, 128, 0, 255, 0, 128, 0, 255, 0, 128, 0, 255, 0, 128,
        0, 255,
        // 第3-14行（内部绿色区域）
        0, 128, 0, 255, 0, 128, 0, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
        255, 255, 255, 255, 0, 128, 0, 255, 0, 128, 0, 255, 0, 128, 0, 255, 0, 128, 0, 255, 255,
        255, 255, 255, 0, 128, 0, 255, 0, 128, 0, 255, 0, 128, 0, 255, 0, 128, 0, 255, 0, 128,
        0, 255, 255, 255, 255, 255, 0, 128, 0, 255, 255, 255, 255, 255, 0, 128, 0, 255, 0, 128,
        0, 255, 255, 255, 255, 255, 255, 255, 255, 255, 0, 128, 0, 255, 0, 128, 0, 255, 255, 255,
        255, 255, 0, 128, 0, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 0, 128, 0,
        255, 0, 128, 0, 255, 255, 255, 255, 255, 0, 128, 0, 255, 0, 128, 0, 255, 255, 255, 255, 255,
        255, 255, 255, 255, 0, 128, 0, 255, 0, 128, 0, 255, 255, 255, 255, 255, 255, 255, 255, 255,
        255, 255, 255, 255, 255, 255, 255, 255, 0, 128, 0, 255, 0, 128, 0, 255, 255, 255, 255, 255,
        0, 128, 0, 255, 0, 128, 0, 255, 255, 255, 255, 255, 255, 255, 255, 255, 0, 128, 0, 255,
        255, 255, 255, 255, 255, 255, 255, 255, 0, 128, 0, 255, 0, 128, 0, 255, 255, 255, 255, 255,
        255, 255, 255, 255, 0, 128, 0, 255, 255, 255, 255, 255, 0, 128, 0, 255, 255, 255, 255, 255,
        0, 128, 0, 255, 0, 128, 0, 255, 255, 255, 255, 255, 255, 255, 0, 255, 0, 128, 0, 255,
        // 第15行
        0, 128, 0, 255, 0, 128, 0, 255, 0, 128, 0, 255, 0, 128, 0, 255, 0, 128, 0, 255, 0, 128,
        0, 255, 0, 128, 0, 255, 0, 128, 0, 255, 0, 128, 0, 255, 0, 128, 0, 255, 0, 128, 0, 255,
        0, 128, 0, 255, 0, 128, 0, 255, 0, 128, 0, 255, 0, 128, 0, 255, 0, 128, 0, 255, 0, 128,
        0, 255, 0, 128, 0, 255, 0, 128, 0, 255, 0, 128, 0, 255, 0, 128, 0, 255,
        // 第16行
        0, 128, 0, 255, 0, 128, 0, 255, 0, 128, 0, 255, 0, 128, 0, 255, 0, 128, 0, 255, 0, 128,
        0, 255, 0, 128, 0, 255, 0, 128, 0, 255, 0, 128, 0, 255, 0, 128, 0, 255, 0, 128, 0, 255,
        0, 128, 0, 255, 0, 128, 0, 255, 0, 128, 0, 255, 0, 128, 0, 255, 0, 128, 0, 255, 0, 128,
        0, 255, 0, 128, 0, 255, 0, 128, 0, 255, 0, 128, 0, 255, 0, 128, 0, 255,
    ];

    egui::IconData {
        rgba: rgba,
        width: 16,
        height: 16,
    }
}
