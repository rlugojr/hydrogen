// Copyright 2015 Nathan Sizemore <nathanrsizemore@gmail.com>
//
// This Source Code Form is subject to the terms of the
// Mozilla Public License, v. 2.0. If a copy of the MPL was not
// distributed with this file, You can obtain one at
// http://mozilla.org/MPL/2.0/.

use log::LogLevelFilter;

#[derive(Clone)]
pub struct Config {
    pub addr: String,
    pub port: u16,
    pub workers: u8,
    pub use_ssl: bool,
    pub ssl_cert: &'static str,
    pub ssl_key: &'static str,
    pub log_level: LogLevelFilter
}
