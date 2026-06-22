use super::{ConnectionForm, ConnectionProfile, DatabaseDriver};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionListFilter {
    All,
    Network,
    File,
}

impl ConnectionListFilter {
    pub fn next(self) -> Self {
        match self {
            Self::All => Self::Network,
            Self::Network => Self::File,
            Self::File => Self::All,
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::All => "ALL",
            Self::Network => "NETWORK",
            Self::File => "FILE",
        }
    }

    pub fn matches(self, profile: &ConnectionProfile) -> bool {
        match self {
            Self::All => true,
            Self::Network => profile.form.driver.uses_network(),
            Self::File => !profile.form.driver.uses_network(),
        }
    }
}

pub fn prototype_profiles() -> Vec<ConnectionProfile> {
    vec![
        ConnectionProfile {
            id: 1,
            form: ConnectionForm {
                name: String::from("LOCAL-SQLITE-FILE"),
                driver: DatabaseDriver::Sqlite,
                location: String::from("./aktsql.db"),
                port: String::new(),
                username: String::new(),
                password: String::new(),
                database: String::new(),
                charset: String::from("UTF8"),
                collation: String::from("BINARY"),
                ssl_enabled: false,
                ssh_tunnel_enabled: true,
                timeout_seconds: String::from("30"),
                notes: String::new(),
            },
        },
        ConnectionProfile {
            id: 2,
            form: ConnectionForm {
                name: String::from("LOCAL-MYSQL"),
                driver: DatabaseDriver::MySql,
                location: String::from("127.0.0.1"),
                port: String::from("3306"),
                username: String::from("root"),
                password: String::from("root123"),
                database: String::from("myapp_db"),
                charset: String::from("utf8mb4"),
                collation: String::from("utf8mb4_unicode_ci"),
                ssl_enabled: false,
                ssh_tunnel_enabled: false,
                timeout_seconds: String::from("30"),
                notes: String::new(),
            },
        },
        ConnectionProfile {
            id: 3,
            form: ConnectionForm {
                name: String::from("LOCAL-POSTGRES"),
                driver: DatabaseDriver::PostgreSql,
                location: String::from("127.0.0.1"),
                port: String::from("5432"),
                username: String::from("postgres"),
                password: String::from("postgres123"),
                database: String::from("myapp_db"),
                charset: String::from("UTF8"),
                collation: String::from("C"),
                ssl_enabled: false,
                ssh_tunnel_enabled: false,
                timeout_seconds: String::from("30"),
                notes: String::new(),
            },
        },
        ConnectionProfile {
            id: 4,
            form: ConnectionForm {
                name: String::from("LOCAL-MONGODB"),
                driver: DatabaseDriver::MongoDb,
                location: String::from("127.0.0.1"),
                port: String::from("27017"),
                username: String::from("admin"),
                password: String::from("admin123"),
                database: String::from("myapp_db"),
                charset: String::from("UTF8"),
                collation: String::from("C"),
                ssl_enabled: false,
                ssh_tunnel_enabled: false,
                timeout_seconds: String::from("30"),
                notes: String::new(),
            },
        },
    ]
}
