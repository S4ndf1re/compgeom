use std::sync::Mutex;

pub type Color = [f32; 4];

pub const RED: Color = [1.0, 0.0, 0.0, 1.0];
pub const GREEN: Color = [0.0, 1.0, 0.0, 1.0];
pub const BLUE: Color = [0.0, 0.0, 1.0, 1.0];
pub const MAGENTA: Color = [1.0, 0.0, 1.0, 1.0];
pub const YELLOW: Color = [1.0, 1.0, 0.0, 1.0];
pub const CYAN: Color = [0.0, 1.0, 1.0, 1.0];
pub const WHITE: Color = [1.0, 1.0, 1.0, 1.0];
pub const ORANGE: Color = [1.0, 165.0 / 255.0, 0.0, 1.0]; // Orange
pub const PURPLE: Color = [128.0 / 255.0, 0.0, 128.0 / 255.0, 1.0]; // Lila
pub const BROWN: Color = [165.0 / 255.0, 42.0 / 255.0, 42.0 / 255.0, 1.0]; // Braun
pub const GOLD: Color = [1.0, 215.0 / 255.0, 0.0, 1.0]; // Gold
pub const TEAL: Color = [0.0, 128.0 / 255.0, 128.0 / 255.0, 1.0]; // Türkis
pub const NAVY: Color = [0.0, 0.0, 128.0 / 255.0, 1.0]; // Marineblau
pub const OLIVE: Color = [128.0 / 255.0, 128.0 / 255.0, 0.0, 1.0]; // Oliv
pub const SALMON_RED: Color = [250.0 / 255.0, 128.0 / 255.0, 114.0 / 255.0, 1.0]; // Lachsrot
pub const GERANIUM: Color = [207.0 / 255.0, 30.0 / 255.0, 39.0 / 255.0, 1.0]; // Geraniumrot
pub const DARK_GREEN: Color = [0.0, 100.0 / 255.0, 0.0, 1.0]; // Dunkelgrün
pub const GRAY: Color = [128.0 / 255.0, 128.0 / 255.0, 128.0 / 255.0, 1.0]; // Grau
pub const DEEP_PINK: Color = [1.0, 20.0 / 255.0, 147.0 / 255.0, 1.0]; // Tiefes Pink
pub const CRIMSON: Color = [220.0 / 255.0, 20.0 / 255.0, 60.0 / 255.0, 1.0]; // Karmesinrot
pub const INDIGO: Color = [75.0 / 255.0, 0.0, 130.0 / 255.0, 1.0]; // Indigo
pub const LIME_GREEN: Color = [50.0 / 255.0, 205.0 / 255.0, 50.0 / 255.0, 1.0]; // Limettengrün
pub const DARK_SLATE_GRAY: Color = [47.0 / 255.0, 79.0 / 255.0, 79.0 / 255.0, 1.0]; // Dunkel-Schiefergrau

pub const COLORS: [Color; 22] = [
    RED,
    GREEN,
    BLUE,
    MAGENTA,
    YELLOW,
    CYAN,
    ORANGE,
    PURPLE,
    BROWN,
    GOLD,
    TEAL,
    NAVY,
    OLIVE,
    SALMON_RED,
    GERANIUM,
    DARK_GREEN,
    GRAY,
    DEEP_PINK,
    CRIMSON,
    INDIGO,
    LIME_GREEN,
    DARK_SLATE_GRAY,
];

// Neue Pastellfarben
pub const PASTELL_SOFT_PINK: Color = [1.0, 182.0 / 255.0, 193.0 / 255.0, 1.0];
pub const PASTELL_LIGHT_BLUE: Color = [173.0 / 255.0, 216.0 / 255.0, 230.0 / 255.0, 1.0];
pub const PASTELL_MINT_GREEN: Color = [152.0 / 255.0, 1.0, 152.0 / 255.0, 1.0];
pub const PASTELL_PEACH: Color = [1.0, 218.0 / 255.0, 185.0 / 255.0, 1.0];
pub const PASTELL_LAVENDER: Color = [230.0 / 255.0, 190.0 / 255.0, 1.0, 1.0];
pub const PASTELL_LIGHT_YELLOW: Color = [1.0, 1.0, 184.0 / 255.0, 1.0];
pub const PASTELL_SOFT_ORANGE: Color = [1.0, 204.0 / 255.0, 153.0 / 255.0, 1.0];
pub const PASTELL_DUSTY_BLUE: Color = [119.0 / 255.0, 158.0 / 255.0, 203.0 / 255.0, 1.0];
pub const PASTELL_PALE_GREEN: Color = [152.0 / 255.0, 251.0 / 255.0, 152.0 / 255.0, 1.0];
pub const PASTELL_SALMON: Color = [1.0, 160.0 / 255.0, 122.0 / 255.0, 1.0];
pub const PASTELL_SOFT_PURPLE: Color = [200.0 / 255.0, 162.0 / 255.0, 200.0 / 255.0, 1.0];
pub const PASTELL_CREAM: Color = [1.0, 253.0 / 255.0, 208.0 / 255.0, 1.0];
pub const PASTELL_LIGHT_CORAL: Color = [240.0 / 255.0, 128.0 / 255.0, 128.0 / 255.0, 1.0];
pub const PASTELL_SOFT_TEAL: Color = [156.0 / 255.0, 220.0 / 255.0, 229.0 / 255.0, 1.0];
pub const PASTELL_LIGHT_GREY: Color = [211.0 / 255.0, 211.0 / 255.0, 211.0 / 255.0, 1.0];
pub const PASTELL_SOFT_MAUVE: Color = [188.0 / 255.0, 143.0 / 255.0, 143.0 / 255.0, 1.0];

pub const PASTELL_COLORS: [Color; 16] = [
    PASTELL_SOFT_PINK,
    PASTELL_LIGHT_BLUE,
    PASTELL_MINT_GREEN,
    PASTELL_PEACH,
    PASTELL_LAVENDER,
    PASTELL_LIGHT_YELLOW,
    PASTELL_SOFT_ORANGE,
    PASTELL_DUSTY_BLUE,
    PASTELL_PALE_GREEN,
    PASTELL_SALMON,
    PASTELL_SOFT_PURPLE,
    PASTELL_CREAM,
    PASTELL_LIGHT_CORAL,
    PASTELL_SOFT_TEAL,
    PASTELL_LIGHT_GREY,
    PASTELL_SOFT_MAUVE,
];

pub static GLOBAL_COLOR_GENERATOR: ColorGenerator = ColorGenerator {
    index: Mutex::new(0),
    pastell_index: Mutex::new(0),
};

pub struct ColorGenerator {
    index: Mutex<usize>,
    pastell_index: Mutex<usize>,
}

impl ColorGenerator {
    pub fn next_color(&self, alpha: f32) -> Color {
        let mut guard = self.index.lock().unwrap();
        *guard = (*guard + 1) % COLORS.len();

        let mut color = COLORS[*guard];
        color[3] = alpha;

        color
    }

    pub fn next_pastell_color(&self, alpha: f32) -> Color {
        let mut guard = self.pastell_index.lock().unwrap();
        *guard = (*guard + 1) % PASTELL_COLORS.len();

        let mut color = PASTELL_COLORS[*guard];
        color[3] = alpha;

        color
    }

    pub fn reset(&self) {
        let mut guard = self.index.lock().unwrap();
        *guard = 0;

        let mut guard = self.pastell_index.lock().unwrap();
        *guard = 0;
    }
}
