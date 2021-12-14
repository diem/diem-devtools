// Copyright (c) The diem-devtools Contributors
// SPDX-License-Identifier: MIT OR Apache-2.0

use config::ConfigError;
use std::{error, fmt};

/// An error that occurred while reading the config.
#[derive(Debug)]
#[non_exhaustive]
pub struct ConfigReadError {
    inner: ConfigError,
}

impl ConfigReadError {
    pub(crate) fn new(inner: ConfigError) -> Self {
        Self { inner }
    }
}

impl fmt::Display for ConfigReadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.inner)
    }
}

impl error::Error for ConfigReadError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        Some(&self.inner)
    }
}

#[derive(Clone, Debug)]
pub struct ProfileNotFound {
    profile: String,
    all_profiles: Vec<String>,
}

impl ProfileNotFound {
    pub(crate) fn new(
        profile: impl Into<String>,
        all_profiles: impl IntoIterator<Item = impl Into<String>>,
    ) -> Self {
        let mut all_profiles: Vec<_> = all_profiles.into_iter().map(|s| s.into()).collect();
        all_profiles.sort_unstable();
        Self {
            profile: profile.into(),
            all_profiles,
        }
    }
}

impl fmt::Display for ProfileNotFound {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "profile '{}' not found (known profiles: {})",
            self.profile,
            self.all_profiles.join(", ")
        )
    }
}

impl error::Error for ProfileNotFound {}
