use iced::widget::canvas::{self, Canvas, Frame, Text};
use iced::widget::{button, column, container, row, scrollable, text, Column, Space};
use iced::{
    Alignment, Border, Color, Element, Font, Length, Padding, Pixels, Point, Rectangle,
    Renderer, Shadow, Theme, Vector, mouse,
    alignment,
    widget::text::LineHeight,
};

use crate::app::{LotteryApp, Message};
use crate::model::{DrawMode, DrawState};
use crate::theme as c;

// ── Root layout ───────────────────────────────────────────────────────────────

pub fn root(app: &LotteryApp) -> Element<'_, Message> {
    let toolbar = toolbar(app);
    let marquee = marquee_bar(app);
    let body = row![prize_panel(app), draw_panel(app), result_panel(app),]
        .spacing(2)
        .height(Length::Fill);

    let content: Element<Message> = if let Some(err) = app.error.as_deref() {
        column![toolbar, marquee, error_banner(err), body]
            .spacing(0)
            .height(Length::Fill)
            .into()
    } else {
        column![toolbar, marquee, body]
            .spacing(0)
            .height(Length::Fill)
            .into()
    };

    container(content)
        .style(|_: &Theme| container::Style {
            background: Some(iced::Background::Color(c::BG_PRIMARY)),
            ..Default::default()
        })
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}

// ── Marquee bar ───────────────────────────────────────────────────────────────

struct MarqueeProgram {
    text: String,
    offset: usize,
}

impl canvas::Program<Message> for MarqueeProgram {
    type State = ();

    fn draw(
        &self,
        _state: &(),
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<canvas::Geometry<Renderer>> {
        let mut frame = Frame::new(renderer, bounds.size());

        let char_count = self.text.chars().count();
        if char_count == 0 {
            return vec![frame.into_geometry()];
        }

        // Each char ≈ 10px at size 15; scroll 2px per tick
        let text_px = char_count as f32 * 10.0;
        let cycle = (text_px + bounds.width) as usize;
        let px_offset = (self.offset * 2) % cycle;
        let x = bounds.width - px_offset as f32;

        frame.fill_text(Text {
            content: self.text.clone(),
            position: Point::new(x, bounds.height / 2.0),
            color: c::GOLD,
            size: Pixels(15.0),
            line_height: LineHeight::Relative(1.2),
            font: Font::default(),
            horizontal_alignment: alignment::Horizontal::Left,
            vertical_alignment: alignment::Vertical::Center,
            shaping: iced::widget::text::Shaping::Advanced,
        });

        vec![frame.into_geometry()]
    }
}

fn marquee_bar(app: &LotteryApp) -> Element<'_, Message> {
    let canvas = Canvas::new(MarqueeProgram {
        text: app.marquee_text.clone(),
        offset: app.marquee_offset,
    })
    .width(Length::Fill)
    .height(30);

    container(canvas)
        .style(|_: &Theme| container::Style {
            background: Some(iced::Background::Color(Color {
                r: 0.06,
                g: 0.04,
                b: 0.12,
                a: 1.0,
            })),
            border: Border {
                color: c::GOLD_DIM,
                width: 1.0,
                radius: 0.0.into(),
            },
            ..Default::default()
        })
        .width(Length::Fill)
        .into()
}

// ── Toolbar ───────────────────────────────────────────────────────────────────

fn toolbar(app: &LotteryApp) -> Element<'_, Message> {
    let drawing = app.draw_state == DrawState::Drawing;

    let left = row![
        gold_button("导入奖品", Message::OpenPrizesDialog, drawing),
        gold_button("导入候选人", Message::OpenCandidatesDialog, drawing),
    ]
    .spacing(8);

    let right = row![
        ghost_button("导出结果", Message::ExportResults, drawing),
        danger_button("重置", Message::Reset, drawing),
    ]
    .spacing(8);

    container(
        row![left, Space::with_width(Length::Fill), right]
            .align_y(Alignment::Center)
            .padding(Padding::from([10, 16])),
    )
    .style(|_: &Theme| container::Style {
        background: Some(iced::Background::Color(c::BG_SECONDARY)),
        ..Default::default()
    })
    .width(Length::Fill)
    .into()
}

// ── Left: Prize panel ─────────────────────────────────────────────────────────

fn prize_panel(app: &LotteryApp) -> Element<'_, Message> {
    let header = text("奖品列表").size(16).color(c::GOLD).width(Length::Fill);

    let mut list = Column::new().spacing(4);

    if app.prizes.is_empty() {
        list = list.push(text("尚未导入奖品").size(13).color(c::TEXT_MUTED));
    } else {
        for (i, prize) in app.prizes.iter().enumerate() {
            let selected = app.selected_prize == Some(i);
            let exhausted = prize.remaining == 0;
            let label = format!("{} ×{}/{}", prize.name, prize.remaining, prize.total);
            let label_color = if exhausted {
                c::TEXT_MUTED
            } else if selected {
                c::GOLD
            } else {
                c::TEXT_PRIMARY
            };

            let row_btn = button(text(label).size(14).color(label_color))
                .on_press_maybe(if !exhausted && app.draw_state == DrawState::Idle {
                    Some(Message::SelectPrize(i))
                } else {
                    None
                })
                .style(move |_: &Theme, status| button::Style {
                    background: Some(iced::Background::Color(if selected {
                        Color { a: 0.25, ..c::GOLD }
                    } else if matches!(status, button::Status::Hovered) && !exhausted {
                        Color { a: 0.10, ..c::GOLD }
                    } else {
                        Color::TRANSPARENT
                    })),
                    border: Border {
                        color: if selected { c::GOLD } else { Color::TRANSPARENT },
                        width: if selected { 2.0 } else { 0.0 },
                        radius: 6.0.into(),
                    },
                    text_color: label_color,
                    ..Default::default()
                })
                .width(Length::Fill)
                .padding(Padding::from([6, 10]));

            list = list.push(row_btn);
        }
    }

    let content = column![
        header,
        Space::with_height(8),
        scrollable(list).height(Length::Fill),
    ]
    .height(Length::Fill);

    panel_container(content.into(), 200)
}

// ── Center: Draw panel ────────────────────────────────────────────────────────

fn draw_panel(app: &LotteryApp) -> Element<'_, Message> {
    let mode_row = row![
        mode_btn("批量抽取", DrawMode::Batch, app.draw_mode),
        mode_btn("逐个抽取", DrawMode::Single, app.draw_mode),
    ]
    .spacing(6);

    let roll_display = rolling_display(app);

    let drawing = app.draw_state == DrawState::Drawing;
    let can_start = !drawing && app.selected_prize.is_some() && !app.candidates.is_empty();

    let btn_start = button(
        text(if drawing { "抽奖中..." } else { "开  始  抽  奖" })
            .size(18)
            .color(if can_start { c::BG_PRIMARY } else { c::TEXT_MUTED }),
    )
    .on_press_maybe(if can_start { Some(Message::StartDraw) } else { None })
    .style(move |_: &Theme, status| button::Style {
        background: Some(iced::Background::Color(if !can_start {
            c::BTN_DISABLED_BG
        } else if matches!(status, button::Status::Hovered) {
            c::GOLD_HOVER
        } else {
            c::GOLD
        })),
        border: Border { radius: 8.0.into(), ..Default::default() },
        text_color: if can_start { c::BG_PRIMARY } else { c::TEXT_MUTED },
        ..Default::default()
    })
    .width(Length::Fill)
    .padding(Padding::from([12, 20]));

    let btn_stop = button(
        text("停  止")
            .size(16)
            .color(if drawing { c::GOLD } else { c::TEXT_MUTED }),
    )
    .on_press_maybe(if drawing { Some(Message::StopDraw) } else { None })
    .style(move |_: &Theme, status| button::Style {
        background: Some(iced::Background::Color(
            if drawing && matches!(status, button::Status::Hovered) {
                Color { a: 0.15, ..c::GOLD }
            } else {
                Color::TRANSPARENT
            },
        )),
        border: Border {
            color: if drawing { c::GOLD } else { c::TEXT_MUTED },
            width: 1.5,
            radius: 8.0.into(),
        },
        text_color: if drawing { c::GOLD } else { c::TEXT_MUTED },
        ..Default::default()
    })
    .width(Length::Fill)
    .padding(Padding::from([10, 20]));

    let prize_info = match app.selected_prize.and_then(|i| app.prizes.get(i)) {
        Some(p) => {
            let hint = match app.draw_mode {
                DrawMode::Batch => format!("本次抽取 {} 人", p.remaining),
                DrawMode::Single => "本次抽取 1 人".to_string(),
            };
            text(format!("当前: {} — {}", p.name, hint))
                .size(13)
                .color(c::TEXT_SECONDARY)
        }
        None => text("← 请从左侧选择奖品").size(13).color(c::TEXT_MUTED),
    };

    let content = column![
        text("抽奖台").size(16).color(c::GOLD).width(Length::Fill),
        Space::with_height(8),
        mode_row,
        Space::with_height(16),
        roll_display,
        Space::with_height(12),
        prize_info,
        Space::with_height(16),
        btn_start,
        Space::with_height(8),
        btn_stop,
    ]
    .align_x(Alignment::Center)
    .height(Length::Fill);

    panel_container(content.into(), 0)
}

fn rolling_display(app: &LotteryApp) -> Element<'_, Message> {
    let drawing = app.draw_state == DrawState::Drawing;
    let border_color = if drawing { c::ROLL_BORDER } else { c::GOLD_DIM };

    let inner: Element<Message> = if app.rolling_names.is_empty() {
        container(text("?").size(64).color(c::GOLD_DIM))
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    } else if app.rolling_names.len() == 1 {
        container(
            text(app.rolling_names[0].clone())
                .size(52)
                .color(if drawing { c::GOLD } else { c::SUCCESS }),
        )
        .center_x(Length::Fill)
        .center_y(Length::Fill)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    } else {
        let names = app.rolling_names.iter().fold(
            Column::new().spacing(6).align_x(Alignment::Center),
            |col, name| {
                col.push(
                    text(name)
                        .size(28)
                        .color(if drawing { c::GOLD } else { c::SUCCESS }),
                )
            },
        );
        container(scrollable(names))
            .center_x(Length::Fill)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    };

    container(inner)
        .style(move |_: &Theme| container::Style {
            background: Some(iced::Background::Color(c::ROLL_BG)),
            border: Border {
                color: border_color,
                width: 2.0,
                radius: 12.0.into(),
            },
            shadow: if drawing {
                Shadow {
                    color: Color { a: 0.45, ..c::GOLD },
                    offset: Vector::ZERO,
                    blur_radius: 20.0,
                }
            } else {
                Shadow::default()
            },
            ..Default::default()
        })
        .width(Length::Fill)
        .height(220)
        .into()
}

fn mode_btn(label: &str, mode: DrawMode, current: DrawMode) -> Element<'static, Message> {
    let active = mode == current;
    let label = label.to_string();
    button(text(label).size(13).color(if active { c::BG_PRIMARY } else { c::TEXT_SECONDARY }))
        .on_press(Message::SetDrawMode(mode))
        .style(move |_: &Theme, status| button::Style {
            background: Some(iced::Background::Color(if active {
                c::GOLD
            } else if matches!(status, button::Status::Hovered) {
                Color { a: 0.15, ..c::GOLD }
            } else {
                Color::TRANSPARENT
            })),
            border: Border { color: c::GOLD_DIM, width: 1.0, radius: 6.0.into() },
            text_color: if active { c::BG_PRIMARY } else { c::TEXT_SECONDARY },
            ..Default::default()
        })
        .padding(Padding::from([5, 14]))
        .into()
}

// ── Right: Result panel ───────────────────────────────────────────────────────

fn result_panel(app: &LotteryApp) -> Element<'_, Message> {
    let won_count = app.candidates.iter().filter(|c| c.won).count();
    let total = app.candidates.len();

    let stats = column![
        stat_row("候选人总数", total.to_string()),
        stat_row("已中奖", won_count.to_string()),
        stat_row("剩余候选", total.saturating_sub(won_count).to_string()),
    ]
    .spacing(5);

    let divider = container(Space::with_height(1))
        .style(|_: &Theme| container::Style {
            background: Some(iced::Background::Color(c::GOLD_DIM)),
            ..Default::default()
        })
        .width(Length::Fill)
        .height(1);

    let mut records_col = Column::new().spacing(10);
    if app.win_records.is_empty() {
        records_col = records_col.push(text("暂无中奖记录").size(13).color(c::TEXT_MUTED));
    } else {
        for record in app.win_records.iter().rev() {
            let winners_str = record.winners.join("、");
            records_col = records_col.push(
                column![
                    text(&record.prize_name).size(13).color(c::GOLD),
                    text(winners_str).size(14).color(c::SUCCESS),
                ]
                .spacing(2),
            );
        }
    }

    let content = column![
        text("候选人 & 记录")
            .size(16)
            .color(c::GOLD)
            .width(Length::Fill),
        Space::with_height(10),
        stats,
        Space::with_height(10),
        divider,
        Space::with_height(10),
        text("中奖记录").size(14).color(c::TEXT_SECONDARY),
        Space::with_height(6),
        scrollable(records_col).height(Length::Fill),
    ]
    .height(Length::Fill);

    panel_container(content.into(), 220)
}

fn stat_row(label: &'static str, value: String) -> Element<'static, Message> {
    row![
        text(label).size(13).color(c::TEXT_MUTED).width(Length::Fill),
        text(value).size(14).color(c::TEXT_PRIMARY),
    ]
    .align_y(Alignment::Center)
    .into()
}

// ── Error banner ──────────────────────────────────────────────────────────────

fn error_banner(msg: &str) -> Element<'_, Message> {
    container(
        row![
            text(msg).size(13).color(Color::WHITE).width(Length::Fill),
            button(text("✕").size(12).color(Color::WHITE))
                .on_press(Message::DismissError)
                .style(|_: &Theme, _| button::Style {
                    background: None,
                    text_color: Color::WHITE,
                    ..Default::default()
                })
                .padding(Padding::from([2, 6])),
        ]
        .align_y(Alignment::Center)
        .padding(Padding::from([8, 16])),
    )
    .style(|_: &Theme| container::Style {
        background: Some(iced::Background::Color(c::BTN_DANGER_BG)),
        ..Default::default()
    })
    .width(Length::Fill)
    .into()
}

// ── Button helpers ────────────────────────────────────────────────────────────

fn gold_button(label: &str, msg: Message, disabled: bool) -> Element<'static, Message> {
    let label = label.to_string();
    button(text(label).size(14).color(if disabled { c::TEXT_MUTED } else { c::BG_PRIMARY }))
        .on_press_maybe(if disabled { None } else { Some(msg) })
        .style(move |_: &Theme, status| button::Style {
            background: Some(iced::Background::Color(if disabled {
                c::BTN_DISABLED_BG
            } else if matches!(status, button::Status::Hovered) {
                c::GOLD_HOVER
            } else {
                c::GOLD
            })),
            border: Border { radius: 6.0.into(), ..Default::default() },
            text_color: if disabled { c::TEXT_MUTED } else { c::BG_PRIMARY },
            ..Default::default()
        })
        .padding(Padding::from([7, 14]))
        .into()
}

fn ghost_button(label: &str, msg: Message, disabled: bool) -> Element<'static, Message> {
    let label = label.to_string();
    button(text(label).size(14).color(if disabled { c::TEXT_MUTED } else { c::TEXT_SECONDARY }))
        .on_press_maybe(if disabled { None } else { Some(msg) })
        .style(move |_: &Theme, status| button::Style {
            background: Some(iced::Background::Color(
                if !disabled && matches!(status, button::Status::Hovered) {
                    Color { a: 0.1, ..c::GOLD }
                } else {
                    Color::TRANSPARENT
                },
            )),
            border: Border {
                color: if disabled { c::TEXT_MUTED } else { c::TEXT_SECONDARY },
                width: 1.0,
                radius: 6.0.into(),
            },
            text_color: if disabled { c::TEXT_MUTED } else { c::TEXT_SECONDARY },
            ..Default::default()
        })
        .padding(Padding::from([7, 14]))
        .into()
}

fn danger_button(label: &str, msg: Message, disabled: bool) -> Element<'static, Message> {
    let label = label.to_string();
    button(text(label).size(14).color(if disabled { c::TEXT_MUTED } else { Color::WHITE }))
        .on_press_maybe(if disabled { None } else { Some(msg) })
        .style(move |_: &Theme, status| button::Style {
            background: Some(iced::Background::Color(if disabled {
                c::BTN_DISABLED_BG
            } else if matches!(status, button::Status::Hovered) {
                c::DANGER
            } else {
                c::BTN_DANGER_BG
            })),
            border: Border { radius: 6.0.into(), ..Default::default() },
            text_color: if disabled { c::TEXT_MUTED } else { Color::WHITE },
            ..Default::default()
        })
        .padding(Padding::from([7, 14]))
        .into()
}

// ── Panel container ───────────────────────────────────────────────────────────

fn panel_container(content: Element<'_, Message>, min_width: u16) -> Element<'_, Message> {
    let base = container(content)
        .style(|_: &Theme| container::Style {
            background: Some(iced::Background::Color(c::BG_PANEL)),
            border: Border {
                color: c::GOLD_DIM,
                width: 1.0,
                radius: 0.0.into(),
            },
            ..Default::default()
        })
        .padding(Padding::from([14, 16]))
        .height(Length::Fill);

    if min_width > 0 {
        base.width(Length::Fixed(min_width as f32)).into()
    } else {
        base.width(Length::Fill).into()
    }
}
