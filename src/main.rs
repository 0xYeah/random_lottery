mod app;
mod excel;
mod export;
mod model;
mod theme;
mod view;

use app::LotteryApp;
use iced::Size;

fn main() -> iced::Result {
    iced::application("随机抽奖系统", LotteryApp::update, LotteryApp::view)
        .subscription(LotteryApp::subscription)
        .window(iced::window::Settings {
            size: Size::new(1024.0, 680.0),
            min_size: Some(Size::new(800.0, 520.0)),
            ..Default::default()
        })
        .run()
}
