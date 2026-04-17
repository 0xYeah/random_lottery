use calamine::{open_workbook_auto, Data, Reader};
use std::path::Path;

use crate::model::{Candidate, Prize};

fn cell_to_string(cell: &Data) -> Option<String> {
    match cell {
        Data::String(s) if !s.trim().is_empty() => Some(s.trim().to_string()),
        Data::Float(f) => Some(f.to_string()),
        Data::Int(i) => Some(i.to_string()),
        Data::Bool(b) => Some(b.to_string()),
        _ => None,
    }
}

pub fn load_prizes(path: &Path) -> Result<Vec<Prize>, String> {
    let mut workbook = open_workbook_auto(path).map_err(|e| format!("Cannot open file: {e}"))?;

    let sheet_name = workbook
        .sheet_names()
        .first()
        .cloned()
        .ok_or("No sheets found")?;

    let range = workbook
        .worksheet_range(&sheet_name)
        .map_err(|e| format!("Cannot read sheet: {e}"))?;

    let mut prizes = Vec::new();

    for row in range.rows() {
        let name = match row.first().and_then(cell_to_string) {
            Some(s) => s,
            None => continue,
        };

        if name.to_lowercase().contains("奖品") || name.to_lowercase().contains("name") {
            continue;
        }

        let total: u32 = match row.get(1) {
            Some(Data::Float(f)) => (*f as u32).max(1),
            Some(Data::Int(i)) => (*i as u32).max(1),
            _ => 1,
        };

        prizes.push(Prize {
            name,
            total,
            remaining: total,
        });
    }

    if prizes.is_empty() {
        return Err("No valid prize data found in file".to_string());
    }

    Ok(prizes)
}

pub fn load_candidates(path: &Path) -> Result<Vec<Candidate>, String> {
    let mut workbook = open_workbook_auto(path).map_err(|e| format!("Cannot open file: {e}"))?;

    let sheet_name = workbook
        .sheet_names()
        .first()
        .cloned()
        .ok_or("No sheets found")?;

    let range = workbook
        .worksheet_range(&sheet_name)
        .map_err(|e| format!("Cannot read sheet: {e}"))?;

    let mut candidates = Vec::new();

    for row in range.rows() {
        let name = match row.first().and_then(cell_to_string) {
            Some(s) => s,
            None => continue,
        };

        if name.to_lowercase().contains("姓名") || name.to_lowercase().contains("name") {
            continue;
        }

        let id = row.get(1).and_then(cell_to_string);

        candidates.push(Candidate {
            name,
            id,
            won: false,
        });
    }

    if candidates.is_empty() {
        return Err("No valid candidate data found in file".to_string());
    }

    Ok(candidates)
}
