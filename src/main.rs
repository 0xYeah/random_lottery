mod app;
mod excel;
mod export;
mod model;
mod theme;
mod view;

use app::LotteryApp;
use iced::{Font, Size};

#[cfg(has_cjk_font)]
static CJK_FONT: &[u8] = include_bytes!("../assets/fonts/NotoSansSC-Regular.ttf");

fn main() -> iced::Result {
    let app = iced::application("随机抽奖系统", LotteryApp::update, LotteryApp::view)
        .subscription(LotteryApp::subscription)
        .window(iced::window::Settings {
            size: Size::new(1024.0, 680.0),
            min_size: Some(Size::new(800.0, 520.0)),
            ..Default::default()
        });

    #[cfg(has_cjk_font)]
    let app = app
        .font(CJK_FONT)
        .default_font(Font::with_name("Noto Sans SC"));

    #[cfg(not(has_cjk_font))]
    let app = app.default_font(platform_cjk_font());

    app.run()
}

#[cfg(not(has_cjk_font))]
fn platform_cjk_font() -> Font {
    #[cfg(target_os = "macos")]
    return Font::with_name("PingFang SC");
    #[cfg(target_os = "windows")]
    return Font::with_name("Microsoft YaHei");
    #[cfg(target_os = "linux")]
    return Font::with_name("Noto Sans CJK SC");
    #[allow(unreachable_code)]
    Font::DEFAULT
}
