#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord)]
pub enum Antigen {
	A,
	AB,
	B,
	O,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
enum RhFactor {
	Positive,
	Negative,
}

#[derive(PartialEq, Eq, PartialOrd)]
pub struct BloodType {
	pub antigen: Antigen,
	pub rh_factor: RhFactor,
}

use std::cmp::{Ord, Ordering};

use std::str::FromStr;

impl FromStr for Antigen {
}

impl FromStr for RhFactor {
}

impl Ord for BloodType {
}

impl FromStr for BloodType {
}

use std::fmt::{self, Debug};

impl Debug for BloodType {
}

impl BloodType {
	pub fn can_receive_from(&self, other: &Self) -> bool {
        (match self.antigen {
            Antigen::A => matches!(other.antigen, Antigen::A | Antigen::O),
            Antigen::B => matches!(other.antigen, Antigen::B | Antigen::O),
            Antigen::AB => true,
            Antigen::O => matches!(other.antigen, Antigen::O),
        }) && match self.rh_factor {
            RhFactor::Positive => true,
            RhFactor::Negative => other.rh_factor == RhFactor::Negative,
        }
	}

	pub fn donors(&self) -> Vec<Self> {
        
	}

	pub fn recipients(&self) -> Vec<BloodType> {}
}