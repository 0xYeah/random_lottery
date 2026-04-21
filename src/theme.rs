#![allow(dead_code)]
use iced::Color;

// ─── Obsidian Gala ────────────────────────────────────────────────────────────
// OLED-optimized near-black with violet undertones for ceremonial depth.
// Palette direction: ui-ux-pro-max "Dark Mode OLED" + "Luxury/Premium" blend.

// Background layers
pub const BG_PRIMARY: Color = Color::from_rgb(0.047, 0.043, 0.063);    // #0C0B10
pub const BG_SECONDARY: Color = Color::from_rgb(0.075, 0.071, 0.102);  // #13121A
pub const BG_CARD: Color = Color::from_rgb(0.110, 0.102, 0.149);       // #1C1A26
pub const BG_PANEL: Color = Color::from_rgb(0.094, 0.086, 0.133);      // #181622

// Ceremonial Gold — true metallic, not amber
pub const GOLD: Color = Color::from_rgb(0.996, 0.831, 0.110);          // #FED41C
pub const GOLD_DIM: Color = Color::from_rgb(0.682, 0.490, 0.020);      // #AE7D05
pub const GOLD_HOVER: Color = Color::from_rgb(1.0, 0.929, 0.490);      // #FFED7D

// Text — warm cream for luxury feel, 7:1+ contrast on BG_PRIMARY
pub const TEXT_PRIMARY: Color = Color::from_rgb(0.976, 0.953, 0.906);  // #F9F3E7
pub const TEXT_SECONDARY: Color = Color::from_rgb(0.702, 0.671, 0.608);// #B3AB9B
pub const TEXT_MUTED: Color = Color::from_rgb(0.408, 0.388, 0.353);    // #68635A

// Status
pub const SUCCESS: Color = Color::from_rgb(0.059, 0.929, 0.576);       // #0FED93 winner emerald
pub const DANGER: Color = Color::from_rgb(0.941, 0.251, 0.286);        // #F04049

// Button states
pub const BTN_PRIMARY_BG: Color = GOLD_DIM;
pub const BTN_DANGER_BG: Color = Color::from_rgb(0.549, 0.137, 0.165); // #8C232A
pub const BTN_DISABLED_BG: Color = Color::from_rgb(0.196, 0.188, 0.216);// #323037

// Draw animation — theatrical pure-black for maximum drama
pub const ROLL_BG: Color = Color::from_rgb(0.016, 0.012, 0.027);       // #040309
pub const ROLL_BORDER: Color = GOLD;
