use crate::Error;
use std::str::FromStr;

/// Running environment
pub enum Environment {
    /// Production environment
    Prod,
    /// Development environment
    Dev,
    /// Continous Integration environment
    CI,
}

impl FromStr for Environment {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "prod" | "Prod" | "PROD" | "Production" | "PRODUCTION" => Ok(Environment::Prod),
            "dev" | "Dev" | "DEV" | "Development" | "DEVELOPMENT" => Ok(Environment::Dev),
            "ci" | "CI" => Ok(Environment::CI),
            _ => Err(Error::Environment),
        }
    }
}
