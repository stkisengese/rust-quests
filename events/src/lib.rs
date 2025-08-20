use chrono::Duration;
use colored::*;

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
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (r, g, b) = (self.color.0, self.color.1, self.color.2);
		write!(f, "{:?}, {}, {}", self.position, self.size, self.content.truecolor(r, g, b))
    }
}

use Event::*;

impl Event<'_> {
	pub fn notify(&self) -> Notification {
        match self {
            Remainder(content) => Notification {
                size: 50,
                color: (50, 50, 50),
                position: Position::Bottom,
                content: format!("{}", content),
            },
            Registration(time) => Notification {
                size: 30,
                color: (255, 2, 22),
                position: Position::Top,
                content: format!("You have {}H:{}M:{}S left before the registration ends", 
                        time.num_seconds()/3600, 
                        time.num_seconds()/60 % 60, 
                        time.num_seconds() % 60
                    ),
            },
            Appointment(content) => Notification {
                size: 100,
                color: (200, 200, 3),
                position: Position::Center,
                content: format!("{}", content),
            },
            Holiday => Notification {
                size: 25,
                color: (0, 255, 0),
                position: Position::Top,
                content: "Enjoy your holiday!".to_string(),
            },
        }
	}
}