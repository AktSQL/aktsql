mod driver;
mod profiles;

pub use driver::DatabaseDriver;
use profiles::{prototype_profiles, ConnectionListFilter};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionField {
    Name,
    Location,
    Port,
    Username,
    Password,
    Database,
    Charset,
    Collation,
    TimeoutSeconds,
    Notes,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionForm {
    pub name: String,
    pub driver: DatabaseDriver,
    pub location: String,
    pub port: String,
    pub username: String,
    #[serde(default, skip_serializing)]
    pub password: String,
    pub database: String,
    pub charset: String,
    pub collation: String,
    pub ssl_enabled: bool,
    pub ssh_tunnel_enabled: bool,
    pub timeout_seconds: String,
    pub notes: String,
}

impl Default for ConnectionForm {
    fn default() -> Self {
        Self::for_driver(DatabaseDriver::MySql)
    }
}

impl ConnectionForm {
    pub fn for_driver(driver: DatabaseDriver) -> Self {
        Self {
            name: format!("Local {}", driver),
            driver,
            location: String::from(driver.default_host()),
            port: String::from(driver.default_port()),
            username: String::from(driver.default_username()),
            password: String::new(),
            database: String::from(driver.default_database()),
            charset: String::from(driver.default_charset()),
            collation: String::from(driver.default_collation()),
            ssl_enabled: false,
            ssh_tunnel_enabled: false,
            timeout_seconds: String::from("30"),
            notes: String::new(),
        }
    }

    pub fn set_driver(&mut self, driver: DatabaseDriver) {
        self.driver = driver;
        self.location = String::from(driver.default_host());
        self.port = String::from(driver.default_port());
        self.username = String::from(driver.default_username());
        self.database = String::from(driver.default_database());
        self.charset = String::from(driver.default_charset());
        self.collation = String::from(driver.default_collation());
    }

    pub fn set_field(&mut self, field: ConnectionField, value: String) {
        match field {
            ConnectionField::Name => self.name = value,
            ConnectionField::Location => self.location = value,
            ConnectionField::Port => self.port = value,
            ConnectionField::Username => self.username = value,
            ConnectionField::Password => self.password = value,
            ConnectionField::Database => self.database = value,
            ConnectionField::Charset => self.charset = value,
            ConnectionField::Collation => self.collation = value,
            ConnectionField::TimeoutSeconds => self.timeout_seconds = value,
            ConnectionField::Notes => self.notes = value,
        }

        if field == ConnectionField::Charset {
            self.ensure_charset_supported();
            self.collation = String::from(self.driver.default_collation_for_charset(&self.charset));
        } else if field == ConnectionField::Collation {
            self.ensure_collation_supported();
        }
    }

    pub fn collation_options(&self) -> &'static [&'static str] {
        self.driver.collation_options(&self.charset)
    }

    pub fn charset_options(&self) -> &'static [&'static str] {
        self.driver.charset_options()
    }

    pub fn default_collation(&self) -> &'static str {
        self.driver.default_collation_for_charset(&self.charset)
    }

    pub fn normalize_for_ui(&mut self) {
        self.ensure_charset_supported();
        self.ensure_collation_supported();
    }

    fn ensure_charset_supported(&mut self) {
        if !self
            .driver
            .charset_options()
            .iter()
            .any(|option| *option == self.charset)
        {
            self.charset = String::from(self.driver.default_charset());
        }
    }

    fn ensure_collation_supported(&mut self) {
        if !self
            .collation_options()
            .iter()
            .any(|option| *option == self.collation)
        {
            self.collation = String::from(self.default_collation());
        }
    }

    pub fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        if self.name.trim().is_empty() {
            errors.push(String::from("Profile name is required."));
        }

        if self.location.trim().is_empty() {
            errors.push(format!("{} is required.", self.driver.location_label()));
        }

        if self.driver.requires_port() {
            match self.port.trim().parse::<u16>() {
                Ok(0) => errors.push(String::from("Port must be between 1 and 65535.")),
                Ok(_) => {}
                Err(_) => errors.push(String::from("Port must be a number between 1 and 65535.")),
            }
        }

        match self.timeout_seconds.trim().parse::<u16>() {
            Ok(0) => errors.push(String::from("Timeout must be greater than 0 seconds.")),
            Ok(_) => {}
            Err(_) => errors.push(String::from(
                "Timeout must be a positive number of seconds.",
            )),
        }

        if self.charset.trim().is_empty() {
            errors.push(String::from("Charset is required."));
        }

        if self.collation.trim().is_empty() {
            errors.push(String::from("Collation is required."));
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionProfile {
    pub id: usize,
    pub form: ConnectionForm,
}

impl ConnectionProfile {
    pub fn display_name(&self) -> String {
        format!("{} ({})", self.form.name, self.form.driver)
    }
}

#[derive(Debug)]
pub struct ConnectionManager {
    profiles: Vec<ConnectionProfile>,
    form: ConnectionForm,
    selected_profile_id: Option<usize>,
    list_filter: ConnectionListFilter,
    search_query: String,
    validation_errors: Vec<String>,
    next_profile_id: usize,
}

impl Default for ConnectionManager {
    fn default() -> Self {
        Self {
            profiles: Vec::new(),
            form: ConnectionForm::default(),
            selected_profile_id: None,
            list_filter: ConnectionListFilter::All,
            search_query: String::new(),
            validation_errors: Vec::new(),
            next_profile_id: 1,
        }
    }
}

impl ConnectionManager {
    pub fn with_profiles(profiles: Vec<ConnectionProfile>) -> Self {
        let mut merged_profiles = prototype_profiles();
        let mut next_profile_id = merged_profiles.len() + 1;

        for mut profile in profiles {
            profile.form.normalize_for_ui();

            let already_seeded = merged_profiles
                .iter()
                .any(|seed| seed.form.name == profile.form.name);

            if !already_seeded {
                profile.id = next_profile_id;
                next_profile_id += 1;
                merged_profiles.push(profile);
            }
        }

        let mut form = ConnectionForm::default();
        form.normalize_for_ui();

        Self {
            profiles: merged_profiles,
            form,
            selected_profile_id: None,
            list_filter: ConnectionListFilter::All,
            search_query: String::new(),
            validation_errors: Vec::new(),
            next_profile_id,
        }
    }

    pub fn profiles(&self) -> &[ConnectionProfile] {
        &self.profiles
    }

    pub fn visible_profiles(&self) -> Vec<&ConnectionProfile> {
        let query = self.search_query.trim().to_ascii_lowercase();

        self.profiles
            .iter()
            .filter(|profile| self.list_filter.matches(profile))
            .filter(|profile| {
                query.is_empty()
                    || profile.form.name.to_ascii_lowercase().contains(&query)
                    || profile.form.location.to_ascii_lowercase().contains(&query)
                    || profile
                        .form
                        .driver
                        .to_string()
                        .to_ascii_lowercase()
                        .contains(&query)
            })
            .collect()
    }

    pub fn search_query(&self) -> &str {
        &self.search_query
    }

    pub fn set_search_query(&mut self, query: String) {
        self.search_query = query;
    }

    pub fn list_filter_label(&self) -> &'static str {
        self.list_filter.label()
    }

    pub fn toggle_list_filter(&mut self) {
        self.list_filter = self.list_filter.next();
    }

    pub fn form(&self) -> &ConnectionForm {
        &self.form
    }

    pub fn selected_profile_id(&self) -> Option<usize> {
        self.selected_profile_id
    }

    pub fn is_new_profile(&self) -> bool {
        self.selected_profile_id.is_none()
    }

    pub fn profile_label(&self, id: usize) -> Option<String> {
        self.profiles
            .iter()
            .find(|profile| profile.id == id)
            .map(ConnectionProfile::display_name)
    }

    pub fn validation_errors(&self) -> &[String] {
        &self.validation_errors
    }

    pub fn active_label(&self) -> String {
        self.selected_profile_id
            .and_then(|id| self.profiles.iter().find(|profile| profile.id == id))
            .map(ConnectionProfile::display_name)
            .unwrap_or_else(|| String::from("No active connection"))
    }

    pub fn new_profile(&mut self) {
        self.form = ConnectionForm::default();
        self.selected_profile_id = None;
        self.validation_errors.clear();
    }

    pub fn set_driver(&mut self, driver: DatabaseDriver) {
        self.form.set_driver(driver);
        self.validation_errors.clear();
    }

    pub fn set_field(&mut self, field: ConnectionField, value: String) {
        self.form.set_field(field, value);
        self.validation_errors.clear();
    }

    pub fn set_ssl_enabled(&mut self, enabled: bool) {
        self.form.ssl_enabled = enabled;
        self.validation_errors.clear();
    }

    pub fn set_ssh_tunnel_enabled(&mut self, enabled: bool) {
        self.form.ssh_tunnel_enabled = enabled;
        self.validation_errors.clear();
    }

    pub fn select_profile(&mut self, id: usize) -> bool {
        let Some(profile) = self.profiles.iter().find(|profile| profile.id == id) else {
            return false;
        };

        self.form = profile.form.clone();
        self.form.normalize_for_ui();
        self.selected_profile_id = Some(id);
        self.validation_errors.clear();
        true
    }

    pub fn delete_profile(&mut self, id: usize) -> bool {
        let original_len = self.profiles.len();
        self.profiles.retain(|profile| profile.id != id);

        if self.profiles.len() == original_len {
            return false;
        }

        if self.selected_profile_id == Some(id) {
            self.new_profile();
        }

        true
    }

    pub fn current_form_for_test(&mut self) -> Result<ConnectionForm, Vec<String>> {
        self.validate_current()?;
        Ok(self.form.clone())
    }

    pub fn save_current(&mut self) -> Result<usize, Vec<String>> {
        self.validate_current()?;

        if let Some(id) = self.selected_profile_id {
            if let Some(profile) = self.profiles.iter_mut().find(|profile| profile.id == id) {
                profile.form = self.form.clone();
                return Ok(id);
            }
        }

        let id = self.next_profile_id;
        self.next_profile_id += 1;
        self.profiles.push(ConnectionProfile {
            id,
            form: self.form.clone(),
        });
        self.selected_profile_id = Some(id);

        Ok(id)
    }

    fn validate_current(&mut self) -> Result<(), Vec<String>> {
        match self.form.validate() {
            Ok(()) => {
                self.validation_errors.clear();
                Ok(())
            }
            Err(errors) => {
                self.validation_errors = errors.clone();
                Err(errors)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mysql_charset_options_include_common_legacy_sets() {
        let options = DatabaseDriver::MySql.charset_options();

        for expected in [
            "utf8mb4", "gbk", "gb2312", "gb18030", "big5", "cp932", "tis620",
        ] {
            assert!(
                options.contains(&expected),
                "{expected} should be selectable"
            );
        }
    }

    #[test]
    fn mysql_utf8mb4_collations_include_common_ci_options() {
        let options = DatabaseDriver::MySql.collation_options("utf8mb4");

        for expected in [
            "utf8mb4_0900_ai_ci",
            "utf8mb4_general_ci",
            "utf8mb4_unicode_ci",
            "utf8mb4_unicode_520_ci",
            "utf8mb4_bin",
        ] {
            assert!(
                options.contains(&expected),
                "{expected} should be selectable"
            );
        }
    }

    #[test]
    fn charset_change_resets_collation_to_matching_default() {
        let mut form = ConnectionForm::for_driver(DatabaseDriver::MySql);
        form.set_field(
            ConnectionField::Collation,
            String::from("utf8mb4_unicode_ci"),
        );

        form.set_field(ConnectionField::Charset, String::from("gb2312"));

        assert_eq!(form.charset, "gb2312");
        assert_eq!(form.collation, "gb2312_chinese_ci");
    }

    #[test]
    fn unsupported_collation_is_normalized_for_current_charset() {
        let mut form = ConnectionForm::for_driver(DatabaseDriver::MySql);
        form.charset = String::from("gbk");
        form.collation = String::from("utf8mb4_0900_ai_ci");

        form.normalize_for_ui();

        assert_eq!(form.charset, "gbk");
        assert_eq!(form.collation, "gbk_chinese_ci");
    }

    #[test]
    fn driver_change_resets_charset_and_collation() {
        let mut form = ConnectionForm::for_driver(DatabaseDriver::MySql);
        form.set_field(ConnectionField::Charset, String::from("gb18030"));

        form.set_driver(DatabaseDriver::PostgreSql);

        assert_eq!(form.charset, "UTF8");
        assert_eq!(form.collation, "C");
    }

    #[test]
    fn selected_profile_is_the_single_current_connection() {
        let first = ConnectionProfile {
            id: 1,
            form: ConnectionForm {
                name: String::from("Local MySQL"),
                driver: DatabaseDriver::MySql,
                location: String::from("127.0.0.1"),
                port: String::from("3306"),
                username: String::from("root"),
                password: String::new(),
                database: String::from("myapp_db"),
                charset: String::from("utf8mb4"),
                collation: String::from("utf8mb4_unicode_ci"),
                ssl_enabled: false,
                ssh_tunnel_enabled: false,
                timeout_seconds: String::from("30"),
                notes: String::new(),
            },
        };
        let second = ConnectionProfile {
            id: 2,
            form: ConnectionForm {
                name: String::from("Local PostgreSQL"),
                driver: DatabaseDriver::PostgreSql,
                location: String::from("127.0.0.1"),
                port: String::from("5432"),
                username: String::from("postgres"),
                password: String::new(),
                database: String::from("myapp_db"),
                charset: String::from("UTF8"),
                collation: String::from("C"),
                ssl_enabled: false,
                ssh_tunnel_enabled: false,
                timeout_seconds: String::from("30"),
                notes: String::new(),
            },
        };
        let mut manager = ConnectionManager::with_profiles(vec![first, second]);
        let mysql_id = manager
            .profiles()
            .iter()
            .find(|profile| profile.form.name == "Local MySQL")
            .map(|profile| profile.id)
            .expect("seeded MySQL profile should exist");
        let postgres_id = manager
            .profiles()
            .iter()
            .find(|profile| profile.form.name == "Local PostgreSQL")
            .map(|profile| profile.id)
            .expect("seeded PostgreSQL profile should exist");

        assert!(manager.select_profile(mysql_id));
        assert_eq!(manager.selected_profile_id(), Some(mysql_id));
        assert_eq!(manager.form().driver, DatabaseDriver::MySql);

        assert!(manager.select_profile(postgres_id));
        assert_eq!(manager.selected_profile_id(), Some(postgres_id));
        assert_eq!(manager.form().driver, DatabaseDriver::PostgreSql);
    }

    #[test]
    fn deleting_selected_profile_returns_to_new_connection_draft() {
        let form = ConnectionForm {
            name: String::from("Temporary"),
            ..ConnectionForm::for_driver(DatabaseDriver::MySql)
        };
        let mut manager = ConnectionManager::with_profiles(vec![ConnectionProfile { id: 1, form }]);
        let id = manager
            .profiles()
            .iter()
            .find(|profile| profile.form.name == "Temporary")
            .map(|profile| profile.id)
            .expect("temporary profile should exist");

        assert!(manager.select_profile(id));
        assert!(manager.delete_profile(id));

        assert_eq!(manager.selected_profile_id(), None);
        assert!(manager.is_new_profile());
        assert_eq!(manager.form().driver, DatabaseDriver::MySql);
    }
}
