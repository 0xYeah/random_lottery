#[derive(Debug, Clone)]
pub struct Prize {
    pub name: String,
    pub total: u32,
    pub remaining: u32,
}

#[derive(Debug, Clone)]
pub struct Candidate {
    pub name: String,
    #[allow(dead_code)]
    pub id: Option<String>,
    pub won: bool,
}

#[derive(Debug, Clone)]
pub struct WinRecord {
    pub prize_name: String,
    pub winners: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DrawMode {
    /// Draw all remaining winners at once
    Batch,
    /// Draw one winner per click
    Single,
}

impl Default for DrawMode {
    fn default() -> Self {
        DrawMode::Batch
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DrawState {
    Idle,
    Drawing,
}
