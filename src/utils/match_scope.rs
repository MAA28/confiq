use std::{process::Command, str::from_utf8};

use crate::{
    confiq::{Confiq, Scope},
    error_handling::MatchScopeError,
    Error, Result,
};
use log::{error, info};
use regex::Regex;

use crate::confiq::ScopeDescriptor;

impl ScopeDescriptor {
    pub fn matching(&self) -> bool {
        self.check_whoami().unwrap_or(false)
    }

    fn check_whoami(&self) -> Result<bool> {
        info!("Checking whoami");

        let mut command = Command::new("whoami");
        let output = command.output().map_err(|_| {
            error!("Failed to check whoami");
            Error::MatchScope(MatchScopeError::CommandFailed)
        })?;

        let whoami = from_utf8(&output.stdout).map_err(|_| {
            error!("Failed to check whoami");
            Error::MatchScope(MatchScopeError::CommandFailed)
        })?;

        Ok(whoami.strip_suffix("\n").unwrap_or("") == self.whoami)
    }
}

impl Scope {
    pub fn matching(&self, confiq: &Confiq) -> bool {
        let regex = Regex::new(&self.0).unwrap();

        let matched = &confiq
            .scopes
            .iter()
            .filter(|(name, _)| regex.is_match(name))
            .filter(|(name, descriptor)| {
                info!("Checking scope: {}", name);
                descriptor.matching()
            });

        matched.clone().count() > 0
    }
}
