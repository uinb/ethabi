// Copyright 2015-2020 Parity Technologies
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use anyhow::anyhow;
use core::{num, string};
use alloc::string::String;

/// Ethabi result type
pub type Result<T> = core::result::Result<T, Error>;

/// Ethabi errors
#[derive(Debug)]
pub enum Error {
	/// Invalid entity such as a bad function name.
	InvalidName(String),
	/// Invalid data.
	InvalidData,
	/// Serialization error.
	SerdeJson(serde_json::Error),
	/// Integer parsing error.
	ParseInt(num::ParseIntError),
	/// UTF-8 parsing error.
	Utf8(alloc::string::FromUtf8Error),
	/// Hex string parsing error.
	Hex(hex::FromHexError),
	/// Other errors.
	Other(anyhow::Error),
}

impl From<uint::FromDecStrErr> for Error {
	fn from(err: uint::FromDecStrErr) -> Self {
		use uint::FromDecStrErr::*;
		match err {
			InvalidCharacter => anyhow!("Uint parse error: InvalidCharacter"),
			InvalidLength => anyhow!("Uint parse error: InvalidLength"),
		}
		.into()
	}
}
