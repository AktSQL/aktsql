#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThemeMode {
    Dark,
    Light,
}

impl ThemeMode {
    pub fn toggle(self) -> Self {
        match self {
            Self::Dark => Self::Light,
            Self::Light => Self::Dark,
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::Dark => "Dark",
            Self::Light => "Light",
        }
    }

    pub fn config_value(self) -> &'static str {
        match self {
            Self::Dark => "dark",
            Self::Light => "light",
        }
    }

    pub fn from_config(value: &str) -> Self {
        match value {
            "light" => Self::Light,
            _ => Self::Dark,
        }
    }
}
