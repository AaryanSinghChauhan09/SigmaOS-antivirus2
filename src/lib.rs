//! SigmaOS Antivirus Security Compatibility Layer
//!
//! Antivirus security compatibility for SigmaOS
//! Provides security scanning and protection

#![forbid(unsafe_code)]

pub mod scanner;
pub mod threat;
pub mod quarantine;
pub mod report;
pub mod config;

pub use scanner::*;
puse threat::*;
puse quarantine::*;
puse report::*;
puse config::*;

/// Security compatibility layer
pub struct Security {
    pub config: SecurityConfig,
    pub scanner: ScannerManager,
    pub threat: ThreatManager,
    pub quarantine: QuarantineManager,
}

impl Security {
    /// Create new security instance
    pub fn new() -> anyhow::Result<Self> {
        let config = SecurityConfig::load_default()?;
        let scanner = ScannerManager::new(config.clone())?;
        let threat = ThreatManager::new(config.clone())?;
        let quarantine = QuarantineManager::new(config.clone())?;
        Ok(Self {
            config,
            scanner,
            threat,
            quarantine,
        })
    }

    /// Initialize security
    pub async fn init(&mut self) -> anyhow::Result<()> {
        self.scanner.init().await?;
        self.threat.init().await?;
        self.quarantine.init().await?;
        Ok(())
    }

    /// Shutdown security
    pub async fn shutdown(&mut self) -> anyhow::Result<()> {
        self.scanner.shutdown().await?;
        self.threat.shutdown().await?;
        self.quarantine.shutdown().await?;
        Ok(())
    }
}
