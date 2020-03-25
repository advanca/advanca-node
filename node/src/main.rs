// Copyright (C) 2020 ADVANCA PTE. LTD.
// Copyright 2017-2020 Parity Technologies (UK) Ltd.
// This file is part of Substrate.

// Substrate is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Substrate is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Substrate.  If not, see <http://www.gnu.org/licenses/>.

//! Advanca Node CLI library.
#![warn(missing_docs)]

mod chain_spec;
#[macro_use]
mod service;
mod cli;
mod command;

pub use sc_cli::{error, VersionInfo};

fn main() -> Result<(), error::Error> {
    let version = VersionInfo {
        name: "Advanca Node",
        commit: env!("VERGEN_SHA_SHORT"),
        version: env!("CARGO_PKG_VERSION"),
        executable_name: "advanca-node",
        author: "Advanca Authors",
        description: "Advanca node implementations based on Substrate",
        support_url: "https://github.com/AdvancaNetwork/",
        copyright_start_year: 2020,
    };

    command::run(version)
}
