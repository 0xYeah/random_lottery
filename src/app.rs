use std::path::PathBuf;
use std::time::Duration;

use iced::{Element, Subscription, Task};

use crate::excel;
use crate::export;
use crate::model::{Candidate, DrawMode, DrawState, Prize, WinRecord};

// ── Messages ──────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub enum Message {
    OpenPrizesDialog,
    OpenCandidatesDialog,
    PrizesFileSelected(Option<PathBuf>),
    CandidatesFileSelected(Option<PathBuf>),
    SelectPrize(usize),
    StartDraw,
    StopDraw,
    Tick,
    SetDrawMode(DrawMode),
    ExportResults,
    ExportFileSelected(Option<PathBuf>),
    Reset,
    DismissError,
}

// ── App State ─────────────────────────────────────────────────────────────────

pub struct LotteryApp {
    pub prizes: Vec<Prize>,
    pub candidates: Vec<Candidate>,
    pub win_records: Vec<WinRecord>,
    pub selected_prize: Option<usize>,
    pub draw_state: DrawState,
    pub draw_mode: DrawMode,
    pub rolling_names: Vec<String>,
    pub tick_count: u32,
    pub error: Option<String>,
}

impl Default for LotteryApp {
    fn default() -> Self {
        Self {
            prizes: Vec::new(),
            candidates: Vec::new(),
            win_records: Vec::new(),
            selected_prize: None,
            draw_state: DrawState::Idle,
            draw_mode: DrawMode::Batch,
            rolling_names: Vec::new(),
            tick_count: 0,
            error: None,
        }
    }
}

// ── update / view / subscription (free functions for iced::application()) ─────

impl LotteryApp {
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::OpenPrizesDialog => {
                return Task::perform(
                    async {
                        rfd::AsyncFileDialog::new()
                            .set_title("选择奖品 Excel 文件")
                            .add_filter("Excel", &["xlsx", "xls", "ods"])
                            .pick_file()
                            .await
                            .map(|f| f.path().to_path_buf())
                    },
                    Message::PrizesFileSelected,
                );
            }

            Message::OpenCandidatesDialog => {
                return Task::perform(
                    async {
                        rfd::AsyncFileDialog::new()
                            .set_title("选择候选人 Excel 文件")
                            .add_filter("Excel", &["xlsx", "xls", "ods"])
                            .pick_file()
                            .await
                            .map(|f| f.path().to_path_buf())
                    },
                    Message::CandidatesFileSelected,
                );
            }

            Message::PrizesFileSelected(Some(path)) => match excel::load_prizes(&path) {
                Ok(prizes) => {
                    self.prizes = prizes;
                    self.selected_prize = None;
                    self.error = None;
                }
                Err(e) => self.error = Some(e),
            },

            Message::CandidatesFileSelected(Some(path)) => match excel::load_candidates(&path) {
                Ok(candidates) => {
                    self.candidates = candidates;
                    self.error = None;
                }
                Err(e) => self.error = Some(e),
            },

            Message::PrizesFileSelected(None) | Message::CandidatesFileSelected(None) => {}

            Message::SelectPrize(idx) => {
                if self.draw_state == DrawState::Idle {
                    self.selected_prize = Some(idx);
                    self.rolling_names.clear();
                }
            }

            Message::SetDrawMode(mode) => {
                self.draw_mode = mode;
            }

            Message::StartDraw => {
                let available_count = self.candidates.iter().filter(|c| !c.won).count();
                let prize_ok = self
                    .selected_prize
                    .and_then(|i| self.prizes.get(i))
                    .map(|p| p.remaining > 0)
                    .unwrap_or(false);

                if available_count == 0 {
                    self.error = Some("没有可用候选人".to_string());
                } else if !prize_ok {
                    self.error = Some("请先选择奖品，或该奖品已无剩余名额".to_string());
                } else {
                    self.draw_state = DrawState::Drawing;
                    self.tick_count = 0;
                    self.rolling_names.clear();
                }
            }

            Message::Tick => {
                if self.draw_state != DrawState::Drawing {
                    return Task::none();
                }

                self.tick_count += 1;

                let pool: Vec<String> = self
                    .candidates
                    .iter()
                    .filter(|c| !c.won)
                    .map(|c| c.name.clone())
                    .collect();

                if pool.is_empty() {
                    self.draw_state = DrawState::Idle;
                    return Task::none();
                }

                let count = match self.draw_mode {
                    DrawMode::Batch => self
                        .selected_prize
                        .and_then(|i| self.prizes.get(i))
                        .map(|p| p.remaining as usize)
                        .unwrap_or(1)
                        .min(pool.len()),
                    DrawMode::Single => 1_usize.min(pool.len()),
                };

                use rand::seq::SliceRandom;
                let mut rng = rand::thread_rng();
                let mut shuffled = pool.clone();
                shuffled.shuffle(&mut rng);
                self.rolling_names = shuffled.into_iter().take(count).collect();

                if self.tick_count >= 50 {
                    return self.update(Message::StopDraw);
                }
            }

            Message::StopDraw => {
                if self.draw_state != DrawState::Drawing {
                    return Task::none();
                }
                self.draw_state = DrawState::Idle;

                if self.rolling_names.is_empty() {
                    return Task::none();
                }

                if let Some(idx) = self.selected_prize {
                    if let Some(prize) = self.prizes.get_mut(idx) {
                        let to_mark = self.rolling_names.len().min(prize.remaining as usize);
                        let winners = self.rolling_names[..to_mark].to_vec();

                        for name in &winners {
                            if let Some(c) = self.candidates.iter_mut().find(|c| &c.name == name) {
                                c.won = true;
                            }
                        }

                        prize.remaining = prize.remaining.saturating_sub(to_mark as u32);

                        self.win_records.push(WinRecord {
                            prize_name: prize.name.clone(),
                            winners,
                        });

                        if self.draw_mode == DrawMode::Batch && prize.remaining == 0 {
                            self.selected_prize = None;
                        }
                    }
                }
            }

            Message::ExportResults => {
                if self.win_records.is_empty() {
                    self.error = Some("暂无抽奖记录可导出".to_string());
                    return Task::none();
                }
                return Task::perform(
                    async {
                        rfd::AsyncFileDialog::new()
                            .set_title("保存抽奖结果")
                            .add_filter("文本文件", &["txt"])
                            .set_file_name("lottery_result.txt")
                            .save_file()
                            .await
                            .map(|f| f.path().to_path_buf())
                    },
                    Message::ExportFileSelected,
                );
            }

            Message::ExportFileSelected(Some(path)) => {
                if let Err(e) = export::save_results(&path, &self.win_records) {
                    self.error = Some(e);
                }
            }

            Message::ExportFileSelected(None) => {}

            Message::Reset => {
                for c in &mut self.candidates {
                    c.won = false;
                }
                for p in &mut self.prizes {
                    p.remaining = p.total;
                }
                self.win_records.clear();
                self.rolling_names.clear();
                self.selected_prize = None;
                self.draw_state = DrawState::Idle;
                self.error = None;
            }

            Message::DismissError => {
                self.error = None;
            }
        }

        Task::none()
    }

    pub fn view(&self) -> Element<Message> {
        crate::view::root(self)
    }

    pub fn subscription(&self) -> Subscription<Message> {
        if self.draw_state == DrawState::Drawing {
            iced::time::every(Duration::from_millis(80)).map(|_| Message::Tick)
        } else {
            Subscription::none()
        }
    }
}
