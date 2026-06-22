#[derive(Debug, Clone)]
pub struct ConnectionTestReport {
    pub driver: crate::DatabaseDriver,
    pub target: String,
    pub elapsed_ms: u128,
    pub connect_latency_ms: u128,
    pub roundtrip_latency_ms: u128,
    pub metadata_latency_ms: Option<u128>,
    pub server_version: Option<String>,
    pub encoding: Option<String>,
}

impl ConnectionTestReport {
    pub fn latency_summary(&self) -> String {
        let metadata = self
            .metadata_latency_ms
            .map(|latency| format!("{latency} ms metadata"))
            .unwrap_or_else(|| String::from("-- metadata"));

        format!(
            "{} ms total · {} ms connect · {} ms query · {metadata}",
            self.elapsed_ms, self.connect_latency_ms, self.roundtrip_latency_ms
        )
    }

    pub fn status_summary(&self) -> String {
        let mut parts = vec![
            format!("{} at {}", self.driver, self.target),
            self.latency_summary(),
        ];

        if let Some(version) = self.server_version.as_deref() {
            parts.push(format!("server {version}"));
        }

        if let Some(encoding) = self.encoding.as_deref() {
            parts.push(encoding.to_owned());
        }

        parts.join(" · ")
    }
}
