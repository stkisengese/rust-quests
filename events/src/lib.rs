pub use chrono::Duration;
pub use colored::*;

#[derive(Debug, Eq, PartialEq)]
pub enum Position {
	Top,
	Bottom,
	Center,
}

#[derive(Debug, Eq, PartialEq)]
 pub struct Notification {
	pub size: u32,
	pub color: (u8, u8, u8),
	pub position: Position,
	pub content: String,
}

#[derive(Debug)]
pub enum Event<'a> {
	Remainder(&'a str),
	Registration(Duration),
	Appointment(&'a str),
	Holiday,
}

use std::fmt;

impl fmt::Display for Notification {
    pub fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        
    }
}

use Event::*;

impl Event {
	pub fn notify(&self) -> Notification {
	}
}