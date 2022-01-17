// Copyright (c) The diem-devtools Contributors
// SPDX-License-Identifier: MIT OR Apache-2.0

use cargo_nextest::{Cli, ExpectedError};
use clap::Parser;
use color_eyre::Result;

fn main() -> Result<()> {
    color_eyre::install()?;
    let _ = enable_ansi_support::enable_ansi_support();

    // let opts = Opts::from_iter(args());
    let Cli::Nextest(opts) = Cli::parse();
    match opts.exec() {
        Ok(()) => Ok(()),
        Err(err) => {
            let expected_error: ExpectedError = err.downcast()?;
            expected_error.display_to_stderr();
            std::process::exit(expected_error.process_exit_code())
        }
    }
}
