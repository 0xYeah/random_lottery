use std::fmt::Write as FmtWrite;
use std::fs;
use std::path::Path;

use crate::model::WinRecord;

pub fn save_results(path: &Path, records: &[WinRecord]) -> Result<(), String> {
    let mut content = String::new();
    writeln!(content, "=== 抽奖结果 ===").ok();
    writeln!(content).ok();

    for record in records {
        let winners = record.winners.join(", ");
        writeln!(content, "{}: {}", record.prize_name, winners).ok();
    }

    fs::write(path, content).map_err(|e| format!("Failed to write file: {e}"))
}
