// Copyright (c) The cargo-guppy Contributors
// SPDX-License-Identifier: MIT OR Apache-2.0

use clap::{ArgEnum, Args};
use env_logger::fmt::Formatter;
use log::{Level, Record};
use owo_colors::{OwoColorize, Style};
use std::io::Write;
use supports_color::Stream;

#[derive(Copy, Clone, Debug, Args)]
#[must_use]
pub(crate) struct OutputOpts {
    // TODO: quiet/verbose?
    /// Produce color output
    #[clap(
        long,
        global = true,
        arg_enum,
        default_value_t = Color::Auto
    )]
    pub(crate) color: Color,
}

impl OutputOpts {
    pub(crate) fn init(self) -> OutputContext {
        let OutputOpts { color } = self;

        color.init_colored();

        OutputContext { color }
    }
}

#[derive(Copy, Clone, Debug)]
#[must_use]
pub(crate) struct OutputContext {
    pub(crate) color: Color,
}

#[derive(Copy, Clone, Debug, PartialEq, ArgEnum)]
#[must_use]
pub enum Color {
    Auto,
    Always,
    Never,
}

impl Color {
    fn init_colored(self) {
        match self {
            Color::Auto => owo_colors::unset_override(),
            Color::Always => owo_colors::set_override(true),
            Color::Never => owo_colors::set_override(false),
        }

        env_logger::Builder::from_env("NEXTEST_LOG")
            .format(format_fn)
            .init();
    }

    pub(crate) fn should_colorize(self, stream: Stream) -> bool {
        match self {
            Color::Auto => supports_color::on_cached(stream).is_some(),
            Color::Always => true,
            Color::Never => false,
        }
    }

    pub(crate) fn to_arg(self) -> &'static str {
        match self {
            Color::Auto => "--color=auto",
            Color::Always => "--color=always",
            Color::Never => "--color=never",
        }
    }
}

fn format_fn(f: &mut Formatter, record: &Record<'_>) -> std::io::Result<()> {
    match record.level() {
        Level::Error => writeln!(
            f,
            "{}: {}",
            "error".if_supports_color(Stream::Stderr, |s| s.style(Style::new().bold().red())),
            record.args()
        ),
        Level::Warn => writeln!(
            f,
            "{}: {}",
            "warning".if_supports_color(Stream::Stderr, |s| s.style(Style::new().bold().yellow())),
            record.args()
        ),
        Level::Info => writeln!(
            f,
            "{}: {}",
            "info".if_supports_color(Stream::Stderr, |s| s.bold()),
            record.args()
        ),
        Level::Debug => writeln!(
            f,
            "{}: {}",
            "debug".if_supports_color(Stream::Stderr, |s| s.bold()),
            record.args()
        ),
        _other => Ok(()),
    }
}
