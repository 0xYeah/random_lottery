use std::fmt::Write as FmtWrite;
use std::fs;
use std::path::Path;

use crate::model::WinRecord;

pub fn save_results(path: &Path, records: &[WinRecord]) -> Result<(), String> {
    match path.extension().and_then(|e| e.to_str()).map(str::to_lowercase).as_deref() {
        Some("csv") => save_csv(path, records),
        Some("xlsx") => save_xlsx(path, records),
        _ => save_txt(path, records),
    }
}

fn save_txt(path: &Path, records: &[WinRecord]) -> Result<(), String> {
    let mut content = String::new();
    writeln!(content, "=== 抽奖结果 ===").ok();
    writeln!(content).ok();
    for r in records {
        writeln!(content, "{}: {}", r.prize_name, r.winners.join(", ")).ok();
    }
    fs::write(path, content).map_err(|e| format!("写入失败: {e}"))
}

fn save_csv(path: &Path, records: &[WinRecord]) -> Result<(), String> {
    let mut content = String::new();
    writeln!(content, "奖项,获奖者").ok();
    for r in records {
        for winner in &r.winners {
            writeln!(content, "{},{}", r.prize_name, winner).ok();
        }
    }
    fs::write(path, content).map_err(|e| format!("写入失败: {e}"))
}

fn save_xlsx(path: &Path, records: &[WinRecord]) -> Result<(), String> {
    use rust_xlsxwriter::{Color, Format, Workbook};

    let mut wb = Workbook::new();
    let ws = wb.add_worksheet();

    let header_fmt = Format::new()
        .set_bold()
        .set_background_color(Color::RGB(0x1A1A2E))
        .set_font_color(Color::RGB(0xF5A623));

    ws.write_with_format(0, 0, "奖项", &header_fmt).ok();
    ws.write_with_format(0, 1, "获奖者", &header_fmt).ok();

    let mut row = 1u32;
    for r in records {
        for winner in &r.winners {
            ws.write(row, 0, r.prize_name.as_str()).ok();
            ws.write(row, 1, winner.as_str()).ok();
            row += 1;
        }
    }

    ws.set_column_width(0, 20).ok();
    ws.set_column_width(1, 20).ok();

    wb.save(path).map_err(|e| format!("写入 Excel 失败: {e}"))
}
