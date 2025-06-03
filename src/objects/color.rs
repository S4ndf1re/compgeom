use std::sync::Mutex;

pub type Color = [f32; 4];

pub const RED: Color = [1.0, 0.0, 0.0, 1.0];
pub const GREEN: Color = [0.0, 1.0, 0.0, 1.0];
pub const BLUE: Color = [0.0, 0.0, 1.0, 1.0];
pub const MAGENTA: Color = [1.0, 0.0, 1.0, 1.0];
pub const YELLOW: Color = [1.0, 1.0, 0.0, 1.0];
pub const CYAN: Color = [0.0, 1.0, 1.0, 1.0];
pub const WHITE: Color = [1.0, 1.0, 1.0, 1.0];
pub const COLORS: [Color; 6] = [RED, GREEN, BLUE, MAGENTA, YELLOW, CYAN];

// Pastell colors
pub const PASTELL_DARKPURPLE: Color = [39.0 / 255.0, 40.0 / 255.0, 56.0 / 255.0, 1.0];
pub const PASTELL_GRAYPURPLE: Color = [93.0 / 255.0, 83.0 / 255.0, 107.0 / 255.0, 1.0];
pub const PASTELL_PURPLE: Color = [125.0 / 255.0, 107.0 / 255.0, 145.0 / 255.0, 1.0];
pub const PASTELL_GRAY: Color = [152.0 / 255.0, 159.0 / 255.0, 206.0 / 255.0, 1.0];
pub const PASTELL_BLUE: Color = [52.0 / 255.0, 127.0 / 255.0, 196.0 / 255.0, 1.0];
pub const PASTELL_BLACK_BEAN: Color = [59.0 / 255.0, 13.0 / 255.0, 17.0 / 255.0, 1.0];
pub const PASTELL_SLATE_GRAY: Color = [116.0 / 255.0, 131.0 / 255.0, 134.0 / 255.0, 1.0];
pub const PASTELL_ENGLISH_VIOLET: Color = [76.0 / 255.0, 57.0 / 255.05, 87.0 / 255.0, 1.0];

pub const PASTELL_COLORS: [Color; 8] = [
    PASTELL_DARKPURPLE,
    PASTELL_GRAYPURPLE,
    PASTELL_PURPLE,
    PASTELL_BLACK_BEAN,
    PASTELL_SLATE_GRAY,
    PASTELL_ENGLISH_VIOLET,
    PASTELL_GRAY,
    PASTELL_BLUE,
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
