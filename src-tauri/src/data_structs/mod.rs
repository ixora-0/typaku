pub mod letter_performance;
use letter_performance::LetterPerformance;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ThemeColors {
    background: String,
    text: String,
}
impl Default for ThemeColors {
    fn default() -> Self {
        Self {
            background: "#ffffff".into(),
            text: "#000000".into(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct PerformanceWeight {
    speed: f64,
    accuracy: f64,
}
impl Default for PerformanceWeight {
    fn default() -> Self {
        Self {
            speed: 1.0,
            accuracy: 1.0,
        }
    }
}

#[derive(Serialize, Deserialize, Default)]
pub struct AppConfig {
    theme_colors: ThemeColors,
    performance_weight: PerformanceWeight,
}

#[derive(Serialize, Deserialize, Default)]
pub struct AppData {
    config: AppConfig,
    letter_performance: LetterPerformance,
}
