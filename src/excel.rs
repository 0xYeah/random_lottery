use calamine::{open_workbook_auto, Data, Reader};
use std::path::Path;

use crate::model::{Candidate, Prize};

// ── Unified entry points (dispatch by extension) ───────────────────────────────

pub fn load_prizes(path: &Path) -> Result<Vec<Prize>, String> {
    match ext(path) {
        "txt" => load_prizes_txt(path),
        "csv" => load_prizes_csv(path),
        _ => load_prizes_excel(path),
    }
}

pub fn load_candidates(path: &Path) -> Result<Vec<Candidate>, String> {
    match ext(path) {
        "txt" => load_candidates_txt(path),
        "csv" => load_candidates_csv(path),
        _ => load_candidates_excel(path),
    }
}

fn ext(path: &Path) -> &str {
    path.extension().and_then(|e| e.to_str()).unwrap_or("").to_lowercase().leak()
}

// ── TXT loaders ───────────────────────────────────────────────────────────────

fn load_prizes_txt(path: &Path) -> Result<Vec<Prize>, String> {
    let content = std::fs::read_to_string(path).map_err(|e| format!("无法读取文件: {e}"))?;
    let mut prizes = Vec::new();
    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') { continue; }
        let (name, total) = parse_name_count(line, ' ');
        if !name.is_empty() {
            prizes.push(Prize { name, total, remaining: total });
        }
    }
    non_empty(prizes, "文件中未找到有效奖品数据")
}

fn load_candidates_txt(path: &Path) -> Result<Vec<Candidate>, String> {
    let content = std::fs::read_to_string(path).map_err(|e| format!("无法读取文件: {e}"))?;
    let mut candidates = Vec::new();
    for line in content.lines() {
        let name = line.trim();
        if name.is_empty() || name.starts_with('#') { continue; }
        candidates.push(Candidate { name: name.to_string(), id: None, won: false });
    }
    non_empty(candidates, "文件中未找到有效候选人数据")
}

// ── CSV loaders ───────────────────────────────────────────────────────────────

fn load_prizes_csv(path: &Path) -> Result<Vec<Prize>, String> {
    let content = std::fs::read_to_string(path).map_err(|e| format!("无法读取文件: {e}"))?;
    let mut prizes = Vec::new();
    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') { continue; }
        let (name, total) = parse_name_count(line, ',');
        let skip = name.to_lowercase().contains("奖品") || name.to_lowercase().contains("name");
        if !name.is_empty() && !skip {
            prizes.push(Prize { name, total, remaining: total });
        }
    }
    non_empty(prizes, "文件中未找到有效奖品数据")
}

fn load_candidates_csv(path: &Path) -> Result<Vec<Candidate>, String> {
    let content = std::fs::read_to_string(path).map_err(|e| format!("无法读取文件: {e}"))?;
    let mut candidates = Vec::new();
    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') { continue; }
        // first column is name; ignore rest
        let name = line.split(',').next().unwrap_or("").trim();
        let skip = name.to_lowercase().contains("姓名") || name.to_lowercase().contains("name");
        if !name.is_empty() && !skip {
            candidates.push(Candidate { name: name.to_string(), id: None, won: false });
        }
    }
    non_empty(candidates, "文件中未找到有效候选人数据")
}

// ── Excel loaders ─────────────────────────────────────────────────────────────

fn cell_str(cell: &Data) -> Option<String> {
    match cell {
        Data::String(s) if !s.trim().is_empty() => Some(s.trim().to_string()),
        Data::Float(f) => Some(f.to_string()),
        Data::Int(i) => Some(i.to_string()),
        Data::Bool(b) => Some(b.to_string()),
        _ => None,
    }
}

fn load_prizes_excel(path: &Path) -> Result<Vec<Prize>, String> {
    let mut wb = open_workbook_auto(path).map_err(|e| format!("无法打开文件: {e}"))?;
    let sheet = wb.sheet_names().first().cloned().ok_or("文件中没有工作表")?;
    let range = wb.worksheet_range(&sheet).map_err(|e| format!("无法读取工作表: {e}"))?;

    let mut prizes = Vec::new();
    for row in range.rows() {
        let name = match row.first().and_then(cell_str) { Some(s) => s, None => continue };
        if name.to_lowercase().contains("奖品") || name.to_lowercase().contains("name") { continue; }
        let total: u32 = match row.get(1) {
            Some(Data::Float(f)) => (*f as u32).max(1),
            Some(Data::Int(i)) => (*i as u32).max(1),
            _ => 1,
        };
        prizes.push(Prize { name, total, remaining: total });
    }
    non_empty(prizes, "文件中未找到有效奖品数据")
}

fn load_candidates_excel(path: &Path) -> Result<Vec<Candidate>, String> {
    let mut wb = open_workbook_auto(path).map_err(|e| format!("无法打开文件: {e}"))?;
    let sheet = wb.sheet_names().first().cloned().ok_or("文件中没有工作表")?;
    let range = wb.worksheet_range(&sheet).map_err(|e| format!("无法读取工作表: {e}"))?;

    let mut candidates = Vec::new();
    for row in range.rows() {
        let name = match row.first().and_then(cell_str) { Some(s) => s, None => continue };
        if name.to_lowercase().contains("姓名") || name.to_lowercase().contains("name") { continue; }
        let id = row.get(1).and_then(cell_str);
        candidates.push(Candidate { name, id, won: false });
    }
    non_empty(candidates, "文件中未找到有效候选人数据")
}

// ── Helpers ───────────────────────────────────────────────────────────────────

fn parse_name_count(line: &str, sep: char) -> (String, u32) {
    if let Some(pos) = line.rfind(sep) {
        let name = line[..pos].trim().to_string();
        let count = line[pos + sep.len_utf8()..].trim().parse::<u32>().unwrap_or(1).max(1);
        if !name.is_empty() { return (name, count); }
    }
    (line.to_string(), 1)
}

fn non_empty<T>(v: Vec<T>, msg: &str) -> Result<Vec<T>, String> {
    if v.is_empty() { Err(msg.to_string()) } else { Ok(v) }
}
