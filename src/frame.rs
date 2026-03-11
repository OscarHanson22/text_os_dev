use crate::{Position, Rect, Positioned};
use crate::text_box::TextBox;
use crate::button_schema::Buttons;

#[derive(Copy, Clone, Debug)]
pub enum ReadWarning {
	OutOfBounds,
}

#[derive(Copy, Clone, Debug)]
pub enum FrameWarning {
	Cutoff, 
	Overwriting, 
}

pub trait AsFrame {
	fn as_frame(&self) -> Frame;
}

#[derive(Clone, Debug)]
pub struct Frame(Vec<Vec<char>>);

impl Rect for Frame {
	fn width(&self) -> usize {
		if self.0.len() == 0 {
			return 0;
		}
		
		self.0[0].len()
	}

	fn height(&self) -> usize {
		self.0.len() 
	}
}

impl Frame {
	pub fn new(width: usize, height: usize) -> Self {
		Self(
			(0..height).into_iter()
				.map(|_| 
					(0..width).into_iter()
						.map(|_| ' ')
						.collect()
				)
				.collect()
		)
	}

	pub fn from(vec: Vec<Vec<char>>) -> Self {
		Self(vec)
	}

	fn check_integrity(&self) -> bool {
		for row in &self.0 {
			if row.len() != self.width() {
				return false;
			}
		}

		return true;
	}

	pub fn read(&self, from_position: Position) -> Result<char, ReadWarning> {
		if from_position.x >= self.width() || from_position.y >= self.height() {
			return Err(ReadWarning::OutOfBounds);
		}

		Ok(self.0[from_position.y][from_position.x])
	}

	pub fn add_frame(&mut self, other_frame: &Frame, at_position: Position) -> Result<(), Vec<FrameWarning>> {
		// NOTE: add_frame-ing another frame, such that frameable.as_frame().as_frame() causes a malformation of the resultant frame. Please investigate why. 

		use std::cmp::min;

		let mut warnings = Vec::new();

		let start_x = at_position.x;
		let start_y = at_position.y;
		let end_x_maybe = start_x + other_frame.width();
		let end_y_maybe = start_y + other_frame.height();
		let end_x = min(end_x_maybe, self.width());
		let end_y = min(end_y_maybe, self.height()); 

		if end_x_maybe > end_x || end_y_maybe > end_y {
			warnings.push(FrameWarning::Cutoff);
		}

		let warnings_length = warnings.len();

		for x in start_x..end_x {
			for y in start_y..end_y {
				if self.0[y][x] != ' ' && warnings_length == warnings.len() {
					warnings.push(FrameWarning::Overwriting);
				}

				if x >= self.width() || y >= self.height() {
					continue;
				} 

				self.0[y][x] = other_frame.0[y - at_position.y]
				[x - at_position.x];
			}
		}

		if warnings.len() != 0 {
			return Err(warnings);
		}

		Ok(())
	}
}

impl std::fmt::Display for Frame {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		for row in self.0.iter() {
			let result = writeln!(f, "{}", row.iter().collect::<String>());

			match result {
				Ok(_) => (), 
				Err(_) => return result,
			}
		}

		Ok(())
	}
}

// PRINTLN!() IMPLEMENTATION BELOW

// impl std::fmt::Display for Frame {
// 	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
// 		for row in 0..self.height() - 1 {
// 			let row = &self.0[row];

// 			let result = writeln!(f, "{}", row.iter().collect::<String>());

// 			match result {
// 				Ok(_) => (), 
// 				Err(_) => return result,
// 			}
// 		}

// 		let last_row = self.height() - 1;
// 		let last_row = &self.0[last_row];

// 		write!(f, "{}", last_row.iter().collect::<String>())
// 	}
// }

// struct FramingCharacters;

mod FramingCharacters {
	pub const HORIZONTAL_LINE: char = '─';
	pub const VERTICAL_LINE: char = '│';
	pub const TOP_LEFT_CORNER: char = '┌';
	pub const TOP_RIGHT_CORNER: char = '┐';
	pub const BOTTOM_LEFT_CORNER: char = '└';
	pub const BOTTOM_RIGHT_CORNER: char = '┘';
	pub const LEFT_FORK: char = '├';
	pub const RIGHT_FORK: char = '┤';
	pub const TOP_FORK: char = '┬';
	pub const BOTTOM_FORK: char = '┴';
	pub const MIDDLE_FORK: char = '┼';
}

pub enum Framing {
	Horizontal(usize), 
	Vertical(usize), 
	Box(usize, usize), 
}

#[derive(PartialEq)]
enum LinePart {
	Start, 
	Middle, 
	End, 
}

impl LinePart {
	fn from(i: usize, length: usize) -> Self {
		match i {
			0 => Self::Start,
			i if i == length - 1 => Self::End,
			_ => Self::Middle, 
		}
	}
}

pub struct FrameBuilder(Frame);

impl FrameBuilder {
	pub fn from_frame(frame: Frame) -> Self {
		Self(frame)
	}

	pub fn with_dimensions(width: usize, height: usize) -> Self {
		Self(Frame::new(width, height))
	}

	pub fn add_border(self) -> Self {
		let width = self.0.width();
		let height = self.0.height();

		self.add_framing(Framing::Box(width, height), Position::new(0, 0))
	} 

	fn find_framing_character(&self, framing: Framing, line_part: LinePart, for_position: Position) -> Option<char> {
		use FramingCharacters::*;

		let is_vertical = |character| character == VERTICAL_LINE || character == RIGHT_FORK || character == LEFT_FORK;
		let is_horizontal = |character| character == HORIZONTAL_LINE || character == TOP_FORK || character == BOTTOM_FORK;

		let is_start = line_part == LinePart::Start;
		let is_end = line_part == LinePart::End;

		let character_above = if for_position == for_position.sub_y(1) { '\0' } else { self.0.read(for_position.sub_y(1)).ok().unwrap_or('\0') };
		let character_below = if for_position == for_position.add_y(1) { '\0' } else { self.0.read(for_position.add_y(1)).ok().unwrap_or('\0') };
		let character_left = if for_position == for_position.sub_x(1) { '\0' } else { self.0.read(for_position.sub_x(1)).ok().unwrap_or('\0') };
		let character_right = if for_position == for_position.add_x(1) { '\0' } else { self.0.read(for_position.add_x(1)).ok().unwrap_or('\0') };

		let c = match self.0.read(for_position) {
			Ok(c) => c, 
			Err(_) => return None, 
		};

		let framing_character = match framing {
			Framing::Horizontal(_) if is_vertical(c) => {
				if is_start {
					if is_vertical(character_above) && is_vertical(character_below) {
						LEFT_FORK
					} else if is_vertical(character_above) {
						BOTTOM_LEFT_CORNER
					} else {
						TOP_LEFT_CORNER
					}
				} else if is_end {
					if is_vertical(character_above) && is_vertical(character_below) {
						RIGHT_FORK
					} else if is_vertical(character_above) {
						BOTTOM_RIGHT_CORNER
					} else {
						TOP_RIGHT_CORNER
					}
				} else {
					MIDDLE_FORK
				}
			}

			Framing::Horizontal(_) => HORIZONTAL_LINE,

			Framing::Vertical(_) if is_horizontal(c) => {
				if is_start {
					if is_horizontal(character_left) && is_horizontal(character_right) {
						TOP_FORK
					} else if is_horizontal(character_left) {
						TOP_RIGHT_CORNER
					} else {
						TOP_LEFT_CORNER
					}
				} else if is_end {
					if is_horizontal(character_left) && is_horizontal(character_right) {
						BOTTOM_FORK
					} else if is_horizontal(character_left) {
						BOTTOM_RIGHT_CORNER
					} else {
						BOTTOM_LEFT_CORNER
					}
				} else {
					MIDDLE_FORK
				}
			}

			Framing::Vertical(_) => VERTICAL_LINE,

			Framing::Box(_, _) => return None,
		};

		Some(framing_character)
	}

	pub fn add_framing(mut self, framing: Framing, at_position: Position) -> Self {
		match framing {
			Framing::Horizontal(length) => {
				for i in 0..length {
					let position = at_position.add_x(i);
					let line_part = LinePart::from(i, length);

					if let Some(framing_character) = self.find_framing_character(Framing::Horizontal(0), line_part, position) {
						let _ = self.0.add_frame(&framing_character.as_frame(), position);
					}
				}
			}

			Framing::Vertical(length) => {
				for i in 0..length {
					let position = at_position.add_y(i);
					let line_part = LinePart::from(i, length);

					if let Some(framing_character) = self.find_framing_character(Framing::Vertical(0), line_part, position) {
						let _ = self.0.add_frame(&framing_character.as_frame(), position);
					}
				}
			}

			Framing::Box(width, height) => {
				return self
					.add_framing(Framing::Vertical(height), at_position)
					.add_framing(Framing::Vertical(height), at_position.add_x(width - 1))
					.add_framing(Framing::Horizontal(width), at_position)
					.add_framing(Framing::Horizontal(width), at_position.add_y(height - 1))
					
				;
					
			}
		}

		self
	}

	pub fn add_text_box(mut self, text_box: &TextBox) -> Self {
		self.0.add_frame(&text_box.as_frame(), text_box.position());
		self
	}

	pub fn add_buttons(mut self, buttons: &Buttons) -> Self {
		for (_name, button) in buttons {
			self.0.add_frame(&button.as_frame(), button.position());
		}
		self
	}

	// pub fn add_frame_builder(mut self, other_frame_builder: Self, at_position: Position) -> Self {
	// 	let _ = self.0.add_frame(&other_frame_builder.0, at_position);
	// 	self
	// }

	pub fn add_frame(mut self, frame: Frame, at_position: Position) -> Self {
		let _ = self.0.add_frame(&frame, at_position);
		self
	}

	pub fn to_frame(self) -> Frame {
		self.0
	}
}