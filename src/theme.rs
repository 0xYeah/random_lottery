#![allow(dead_code)]
use iced::Color;

// ─── Midnight Champagne ───────────────────────────────────────────────────────
// Neutral charcoal base (no color cast) + restrained champagne gold accent.
// Goal: calm luxury, readable contrast, less neon than prior palette.

// Background layers — true neutral charcoal, no violet undertone
pub const BG_PRIMARY: Color = Color::from_rgb(0.055, 0.055, 0.063);    // #0E0E10
pub const BG_SECONDARY: Color = Color::from_rgb(0.090, 0.090, 0.106);  // #17171B
pub const BG_CARD: Color = Color::from_rgb(0.118, 0.118, 0.141);       // #1E1E24
pub const BG_PANEL: Color = Color::from_rgb(0.102, 0.102, 0.122);      // #1A1A1F

// Champagne Gold — warm metallic, not saturated yellow
pub const GOLD: Color = Color::from_rgb(0.831, 0.659, 0.294);          // #D4A84B
pub const GOLD_DIM: Color = Color::from_rgb(0.541, 0.435, 0.180);      // #8A6F2E
pub const GOLD_HOVER: Color = Color::from_rgb(0.910, 0.745, 0.373);    // #E8BE5F

// Text — warm ivory, clear contrast on charcoal
pub const TEXT_PRIMARY: Color = Color::from_rgb(0.929, 0.902, 0.839);  // #EDE6D6
pub const TEXT_SECONDARY: Color = Color::from_rgb(0.659, 0.624, 0.553);// #A89F8D
pub const TEXT_MUTED: Color = Color::from_rgb(0.420, 0.392, 0.333);    // #6B6455

// Status
pub const SUCCESS: Color = Color::from_rgb(0.290, 0.871, 0.502);       // #4ADE80 soft emerald
pub const DANGER: Color = Color::from_rgb(0.878, 0.282, 0.282);        // #E04848

// Button states
pub const BTN_PRIMARY_BG: Color = GOLD_DIM;
pub const BTN_DANGER_BG: Color = Color::from_rgb(0.478, 0.125, 0.141); // #7A2024
pub const BTN_DISABLED_BG: Color = Color::from_rgb(0.165, 0.161, 0.188);// #2A2930

// Draw animation — deep near-black for stage contrast
pub const ROLL_BG: Color = Color::from_rgb(0.020, 0.020, 0.027);       // #050507
pub const ROLL_BORDER: Color = GOLD;
