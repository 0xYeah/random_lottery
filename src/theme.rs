use iced::Color;

// Background layers
pub const BG_PRIMARY: Color = Color::from_rgb(0.102, 0.102, 0.180); // #1A1A2E
pub const BG_SECONDARY: Color = Color::from_rgb(0.086, 0.129, 0.243); // #16213E
pub const BG_CARD: Color = Color::from_rgb(0.059, 0.204, 0.376); // #0F3460
pub const BG_PANEL: Color = Color::from_rgb(0.118, 0.118, 0.200); // #1E1E33

// Accent / Gold
pub const GOLD: Color = Color::from_rgb(0.961, 0.651, 0.137); // #F5A623
pub const GOLD_DIM: Color = Color::from_rgb(0.631, 0.384, 0.027); // #A16207
pub const GOLD_HOVER: Color = Color::from_rgb(1.0, 0.780, 0.314); // #FFC750

// Text
pub const TEXT_PRIMARY: Color = Color::from_rgb(0.949, 0.918, 0.835); // #F2EAD5
pub const TEXT_SECONDARY: Color = Color::from_rgb(0.690, 0.671, 0.620); // #BFAB9E
pub const TEXT_MUTED: Color = Color::from_rgb(0.451, 0.431, 0.400); // #736E66

// Status
pub const SUCCESS: Color = Color::from_rgb(0.298, 0.867, 0.580); // #4CDD94
pub const DANGER: Color = Color::from_rgb(0.863, 0.251, 0.251); // #DC4040

// Button states
pub const BTN_PRIMARY_BG: Color = GOLD_DIM;
pub const BTN_DANGER_BG: Color = Color::from_rgb(0.545, 0.161, 0.161); // #8B2929
pub const BTN_DISABLED_BG: Color = Color::from_rgb(0.220, 0.208, 0.196); // #383532

// Animation display
pub const ROLL_BG: Color = Color::from_rgb(0.055, 0.055, 0.118); // #0E0E1E
pub const ROLL_BORDER: Color = GOLD;
