use crate::{Frame, AsFrame, FrameBuilder, Rect, Position, Positioned, Interact, ControlFlow};
use crate::key::{Key, KeyCode};

#[derive(Copy, Clone, Debug)]
pub enum Alignment {
	Left, 
	Center, 
	Right, 
}

// pub struct TextBox
// pub struct ScrollableTextBox 
// pub struct CursoredTextBox

pub trait ProgramElementInteract {
	fn handle_input(&self, key: Key, cursor: &mut Cursor, position) -> Vec<ControlFlow>;
}

// animation for textboxes to 'rotate'

// pub trait Animation {
// 	fn next_frame(&mut self) -> Frame;
// }

// impl Animation for TextBox {
// 	fn next_frame(&mut self) -> Frame {

use std::cell::RefCell;

pub struct RotatingTextBox {
	text: String, 
	alignment: Alignment, 
	side_scroll: RefCell<usize>,
}

impl AsFrame for RotatingTextBox {
	fn as_frame(&self) -> Frame {
		



pub struct TextBox { // want there to be a way to highlight and copy from these, so they still need to accept input (if they are part of the schema)
	text: String, 
	width: usize, 
	height: usize,
	alignment: Alignment,
}

pub struct ScrollableTextBox {
	text_box: TextBox,
	scroll: usize, 
	scrollable: bool, 
	side_scroll: usize, 
}

pub struct CursoredTextBox {
	text_box: ScrollableTextBox, 
	cursor_position: Position, 
}





/*

pub trait TextBox: AsFrame + Rect {

mod FramingCharacters {
	pub const HORIZONTAL_LINE: char = '─';
	pub const VERTICAL_LINE: char = '│';
	pub const TOP_LEFT_CORNER: char = '┌';
	pub const TOP_RIGHT_CORNER: char = '┐';
	pub const BOTTOM_LEFT_CORNER: char = '└';
	pub const BOTTOM_RIGHT_CORNER: char = '┘';
	pub const LEFT_FORK: char = '├';
	pub const RIGHT_FORK: char = '1';
	pub const TOP_FORK: char = '┬';
	pub const BOTTOM_FORK: char = '┴';
	pub const MIDDLE_FORK: char = '┼';
}


// scroll bar design
┌────────────────┬─┐
│ v Item 1       ├─┤
│                │ │
│ Item 2;        ├─┤
│ Item 3         │ │
│ Item 4         │ │
├──┬──────┬──────┼─┤
└──┴──────┴──────┴─┘

*/


#[derive(Debug, Clone)]
pub struct TextBox {
	text: String, 
	width: usize, 
	height: usize, 
	position: Position,
	alignment: Alignment, 
	scroll: usize, 
	side_scroll: usize,
}

impl TextBox {
	pub fn new(from_string: &str, width: usize, height: usize, position: Position) -> Self {
		Self {
			text: from_string.to_string(), 
			width, 
			height, 
			position, 
			alignment: Alignment::Left, 
			scroll: 0, 
			side_scroll: 0,
		}
	}

	pub fn text(&mut self) -> &mut String {
		&mut self.text
	}

	pub fn positioned_text(&self) -> Vec<Vec<(char, Position)>> {
		self.position_text()
	}

	pub fn alignment(mut self, alignment: Alignment) -> Self {
		self.alignment = alignment;
		self
	}

	pub fn set_alignment(&mut self, alignment: Alignment) {
		self.alignment = alignment;
	}

	pub fn set_scroll(&mut self, scroll: usize) {
		self.scroll = scroll;
	}

	pub fn change_scroll(&mut self, scroll: isize) {
		self.scroll = self.scroll.saturating_add_signed(scroll);
	}

	pub fn set_side_scroll(&mut self, side_scroll: usize) {
		self.side_scroll = side_scroll;
	}

	pub fn change_side_scroll(&mut self, side_scroll: isize) {
		self.side_scroll = self.side_scroll.saturating_add_signed(side_scroll);
	}

	fn position_text(&self) -> Vec<Vec<(char, Position)>> {
		let wrapped_text = wrap_text(&self.text, self.width);

		let mut positioned_text = Vec::new();
		let mut x = 0;
		let mut y = 0;

		positioned_text.push(Vec::new());

		for c in wrapped_text.chars() {
			if c == '\n' || c == '\r' {
				positioned_text.push(Vec::new());
				x = 0;
				y += 1;
				continue;
			}

			let last_index = positioned_text.len() - 1;
			positioned_text[last_index].push((c, Position::new(x, y)));
			x += 1;
		}

		for line in &mut positioned_text {
			let line_length = line.len();

			for (c, position) in &mut *line {
				*position = position.add_x(
					match &self.alignment {
						Alignment::Right => self.width - line_length,
						Alignment::Center => (self.width - line_length) / 2,
						Alignment::Left => 0,
					}
				);
			}
		}

		let positioned_text: Vec<Vec<(char, Position)>> = positioned_text.iter()
			.skip(self.scroll)
			.take(self.height)
			.map(|line| line.iter()
				.skip(self.side_scroll)
				.map(|(c, position)| (*c, position.sub(&Position::new(self.side_scroll, self.scroll))))
				.collect()
			)
			.collect();

		positioned_text
	}
}

impl Rect for TextBox {
	fn width(&self) -> usize { self.width }
	fn height(&self) -> usize { self.height }
}

impl Positioned for TextBox {
	fn position(&self) -> Position { self.position }
}

impl AsFrame for &TextBox {
	fn as_frame(&self) -> Frame {
		let mut frame = FrameBuilder::with_dimensions(self.width, self.height).to_frame();

		for line in &self.positioned_text() {
			for (character, position) in line {
				frame.add_frame(&character.as_frame(), *position);
			}
		}

		frame
	}
}

impl Interact for TextBox {
	fn handle_input(&mut self, key: Key) -> Vec<ControlFlow> {
		if key.key_down() {
			if !key.character().is_ascii_control() {
				self.text().push(key.character());
			}

			match key.key_code() {
				Some(KeyCode::Backspace) => {
					let _ = self.text().pop();
				}

				Some(KeyCode::Enter) => {
					self.text().push('\n');
				}

				_ => (),
			}

			let positioned_text = self.positioned_text();

			if let Some(last_element) = last_element(&positioned_text) {
				vec![ControlFlow::SetCursorSizeTo(1, 1), ControlFlow::MoveCursorToPosition(last_element.1.add(&self.position()).add_x(1))];
			} else {
				vec![ControlFlow::SetCursorSizeTo(1, 1), ControlFlow::MoveCursorToPosition(self.position())];
			}
		}

		vec![ControlFlow::Continue]
	}
}

pub fn last_element<T: Copy>(elements: &Vec<Vec<T>>) -> Option<T> {
	if elements.len() == 0 {
		return None;
	} 

	let last_outer_index = elements.len() - 1;
	if elements[last_outer_index].len() == 0 {
		return None;
	}

	let last_inner_index = elements[last_outer_index].len() - 1;

	Some(elements[last_outer_index][last_inner_index])
}

fn side_scroll_text(text: &str, side_scroll: usize) -> String {
	text.lines()
		.map(|line| line.to_string().chars().skip(side_scroll).collect::<String>() + "\n")
		.collect()
}

fn scroll_text(text: &str, scroll: usize, height: usize) -> String {
	text.lines()
		.skip(scroll)
		.take(height)
		.map(|line| line.to_string() + "\n")
		.collect()
}

fn align_text(text: &str, alignment: Alignment, width: usize) -> String {
	text.lines()
		.map(|line| {
			let padding = match alignment {
				Alignment::Right => width - line.len(),
				Alignment::Center => (width - line.len()) / 2,
				Alignment::Left => 0,
			};
			
			" ".repeat(padding) + &line.to_string() + "\n"
		})
		.collect()
}

pub fn wrap_text(text: &str, width: usize) -> String {
	if width == 0 {
		return String::new();
	}

	let mut wrapped_text = String::new();
	let mut line_length = 0;
	// let mut amount_of_lines = 0;
	let mut previous_character = '\0';

	for c in text.chars() {
		// Fit text in `width` (+1 is for '-'), adding a new line when it goes over
		if line_length + 1 == width {
			// Add '-' if a word is broken
			if previous_character != ' ' && c != ' ' {
				wrapped_text.push('-');
			}

			wrapped_text.push('\n');
			line_length = 0;
			// amount_of_lines += 1;
		}

		// Skip leading spaces on lines
		// if line_length == 0 && c == ' ' {
		// 	continue;
		// }

		wrapped_text.push(c);

		// If the char is a newline (or carriage return), increase amount of lines and reset line length 
		if c == '\n' || c == '\r' {
			line_length = 0;
			// amount_of_lines += 1;
		} else {
			line_length += 1;
		}

		previous_character = c;
	}

	wrapped_text
}