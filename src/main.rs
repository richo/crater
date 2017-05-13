#![recursion_limit = "1024"]

#![allow(unused_must_use)]

extern crate rand;
extern crate clap;
#[macro_use]
extern crate error_chain;
extern crate tempdir;
extern crate url;
extern crate walkdir;
extern crate json;
extern crate semver;
#[macro_use]
extern crate lazy_static;
extern crate chrono;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate serde;
extern crate tar;
extern crate flate2;
extern crate toml;
#[macro_use]
extern crate scopeguard;
extern crate libc;
extern crate winapi;
extern crate kernel32;
extern crate reqwest;

#[macro_use]
mod log;
mod errors;
mod toolchain;
mod registry;
mod lists;
mod file;
mod dl;
mod gh;
mod util;
mod run;
mod crates;
mod git;
mod ex;
mod ex_run;
mod toml_frobber;
mod model;
mod gh_mirrors;
mod report;
mod docker;
mod dirs;

use clap::{App, AppSettings};
use dirs::*;
use errors::*;
use std::panic;
use std::process;

fn main() {
    log::init();
    let success = match panic::catch_unwind(main_) {
        Ok(Ok(())) => {
            true
        }
        Ok(Err(e)) => {
            util::report_error(&e);
            false
        }
        Err(e) => {
            util::report_panic(&*e);
            false
        }
    };
    info!("{}",
          if success {
              "command succeeded"
          } else {
              "command failed"
          });
    log::finish();
    process::exit(if success { 0 } else { 1 });
}

fn main_() -> Result<()> {
    let matches = cli().get_matches();
    let cmd = model::conv::clap_args_to_cmd(&matches)?;
    cmd.run()
}

fn cli() -> App<'static, 'static> {
    App::new("cargobomb")
        .version(env!("CARGO_PKG_VERSION"))
        .about("Kaboom!")
        .setting(AppSettings::VersionlessSubcommands)
        .setting(AppSettings::DeriveDisplayOrder)
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommands(model::conv::clap_cmds())
}
