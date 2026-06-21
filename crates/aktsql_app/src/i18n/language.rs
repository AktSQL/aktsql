use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Language {
    EnUs,
    FrFr,
    RuRu,
    ZhCn,
    JaJp,
}

impl Language {
    pub const ALL: [Language; 5] = [
        Language::EnUs,
        Language::FrFr,
        Language::RuRu,
        Language::ZhCn,
        Language::JaJp,
    ];

    pub fn label(self) -> &'static str {
        match self {
            Self::EnUs => "English",
            Self::FrFr => "French",
            Self::RuRu => "Russian",
            Self::ZhCn => "Chinese",
            Self::JaJp => "Japanese",
        }
    }

    pub fn local_label(self) -> &'static str {
        match self {
            Self::EnUs => "english",
            Self::FrFr => "français",
            Self::RuRu => "русский",
            Self::ZhCn => "中文",
            Self::JaJp => "日本語",
        }
    }
}

impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.label())
    }
}
