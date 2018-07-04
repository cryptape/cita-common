// Copyright 2015-2017 Parity Technologies (UK) Ltd.
// This file is part of Parity.

// This software is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This software is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Parity.  If not, see <http://www.gnu.org/licenses/>.
#![feature(custom_attribute)]
#![allow(unused_attributes)]

extern crate byteorder;
extern crate cita_types as types;
extern crate crypto as rcrypto;
extern crate rand;
extern crate rustc_hex;
extern crate secp256k1;
extern crate tiny_keccak;

#[macro_use]
extern crate lazy_static;

mod brain;
mod error;
mod extended;
mod keccak;
mod keypair;
mod prefix;
mod random;
mod secret;
mod signature;

pub mod math;

pub use self::brain::Brain;
pub use self::error::Error;
pub use self::extended::{
    Derivation, DerivationError, ExtendedKeyPair, ExtendedPublic, ExtendedSecret,
};
pub use self::keypair::{public_to_address, KeyPair};
pub use self::math::public_is_valid;
pub use self::prefix::Prefix;
pub use self::random::Random;
pub use self::secret::Secret;
pub use self::signature::{recover, sign, verify_address, verify_public, Signature};

use types::{H160, H256, H512};

lazy_static! {
    pub static ref SECP256K1: secp256k1::Secp256k1 = secp256k1::Secp256k1::new();
}

/// Uninstantiatable error type for infallible generators.
#[derive(Debug)]
pub enum Void {}

/// Generates new keypair.
pub trait Generator {
    type Error;

    /// Should be called to generate new keypair.
    fn generate(self) -> Result<KeyPair, Self::Error>;
}

pub type Address = H160;
pub type Message = H256;
pub type Public = H512;
