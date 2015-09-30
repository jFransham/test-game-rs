#![allow(dead_code)]

pub const TRANSPARENT_BLACK: [u8; 4] = [0, 0, 0, 0];
pub const TRANSPARENT_WHITE: [u8; 4] = [255, 255, 255, 0];
pub const ALICE_BLUE: [u8; 4] = [240, 248, 255, 255];
pub const ANTIQUE_WHITE: [u8; 4] = [250, 235, 215, 255];
pub const AQUA: [u8; 4] = [0, 255, 255, 255];
pub const AQUAMARINE: [u8; 4] = [127, 255, 212, 255];
pub const AZURE: [u8; 4] = [240, 255, 255, 255];
pub const BEIGE: [u8; 4] = [245, 245, 220, 255];
pub const BISQUE: [u8; 4] = [255, 228, 196, 255];
pub const BLACK: [u8; 4] = [0, 0, 0, 255];
pub const BLANCHED_ALMOND: [u8; 4] = [255, 235, 205, 255];
pub const BLUE: [u8; 4] = [0, 0, 255, 255];
pub const BLUE_VIOLET: [u8; 4] = [138, 43, 226, 255];
pub const BROWN: [u8; 4] = [165, 42, 42, 255];
pub const BURLY_WOOD: [u8; 4] = [222, 184, 135, 255];
pub const CADET_BLUE: [u8; 4] = [95, 158, 160, 255];
pub const CHARTREUSE: [u8; 4] = [127, 255, 0, 255];
pub const CHOCOLATE: [u8; 4] = [210, 105, 30, 255];
pub const CORAL: [u8; 4] = [255, 127, 80, 255];
pub const CORNFLOWER_BLUE: [u8; 4] = [100, 149, 237, 255];
pub const CORNSILK: [u8; 4] = [255, 248, 220, 255];
pub const CRIMSON: [u8; 4] = [220, 20, 60, 255];
pub const CYAN: [u8; 4] = [0, 255, 255, 255];
pub const DARK_BLUE: [u8; 4] = [0, 0, 139, 255];
pub const DARK_CYAN: [u8; 4] = [0, 139, 139, 255];
pub const DARK_GOLDENROD: [u8; 4] = [184, 134, 11, 255];
pub const DARK_GRAY: [u8; 4] = [169, 169, 169, 255];
pub const DARK_GREEN: [u8; 4] = [0, 100, 0, 255];
pub const DARK_KHAKI: [u8; 4] = [189, 183, 107, 255];
pub const DARK_MAGENTA: [u8; 4] = [139, 0, 139, 255];
pub const DARK_OLIVE_GREEN: [u8; 4] = [85, 107, 47, 255];
pub const DARK_ORANGE: [u8; 4] = [255, 140, 0, 255];
pub const DARK_ORCHID: [u8; 4] = [153, 50, 204, 255];
pub const DARK_RED: [u8; 4] = [139, 0, 0, 255];
pub const DARK_SALMON: [u8; 4] = [233, 150, 122, 255];
pub const DARK_SEA_GREEN: [u8; 4] = [143, 188, 139, 255];
pub const DARK_SLATE_BLUE: [u8; 4] = [72, 61, 139, 255];
pub const DARK_SLATE_GRAY: [u8; 4] = [47, 79, 79, 255];
pub const DARK_TURQUOISE: [u8; 4] = [0, 206, 209, 255];
pub const DARK_VIOLET: [u8; 4] = [148, 0, 211, 255];
pub const DEEP_PINK: [u8; 4] = [255, 20, 147, 255];
pub const DEEP_SKY_BLUE: [u8; 4] = [0, 191, 255, 255];
pub const DIM_GRAY: [u8; 4] = [105, 105, 105, 255];
pub const DODGER_BLUE: [u8; 4] = [30, 144, 255, 255];
pub const FIREBRICK: [u8; 4] = [178, 34, 34, 255];
pub const FLORAL_WHITE: [u8; 4] = [255, 250, 240, 255];
pub const FOREST_GREEN: [u8; 4] = [34, 139, 34, 255];
pub const FUCHSIA: [u8; 4] = [255, 0, 255, 255];
pub const GAINSBORO: [u8; 4] = [220, 220, 220, 255];
pub const GHOST_WHITE: [u8; 4] = [248, 248, 255, 255];
pub const GOLD: [u8; 4] = [255, 215, 0, 255];
pub const GOLDENROD: [u8; 4] = [218, 165, 32, 255];
pub const GRAY: [u8; 4] = [128, 128, 128, 255];
pub const GREEN: [u8; 4] = [0, 128, 0, 255];
pub const GREEN_YELLOW: [u8; 4] = [173, 255, 47, 255];
pub const HONEYDEW: [u8; 4] = [240, 255, 240, 255];
pub const HOT_PINK: [u8; 4] = [255, 105, 180, 255];
pub const INDIAN_RED: [u8; 4] = [205, 92, 92, 255];
pub const INDIGO: [u8; 4] = [75, 0, 130, 255];
pub const IVORY: [u8; 4] = [255, 255, 240, 255];
pub const KHAKI: [u8; 4] = [240, 230, 140, 255];
pub const LAVENDER: [u8; 4] = [230, 230, 250, 255];
pub const LAVENDER_BLUSH: [u8; 4] = [255, 240, 245, 255];
pub const LAWN_GREEN: [u8; 4] = [124, 252, 0, 255];
pub const LEMON_CHIFFON: [u8; 4] = [255, 250, 205, 255];
pub const LIGHT_BLUE: [u8; 4] = [173, 216, 230, 255];
pub const LIGHT_CORAL: [u8; 4] = [240, 128, 128, 255];
pub const LIGHT_CYAN: [u8; 4] = [224, 255, 255, 255];
pub const LIGHT_GOLDENROD_YELLOW: [u8; 4] = [250, 250, 210, 255];
pub const LIGHT_GREEN: [u8; 4] = [144, 238, 144, 255];
pub const LIGHT_GRAY: [u8; 4] = [211, 211, 211, 255];
pub const LIGHT_PINK: [u8; 4] = [255, 182, 193, 255];
pub const LIGHT_SALMON: [u8; 4] = [255, 160, 122, 255];
pub const LIGHT_SEA_GREEN: [u8; 4] = [32, 178, 170, 255];
pub const LIGHT_SKY_BLUE: [u8; 4] = [135, 206, 250, 255];
pub const LIGHT_SLATE_GRAY: [u8; 4] = [119, 136, 153, 255];
pub const LIGHT_STEEL_BLUE: [u8; 4] = [176, 196, 222, 255];
pub const LIGHT_YELLOW: [u8; 4] = [255, 255, 224, 255];
pub const LIME: [u8; 4] = [0, 255, 0, 255];
pub const LIME_GREEN: [u8; 4] = [50, 205, 50, 255];
pub const LINEN: [u8; 4] = [250, 240, 230, 255];
pub const MAGENTA: [u8; 4] = [255, 0, 255, 255];
pub const MAROON: [u8; 4] = [128, 0, 0, 255];
pub const MEDIUM_AQUAMARINE: [u8; 4] = [102, 205, 170, 255];
pub const MEDIUM_BLUE: [u8; 4] = [0, 0, 205, 255];
pub const MEDIUM_ORCHID: [u8; 4] = [186, 85, 211, 255];
pub const MEDIUM_PURPLE: [u8; 4] = [147, 112, 219, 255];
pub const MEDIUM_SEA_GREEN: [u8; 4] = [60, 179, 113, 255];
pub const MEDIUM_SLATE_BLUE: [u8; 4] = [123, 104, 238, 255];
pub const MEDIUM_SPRING_GREEN: [u8; 4] = [0, 250, 154, 255];
pub const MEDIUM_TURQUOISE: [u8; 4] = [72, 209, 204, 255];
pub const MEDIUM_VIOLET_RED: [u8; 4] = [199, 21, 133, 255];
pub const MIDNIGHT_BLUE: [u8; 4] = [25, 25, 112, 255];
pub const MINT_CREAM: [u8; 4] = [245, 255, 250, 255];
pub const MISTY_ROSE: [u8; 4] = [255, 228, 225, 255];
pub const MOCCASIN: [u8; 4] = [255, 228, 181, 255];
pub const NAVAJO_WHITE: [u8; 4] = [255, 222, 173, 255];
pub const NAVY: [u8; 4] = [0, 0, 128, 255];
pub const OLD_LACE: [u8; 4] = [253, 245, 230, 255];
pub const OLIVE: [u8; 4] = [128, 128, 0, 255];
pub const OLIVE_DRAB: [u8; 4] = [107, 142, 35, 255];
pub const ORANGE: [u8; 4] = [255, 165, 0, 255];
pub const ORANGE_RED: [u8; 4] = [255, 69, 0, 255];
pub const ORCHID: [u8; 4] = [218, 112, 214, 255];
pub const PALE_GOLDENROD: [u8; 4] = [238, 232, 170, 255];
pub const PALE_GREEN: [u8; 4] = [152, 251, 152, 255];
pub const PALE_TURQUOISE: [u8; 4] = [175, 238, 238, 255];
pub const PALE_VIOLET_RED: [u8; 4] = [219, 112, 147, 255];
pub const PAPAYA_WHIP: [u8; 4] = [255, 239, 213, 255];
pub const PEACH_PUFF: [u8; 4] = [255, 218, 185, 255];
pub const PERU: [u8; 4] = [205, 133, 63, 255];
pub const PINK: [u8; 4] = [255, 192, 203, 255];
pub const PLUM: [u8; 4] = [221, 160, 221, 255];
pub const POWDER_BLUE: [u8; 4] = [176, 224, 230, 255];
pub const PURPLE: [u8; 4] = [128, 0, 128, 255];
pub const RED: [u8; 4] = [255, 0, 0, 255];
pub const ROSY_BROWN: [u8; 4] = [188, 143, 143, 255];
pub const ROYAL_BLUE: [u8; 4] = [65, 105, 225, 255];
pub const SADDLE_BROWN: [u8; 4] = [139, 69, 19, 255];
pub const SALMON: [u8; 4] = [250, 128, 114, 255];
pub const SANDY_BROWN: [u8; 4] = [244, 164, 96, 255];
pub const SEA_GREEN: [u8; 4] = [46, 139, 87, 255];
pub const SEA_SHELL: [u8; 4] = [255, 245, 238, 255];
pub const SIENNA: [u8; 4] = [160, 82, 45, 255];
pub const SILVER: [u8; 4] = [192, 192, 192, 255];
pub const SKY_BLUE: [u8; 4] = [135, 206, 235, 255];
pub const SLATE_BLUE: [u8; 4] = [106, 90, 205, 255];
pub const SLATE_GRAY: [u8; 4] = [112, 128, 144, 255];
pub const SNOW: [u8; 4] = [255, 250, 250, 255];
pub const SPRING_GREEN: [u8; 4] = [0, 255, 127, 255];
pub const STEEL_BLUE: [u8; 4] = [70, 130, 180, 255];
pub const TAN: [u8; 4] = [210, 180, 140, 255];
pub const TEAL: [u8; 4] = [0, 128, 128, 255];
pub const THISTLE: [u8; 4] = [216, 191, 216, 255];
pub const TOMATO: [u8; 4] = [255, 99, 71, 255];
pub const TURQUOISE: [u8; 4] = [64, 224, 208, 255];
pub const VIOLET: [u8; 4] = [238, 130, 238, 255];
pub const WHEAT: [u8; 4] = [245, 222, 179, 255];
pub const WHITE: [u8; 4] = [255, 255, 255, 255];
pub const WHITE_SMOKE: [u8; 4] = [245, 245, 245, 255];
pub const YELLOW: [u8; 4] = [255, 255, 0, 255];
pub const YELLOW_GREEN: [u8; 4] = [154, 205, 50, 255];

pub trait Color {
    fn f64_4(&self) -> [f64; 4];
    fn f32_4(&self) -> [f32; 4];
}

impl Color for [u8; 4] {
    fn f64_4(&self) -> [f64; 4] {
        return [
            self[0] as f64 / 255.0,
            self[1] as f64 / 255.0,
            self[2] as f64 / 255.0,
            self[3] as f64 / 255.0,
        ];
    }

    fn f32_4(&self) -> [f32; 4] {
        return [
            self[0] as f32 / 255.0,
            self[1] as f32 / 255.0,
            self[2] as f32 / 255.0,
            self[3] as f32 / 255.0,
        ];
    }
}
