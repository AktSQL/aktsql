use serde::{Deserialize, Serialize};
use std::fmt;

type OptionSlice = &'static [&'static str];

struct Options<const N: usize>(&'static [&'static str; N]);

impl<const N: usize> Options<N> {
    fn as_slice(&self) -> OptionSlice {
        self.0.as_slice()
    }
}

macro_rules! options {
    ($($item:expr),* $(,)?) => {{
        Options(&[$($item),*])
    }};
}

static MYSQL_CHARSETS: Options<41> = options![
    "utf8mb4", "utf8mb3", "utf8", "latin1", "ascii", "binary", "gbk", "gb2312", "gb18030", "big5",
    "ucs2", "utf16", "utf16le", "utf32", "cp932", "sjis", "ujis", "euckr", "armscii8", "cp1250",
    "cp1251", "cp1256", "cp1257", "cp850", "cp852", "cp866", "dec8", "geostd8", "greek", "hebrew",
    "hp8", "keybcs2", "koi8r", "koi8u", "latin2", "latin5", "latin7", "macce", "macroman", "swe7",
    "tis620",
];
static POSTGRES_CHARSETS: Options<7> = options![
    "UTF8",
    "LATIN1",
    "SQL_ASCII",
    "WIN1252",
    "EUC_CN",
    "GB18030",
    "BIG5",
];
static SQLITE_CHARSETS: Options<3> = options!["UTF8", "UTF16le", "UTF16be"];
static SQL_SERVER_CHARSETS: Options<5> = options!["UTF8", "UTF16", "CP936", "CP950", "CP1252"];
static ORACLE_CHARSETS: Options<5> =
    options!["AL32UTF8", "UTF8", "ZHS16GBK", "WE8MSWIN1252", "AL16UTF16"];
static GENERIC_CHARSETS: Options<3> = options!["UTF8", "UTF16", "latin1"];

static MYSQL_UTF8MB4_COLLATIONS: Options<46> = options![
    "utf8mb4_0900_ai_ci",
    "utf8mb4_0900_as_ci",
    "utf8mb4_0900_as_cs",
    "utf8mb4_0900_bin",
    "utf8mb4_general_ci",
    "utf8mb4_unicode_ci",
    "utf8mb4_unicode_520_ci",
    "utf8mb4_bin",
    "utf8mb4_croatian_ci",
    "utf8mb4_czech_ci",
    "utf8mb4_danish_ci",
    "utf8mb4_esperanto_ci",
    "utf8mb4_estonian_ci",
    "utf8mb4_german2_ci",
    "utf8mb4_hungarian_ci",
    "utf8mb4_icelandic_ci",
    "utf8mb4_latvian_ci",
    "utf8mb4_lithuanian_ci",
    "utf8mb4_persian_ci",
    "utf8mb4_polish_ci",
    "utf8mb4_roman_ci",
    "utf8mb4_romanian_ci",
    "utf8mb4_sinhala_ci",
    "utf8mb4_slovak_ci",
    "utf8mb4_slovenian_ci",
    "utf8mb4_spanish_ci",
    "utf8mb4_spanish2_ci",
    "utf8mb4_swedish_ci",
    "utf8mb4_turkish_ci",
    "utf8mb4_vietnamese_ci",
    "utf8mb4_de_pb_0900_ai_ci",
    "utf8mb4_is_0900_ai_ci",
    "utf8mb4_lv_0900_ai_ci",
    "utf8mb4_ro_0900_ai_ci",
    "utf8mb4_sl_0900_ai_ci",
    "utf8mb4_pl_0900_ai_ci",
    "utf8mb4_et_0900_ai_ci",
    "utf8mb4_es_0900_ai_ci",
    "utf8mb4_sv_0900_ai_ci",
    "utf8mb4_tr_0900_ai_ci",
    "utf8mb4_vi_0900_ai_ci",
    "utf8mb4_zh_0900_as_cs",
    "utf8mb4_ja_0900_as_cs",
    "utf8mb4_ja_0900_as_cs_ks",
    "utf8mb4_ko_0900_as_cs",
    "utf8mb4_ru_0900_ai_ci",
];
static MARIADB_UTF8MB4_COLLATIONS: Options<8> = options![
    "utf8mb4_general_ci",
    "utf8mb4_unicode_ci",
    "utf8mb4_unicode_520_ci",
    "utf8mb4_bin",
    "utf8mb4_uca1400_ai_ci",
    "utf8mb4_uca1400_as_ci",
    "utf8mb4_uca1400_as_cs",
    "utf8mb4_0900_ai_ci",
];
static MYSQL_UTF8_COLLATIONS: Options<4> = options![
    "utf8_general_ci",
    "utf8_unicode_ci",
    "utf8_unicode_520_ci",
    "utf8_bin",
];
static MYSQL_LATIN1_COLLATIONS: Options<4> = options![
    "latin1_swedish_ci",
    "latin1_general_ci",
    "latin1_general_cs",
    "latin1_bin",
];
static MYSQL_ASCII_COLLATIONS: Options<2> = options!["ascii_general_ci", "ascii_bin"];
static MYSQL_BINARY_COLLATIONS: Options<1> = options!["binary"];
static MYSQL_GBK_COLLATIONS: Options<2> = options!["gbk_chinese_ci", "gbk_bin"];
static MYSQL_GB2312_COLLATIONS: Options<2> = options!["gb2312_chinese_ci", "gb2312_bin"];
static MYSQL_GB18030_COLLATIONS: Options<3> = options![
    "gb18030_chinese_ci",
    "gb18030_unicode_520_ci",
    "gb18030_bin",
];
static MYSQL_BIG5_COLLATIONS: Options<2> = options!["big5_chinese_ci", "big5_bin"];
static MYSQL_UCS2_COLLATIONS: Options<3> =
    options!["ucs2_general_ci", "ucs2_unicode_ci", "ucs2_bin"];
static MYSQL_UTF16_COLLATIONS: Options<3> =
    options!["utf16_general_ci", "utf16_unicode_ci", "utf16_bin"];
static MYSQL_UTF16LE_COLLATIONS: Options<2> = options!["utf16le_general_ci", "utf16le_bin"];
static MYSQL_UTF32_COLLATIONS: Options<3> =
    options!["utf32_general_ci", "utf32_unicode_ci", "utf32_bin"];
static MYSQL_CP932_COLLATIONS: Options<2> = options!["cp932_japanese_ci", "cp932_bin"];
static MYSQL_SJIS_COLLATIONS: Options<2> = options!["sjis_japanese_ci", "sjis_bin"];
static MYSQL_UJIS_COLLATIONS: Options<2> = options!["ujis_japanese_ci", "ujis_bin"];
static MYSQL_EUCKR_COLLATIONS: Options<2> = options!["euckr_korean_ci", "euckr_bin"];
static MYSQL_ARMSCII8_COLLATIONS: Options<2> = options!["armscii8_general_ci", "armscii8_bin"];
static MYSQL_CP1250_COLLATIONS: Options<5> = options![
    "cp1250_general_ci",
    "cp1250_czech_cs",
    "cp1250_croatian_ci",
    "cp1250_polish_ci",
    "cp1250_bin",
];
static MYSQL_CP1251_COLLATIONS: Options<4> = options![
    "cp1251_general_ci",
    "cp1251_bulgarian_ci",
    "cp1251_ukrainian_ci",
    "cp1251_bin",
];
static MYSQL_CP1256_COLLATIONS: Options<2> = options!["cp1256_general_ci", "cp1256_bin"];
static MYSQL_CP1257_COLLATIONS: Options<3> =
    options!["cp1257_general_ci", "cp1257_lithuanian_ci", "cp1257_bin"];
static MYSQL_CP850_COLLATIONS: Options<2> = options!["cp850_general_ci", "cp850_bin"];
static MYSQL_CP852_COLLATIONS: Options<2> = options!["cp852_general_ci", "cp852_bin"];
static MYSQL_CP866_COLLATIONS: Options<2> = options!["cp866_general_ci", "cp866_bin"];
static MYSQL_DEC8_COLLATIONS: Options<2> = options!["dec8_swedish_ci", "dec8_bin"];
static MYSQL_GEOSTD8_COLLATIONS: Options<2> = options!["geostd8_general_ci", "geostd8_bin"];
static MYSQL_GREEK_COLLATIONS: Options<2> = options!["greek_general_ci", "greek_bin"];
static MYSQL_HEBREW_COLLATIONS: Options<2> = options!["hebrew_general_ci", "hebrew_bin"];
static MYSQL_HP8_COLLATIONS: Options<2> = options!["hp8_english_ci", "hp8_bin"];
static MYSQL_KEYBCS2_COLLATIONS: Options<2> = options!["keybcs2_general_ci", "keybcs2_bin"];
static MYSQL_KOI8R_COLLATIONS: Options<2> = options!["koi8r_general_ci", "koi8r_bin"];
static MYSQL_KOI8U_COLLATIONS: Options<2> = options!["koi8u_general_ci", "koi8u_bin"];
static MYSQL_LATIN2_COLLATIONS: Options<5> = options![
    "latin2_general_ci",
    "latin2_czech_cs",
    "latin2_croatian_ci",
    "latin2_hungarian_ci",
    "latin2_bin",
];
static MYSQL_LATIN5_COLLATIONS: Options<2> = options!["latin5_turkish_ci", "latin5_bin"];
static MYSQL_LATIN7_COLLATIONS: Options<4> = options![
    "latin7_general_ci",
    "latin7_estonian_cs",
    "latin7_general_cs",
    "latin7_bin",
];
static MYSQL_MACCE_COLLATIONS: Options<2> = options!["macce_general_ci", "macce_bin"];
static MYSQL_MACROMAN_COLLATIONS: Options<2> = options!["macroman_general_ci", "macroman_bin"];
static MYSQL_SWE7_COLLATIONS: Options<2> = options!["swe7_swedish_ci", "swe7_bin"];
static MYSQL_TIS620_COLLATIONS: Options<2> = options!["tis620_thai_ci", "tis620_bin"];

static POSTGRES_COLLATIONS: Options<5> =
    options!["C", "POSIX", "en_US.utf8", "zh_CN.utf8", "und-x-icu"];
static SQLITE_COLLATIONS: Options<3> = options!["BINARY", "NOCASE", "RTRIM"];
static SQL_SERVER_UTF8_COLLATIONS: Options<3> = options![
    "Latin1_General_100_CI_AS_SC_UTF8",
    "Latin1_General_100_BIN2_UTF8",
    "Chinese_PRC_100_CI_AS_SC_UTF8",
];
static SQL_SERVER_COLLATIONS: Options<5> = options![
    "SQL_Latin1_General_CP1_CI_AS",
    "Latin1_General_CI_AS",
    "Latin1_General_BIN2",
    "Chinese_PRC_CI_AS",
    "Chinese_Taiwan_Stroke_CI_AS",
];
static ORACLE_COLLATIONS: Options<4> =
    options!["BINARY", "BINARY_CI", "BINARY_AI", "USING_NLS_COMP"];
static GENERIC_COLLATIONS: Options<3> = options!["C", "BINARY", "NOCASE"];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DatabaseDriver {
    MySql,
    MariaDb,
    TiDb,
    PostgreSql,
    CockroachDb,
    Sqlite,
    SqlServer,
    Oracle,
    MongoDb,
}

impl DatabaseDriver {
    pub const ALL: [DatabaseDriver; 7] = [
        DatabaseDriver::MySql,
        DatabaseDriver::MariaDb,
        DatabaseDriver::TiDb,
        DatabaseDriver::PostgreSql,
        DatabaseDriver::CockroachDb,
        DatabaseDriver::Sqlite,
        DatabaseDriver::MongoDb,
    ];

    pub fn default_host(self) -> &'static str {
        match self {
            Self::Sqlite => "./aktsql.db",
            _ => "127.0.0.1",
        }
    }

    pub fn default_port(self) -> &'static str {
        match self {
            Self::MySql | Self::MariaDb => "3306",
            Self::TiDb => "4000",
            Self::PostgreSql | Self::CockroachDb => "5432",
            Self::SqlServer => "1433",
            Self::Oracle => "1521",
            Self::MongoDb => "27017",
            Self::Sqlite => "",
        }
    }

    pub fn default_username(self) -> &'static str {
        match self {
            Self::MySql | Self::MariaDb | Self::TiDb => "root",
            Self::PostgreSql | Self::CockroachDb => "postgres",
            Self::SqlServer => "sa",
            Self::Oracle => "system",
            Self::Sqlite => "",
            _ => "admin",
        }
    }

    pub fn default_database(self) -> &'static str {
        match self {
            Self::PostgreSql | Self::CockroachDb => "postgres",
            Self::SqlServer => "master",
            Self::Oracle => "ORCL",
            _ => "",
        }
    }

    pub fn default_charset(self) -> &'static str {
        match self {
            Self::MySql | Self::MariaDb | Self::TiDb => "utf8mb4",
            _ => "UTF8",
        }
    }

    pub fn charset_options(self) -> OptionSlice {
        match self {
            Self::MySql | Self::MariaDb | Self::TiDb => MYSQL_CHARSETS.as_slice(),
            Self::PostgreSql | Self::CockroachDb => POSTGRES_CHARSETS.as_slice(),
            Self::Sqlite => SQLITE_CHARSETS.as_slice(),
            Self::SqlServer => SQL_SERVER_CHARSETS.as_slice(),
            Self::Oracle => ORACLE_CHARSETS.as_slice(),
            _ => GENERIC_CHARSETS.as_slice(),
        }
    }

    pub fn default_collation(self) -> &'static str {
        self.default_collation_for_charset(self.default_charset())
    }

    pub fn default_collation_for_charset(self, charset: &str) -> &'static str {
        self.collation_options(charset)
            .first()
            .copied()
            .unwrap_or("C")
    }

    pub fn collation_options(self, charset: &str) -> OptionSlice {
        match self {
            Self::MySql | Self::TiDb => mysql_collations(charset),
            Self::MariaDb => mariadb_collations(charset),
            Self::PostgreSql | Self::CockroachDb => POSTGRES_COLLATIONS.as_slice(),
            Self::Sqlite => SQLITE_COLLATIONS.as_slice(),
            Self::SqlServer => {
                if charset == "UTF8" {
                    SQL_SERVER_UTF8_COLLATIONS.as_slice()
                } else {
                    SQL_SERVER_COLLATIONS.as_slice()
                }
            }
            Self::Oracle => ORACLE_COLLATIONS.as_slice(),
            _ => GENERIC_COLLATIONS.as_slice(),
        }
    }

    pub fn location_label(self) -> &'static str {
        match self {
            Self::Sqlite => "Database File",
            _ => "Host",
        }
    }

    pub fn uses_network(self) -> bool {
        !matches!(self, Self::Sqlite)
    }

    pub fn requires_port(self) -> bool {
        !matches!(self, Self::Sqlite)
    }
}

impl fmt::Display for DatabaseDriver {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MySql => f.write_str("MySQL"),
            Self::MariaDb => f.write_str("MariaDB"),
            Self::TiDb => f.write_str("TiDB"),
            Self::PostgreSql => f.write_str("PostgreSQL"),
            Self::CockroachDb => f.write_str("CockroachDB"),
            Self::Sqlite => f.write_str("SQLite"),
            Self::SqlServer => f.write_str("SQL Server"),
            Self::Oracle => f.write_str("Oracle"),
            Self::MongoDb => f.write_str("MongoDB"),
        }
    }
}

fn mysql_collations(charset: &str) -> OptionSlice {
    match charset {
        "utf8mb4" => MYSQL_UTF8MB4_COLLATIONS.as_slice(),
        "utf8mb3" | "utf8" => MYSQL_UTF8_COLLATIONS.as_slice(),
        "latin1" => MYSQL_LATIN1_COLLATIONS.as_slice(),
        "ascii" => MYSQL_ASCII_COLLATIONS.as_slice(),
        "binary" => MYSQL_BINARY_COLLATIONS.as_slice(),
        "gbk" => MYSQL_GBK_COLLATIONS.as_slice(),
        "gb2312" => MYSQL_GB2312_COLLATIONS.as_slice(),
        "gb18030" => MYSQL_GB18030_COLLATIONS.as_slice(),
        "big5" => MYSQL_BIG5_COLLATIONS.as_slice(),
        "ucs2" => MYSQL_UCS2_COLLATIONS.as_slice(),
        "utf16" => MYSQL_UTF16_COLLATIONS.as_slice(),
        "utf16le" => MYSQL_UTF16LE_COLLATIONS.as_slice(),
        "utf32" => MYSQL_UTF32_COLLATIONS.as_slice(),
        "cp932" => MYSQL_CP932_COLLATIONS.as_slice(),
        "sjis" => MYSQL_SJIS_COLLATIONS.as_slice(),
        "ujis" => MYSQL_UJIS_COLLATIONS.as_slice(),
        "euckr" => MYSQL_EUCKR_COLLATIONS.as_slice(),
        "armscii8" => MYSQL_ARMSCII8_COLLATIONS.as_slice(),
        "cp1250" => MYSQL_CP1250_COLLATIONS.as_slice(),
        "cp1251" => MYSQL_CP1251_COLLATIONS.as_slice(),
        "cp1256" => MYSQL_CP1256_COLLATIONS.as_slice(),
        "cp1257" => MYSQL_CP1257_COLLATIONS.as_slice(),
        "cp850" => MYSQL_CP850_COLLATIONS.as_slice(),
        "cp852" => MYSQL_CP852_COLLATIONS.as_slice(),
        "cp866" => MYSQL_CP866_COLLATIONS.as_slice(),
        "dec8" => MYSQL_DEC8_COLLATIONS.as_slice(),
        "geostd8" => MYSQL_GEOSTD8_COLLATIONS.as_slice(),
        "greek" => MYSQL_GREEK_COLLATIONS.as_slice(),
        "hebrew" => MYSQL_HEBREW_COLLATIONS.as_slice(),
        "hp8" => MYSQL_HP8_COLLATIONS.as_slice(),
        "keybcs2" => MYSQL_KEYBCS2_COLLATIONS.as_slice(),
        "koi8r" => MYSQL_KOI8R_COLLATIONS.as_slice(),
        "koi8u" => MYSQL_KOI8U_COLLATIONS.as_slice(),
        "latin2" => MYSQL_LATIN2_COLLATIONS.as_slice(),
        "latin5" => MYSQL_LATIN5_COLLATIONS.as_slice(),
        "latin7" => MYSQL_LATIN7_COLLATIONS.as_slice(),
        "macce" => MYSQL_MACCE_COLLATIONS.as_slice(),
        "macroman" => MYSQL_MACROMAN_COLLATIONS.as_slice(),
        "swe7" => MYSQL_SWE7_COLLATIONS.as_slice(),
        "tis620" => MYSQL_TIS620_COLLATIONS.as_slice(),
        _ => MYSQL_UTF8MB4_COLLATIONS.as_slice(),
    }
}

fn mariadb_collations(charset: &str) -> OptionSlice {
    match charset {
        "utf8mb4" => MARIADB_UTF8MB4_COLLATIONS.as_slice(),
        _ => mysql_collations(charset),
    }
}
