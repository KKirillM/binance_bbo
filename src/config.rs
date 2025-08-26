pub struct Config {
    addr: String,
    port: u16,
    currencies: Vec<String>,
}

impl Config {
    pub fn new(url: &str, currencies: &[String]) -> Result<Self, String> {
        let addr_port = url.trim_start_matches("wss://");
        let (addr, port_str) = addr_port
            .split_once(':')
            .ok_or_else(|| String::from("ip-address and port not found"))?;
        let port = port_str
            .parse::<u16>()
            .map_err(|err| format!("wrong port format: {}", err))?;
        Ok(Self {
            addr: addr.to_string(),
            port,
            currencies: currencies.to_owned(),
        })
    }

    pub fn addr(&self) -> &str {
        &self.addr
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub fn currencies(&self) -> &[String] {
        &self.currencies
    }
}

#[cfg(test)]
mod tests {
    use super::Config;

    #[test]
    fn parses_valid_url() {
        let currencies = vec!["btcusdt".into(), "ethusdt".into()];
        let cfg = Config::new("wss://example.com:9443", &currencies).unwrap();
        assert_eq!(cfg.addr(), "example.com");
        assert_eq!(cfg.port(), 9443);
        assert_eq!(cfg.currencies(), &currencies);
    }

    #[test]
    fn rejects_invalid_url() {
        let currencies = vec!["btcusdt".into()];
        assert!(Config::new("wss://example.com", &currencies).is_err());
    }
}
