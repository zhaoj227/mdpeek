//! 极轻量的 Windows Markdown 单文件阅读器
//! 用法：mdreader.exe <file.md>，或在资源管理器中通过「打开方式」选择本程序。
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::path::PathBuf;

use eframe::egui;
use egui_commonmark::{CommonMarkCache, CommonMarkViewer};

fn main() -> eframe::Result {
    let path = std::env::args().nth(1).map(PathBuf::from);

    let (markdown, title) = match path {
        Some(p) if p.is_file() => match std::fs::read_to_string(&p) {
            Ok(content) => {
                // 切到文件所在目录，让 markdown 里的相对路径图片可以正常加载
                if let Some(dir) = p.parent() {
                    let _ = std::env::set_current_dir(dir);
                }
                let title = p
                    .file_name()
                    .map(|n| n.to_string_lossy().to_string())
                    .unwrap_or_else(|| "Markdown 阅读器".to_owned());
                (content, title)
            }
            Err(e) => (
                format!("> 无法读取文件：{}\n>\n> {}", p.display(), e),
                "Markdown 阅读器".to_owned(),
            ),
        },
        Some(p) => (
            format!("> 未找到文件：{}", p.display()),
            "Markdown 阅读器".to_owned(),
        ),
        None => (
            "# Markdown 阅读器\n\n在资源管理器中右键 `.md` 文件 → **打开方式** → 选择 **Markdown 阅读器**。\n\n也可以通过命令行传入文件路径，或把文件拖到本窗口上。\n"
                .to_owned(),
            "Markdown 阅读器".to_owned(),
        ),
    };

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([880.0, 760.0])
            .with_min_inner_size([460.0, 360.0])
            .with_title(title),
        // 用 OpenGL 后端，内存占用比默认的 wgpu(DX12) 低很多
        renderer: eframe::Renderer::Glow,
        ..Default::default()
    };

    eframe::run_native(
        "mdreader",
        options,
        Box::new(|cc| {
            setup_fonts(&cc.egui_ctx);
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::new(App {
                cache: CommonMarkCache::default(),
                markdown,
            }))
        }),
    )
}

/// egui 默认字体不含中文，从 Windows 系统字体里加载一个中文字体。
fn setup_fonts(ctx: &egui::Context) {
    const CANDIDATES: [&str; 4] = [
        r"C:\Windows\Fonts\msyh.ttc",   // 微软雅黑
        r"C:\Windows\Fonts\msyhbd.ttc", // 微软雅黑粗体
        r"C:\Windows\Fonts\simhei.ttf", // 黑体
        r"C:\Windows\Fonts\simsun.ttc", // 宋体
    ];
    for candidate in CANDIDATES {
        if let Ok(bytes) = std::fs::read(candidate) {
            let mut fonts = egui::FontDefinitions::default();
            fonts
                .font_data
                .insert("cjk".to_owned(), egui::FontData::from_owned(bytes).into());
            for family in [egui::FontFamily::Proportional, egui::FontFamily::Monospace] {
                fonts.families.entry(family).or_default().push("cjk".to_owned());
            }
            ctx.set_fonts(fonts);
            return;
        }
    }
}

struct App {
    cache: CommonMarkCache,
    markdown: String,
}

impl eframe::App for App {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            egui::ScrollArea::vertical()
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    ui.add_space(14.0);
                    egui::Frame::default()
                        .inner_margin(egui::Margin::symmetric(24, 14))
                        .show(ui, |ui| {
                            CommonMarkViewer::new()
                                .default_width(Some(760))
                                .max_image_width(Some(820))
                                .show(ui, &mut self.cache, &self.markdown);
                        });
                    ui.add_space(30.0);
                });
        });
    }
}
