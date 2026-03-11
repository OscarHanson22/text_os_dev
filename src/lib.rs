pub mod text_box;
pub mod button_schema;
pub mod frame;
pub mod interface;
pub mod key;
// pub mod drop_down;
 
use text_box::{TextBox, Alignment};
use key::{Key, KeyCode, ControlKeyState};
use button_schema::{Buttons, Binding, KeyBindings, ButtonGroup, ButtonSchema, ButtonInteract};
use frame::{Frame, FrameBuilder, Framing, AsFrame};
use interface::Interface;

use win32console::input::Coord;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Position {
	pub x: usize, 
	pub y: usize, 
}

impl Position {
	pub fn new(x: usize, y: usize) -> Self {
		Self {
			x, 
			y,
		}
	}

	pub fn add_y(&self, y: usize) -> Self {
		Self {
			x: self.x, 
			y: self.y + y, 
		}
	}

	pub fn sub_y(&self, y: usize) -> Self {
		Self {
			x: self.x, 
			y: self.y.saturating_sub(y),
		}
	}

	pub fn add_x(&self, x: usize) -> Self {
		Self {
			x: self.x + x, 
			y: self.y, 
		}
	}

	pub fn sub_x(&self, x: usize) -> Self {
		Self {
			x: self.x.saturating_sub(x), 
			y: self.y,
		}
	}

	pub fn add(&self, other: &Self) -> Self {
		Self {
			x: self.x + other.x, 
			y: self.y + other.y, 
		}
	}

	pub fn sub(&self, other: &Self) -> Self {
		Self {
			x: self.x.saturating_sub(other.x), 
			y: self.y.saturating_sub(other.y), 
		}
	}

	pub fn as_coord(&self) -> Coord {
		Coord::new(self.x.try_into().expect("Position x value is too large"), self.y.try_into().expect("Position y value is too large"))
	}
}

pub trait Positioned {
	fn position(&self) -> Position;
}

pub trait Rect {
	fn width(&self) -> usize;
	fn height(&self) -> usize;
}

pub trait Interact {
	/// Should return the button to be selected by the cursor. 
	fn handle_input(&mut self, key: Key) -> Vec<ControlFlow>;
}

pub trait Button: Rect + Positioned + AsFrame + ButtonClone {
	fn name(&self) -> &'static str;
	fn callback(&self) -> Vec<ControlFlow>;
}

pub trait Initialize {
	fn button_schema(&mut self) -> &mut ButtonSchema;

	fn initialize(&mut self) {
		let primer = Key::new(false, None, '\0', &[]);
		self.button_schema().handle_input(primer);
	}
}

pub trait Program: Initialize + Interact + AsFrame + DisplayCursor + Rect + Positioned + ProgramClone { }

/// Represents a node in a program tree. 
///
/// Each node has a parent (or none if it is the root), and none or more children. 
/// The "data" of each node is stored in the actual tree structure as the program tree node's key.  


#[derive(Copy, Clone, Debug)]
pub enum ProgramId {
	This, // wanted `Self` but its a keyword :(
	Parent, 
	Child(usize), 
	// Other(usize), 
}

#[derive(Copy, Clone, Debug)]
pub enum DependencyStatus {
	Independent,
	Dependent(AttentionLevel),  
}

#[derive(Copy, Clone, Debug)]
pub enum AttentionLevel {
	Passive, 
	Blocking, 
}

pub enum PositionChange {
	IncrementX, 
	IncrementY, 
	DecrementX, 
	DecrementY, 
}

pub enum DimensionChange {
	IncrementWidth,
	IncrementHeight,
	DecrementWidth, 
	DecrementHeight, 
}

#[derive(Clone)]
pub enum ControlFlow {
	Continue,
	Close(ProgramId), 
	Open(Box<dyn Program>, DependencyStatus),
	ChangePosition(PositionChange), 
	ChangeDimensionsBy(Change, Dimensions),
}

ControlFlow::ChangePositionBy(Change::Increase, Position::new(0, 1))

/* BLANKET IMPLEMENTATATATATATION OF AsFrame FOR ANYTHING THAT IMPLEMENTS Display !!!!! */

// impl<T> AsFrame for T 
// where 
// 	T: std::fmt::Display + ?Sized,
// {
// 	fn as_frame(&self) -> Frame {
// 		let string = self.to_string();
// 		let (lines, lengths): (Vec<&str>, Vec<usize>) = string.lines()
// 			.map(|line| (line, line.len()))
// 			.unzip();

// 		let longest_line_length = lengths.into_iter().max().unwrap_or(0);
// 		let rows: Vec<Vec<char>> = lines.into_iter().map(|line| format!("{}{}", line, " ".repeat(longest_line_length - line.len())).chars().collect::<Vec<char>>()).collect();

// 		Frame::from(rows)
// 	}
// }

pub struct InputManager;

impl InputManager {
	pub fn start(mut interface: Interface) {
	    use win32console::{console::WinConsole, input::{Coord, KeyEvent}};
	    use std::time::{Duration, Instant};
	    use std::thread;
	    use std::sync::mpsc::channel;

	    let FRAME_TIME = Duration::from_millis(100);

	    let (sender, receiver) = channel();

	    thread::spawn(move || {
	        loop {
	            if let KeyEvent(key) = WinConsole::input().read_single_input().unwrap() {
	                let key = Key::from(key);
	                // println!("{key:?}");
	                // println!("{}", key.is_enhanced_key());
	                sender.send(key).unwrap();
	            }
	        }
	    });

	    WinConsole::output().clear();
        println!("\x1b[?25l"); // ANSI escape code that hides the cursor

	    'main: loop {
	        let timer = Instant::now();

	        loop {
	            for key in receiver.try_iter() {
	                let control_flows = interface.handle_input(key);

		    		for control_flow in control_flows {
						match control_flow {
							ControlFlow::Close(_) => break 'main,
							_ => (),
						};
					}
	            }

	            if timer.elapsed() >= FRAME_TIME {
					WinConsole::output().set_cursor_position(Coord::new(0, 0));
					print!("{}", interface.as_frame());
					if let Some(current_program) = interface.current_program() {
						current_program.display_cursor();
					}
	                break;
	            }
	        }
	    }

		// clear screen when exiting
	    WinConsole::output().clear();
	    println!("\x1b[?25h"); // ANSI escape code that shows the cursor
	}
}


// #[derive(Copy, Clone, Debug)]
// pub struct Cursor {
// 	width: usize, 
// 	height: usize, 
// 	position: Position, 
// 	visible: bool,
// }

#[derive(Clone, Debug)]
pub struct Cursor {
	positions: Vec<Position>, 
	// flip: usize, 
	visible: bool, 
}

impl Cursor {
	pub fn new(positions: Vec<Position>) -> Self {
		Self {
			positions,
			visible: true,
		}
	}

	fn positions(width: usize, height: usize, from_position: Position) -> Vec<Position> {
		0..width.zip(0..height).map(|(w, h)| at_position.add(Position::new(w, h))).collect();
	}

	pub fn with_dimensions(width: usize, height: usize, at_position: Position) -> Self {
		let positions = Self::positions(width, height, at_position);

		Self::new(positions)
	}

	pub fn add_position(&mut self, position: Position) {
		self.positions.push(position);
	}

	pub fn remove_position(&mut self, position: Position) {
		if let Some(position_index) = self.positions.iter().position(|pos| pos == position) {
			let _ = self.positions.remove(position_index);
		}
	}

	pub fn clear(&mut self) {
		self.positions = Vec::new();
	}

	pub fn add_box(&mut self, box_width: usize, box_height: usize, box_position: Position) {
		for position in Self::positions(box_width, box_height, box_position) {
			self.add_position(position);
		}
	}

	pub fn remove_box(&mut self, box_width: usize, box_height: usize, box_position: Position) {
		for position in Self::positions(box_width, box_height, box_position) {
			self.remove_position(position);
		}
	}

	pub fn toggle_visibility(&mut self) {
		self.visible = !self.visible;
	} 

	pub fn display(&mut self) {
		use win32console::{console::WinConsole, structs::console_color::ConsoleColor};

		if !self.visible {
			return;
		}

		let attribute: Vec<u16> = (0..self.width).map(|_| ConsoleColor::White.as_background_color()).collect();

		for position in &self.positions {
			WinConsole::output()
				.write_output_attribute(
					&[ConsoleColor::White.as_background_color()],
					position.as_coord()
				);
		}
	}
}

impl Cursor {
	pub fn new(width: usize, height: usize, position: Position) -> Self {
		Self {
			width, 
			height, 
			position,
			visible: true,
		}
	}

	pub fn default() -> Self {
		Self::new(0, 0, Position::new(0, 0))
	}

	pub fn go_to_button(&mut self, button: &Box<dyn Button>) {
		self.position = button.position();
		self.width = button.width();
		self.height = button.height();
	}

	pub fn go_to_position(&mut self, position: Position) {
		self.position = position;
	}

	pub fn set_size_to(&mut self, width: usize, height: usize) {
		self.width = width;
		self.height = height;
	}

	pub fn add_position(&self, position: Position) -> Self {
		Self {
			position: self.position.add(&position),
			..*self
		}
	}

	pub fn display(&self) {
		use win32console::{console::WinConsole, structs::console_color::ConsoleColor};

		if !self.visible {
			return;
		}

		let attributes: Vec<u16> = (0..self.width).map(|_| ConsoleColor::White.as_background_color()).collect();

		for y in 0..self.height {
			WinConsole::output().write_output_attribute(&attributes, self.position.add_y(y).as_coord());
		}
	}

	pub fn toggle_visibility(&mut self) {
		self.visible = !self.visible;
	} 
}

impl Rect for Cursor {
	fn width(&self) -> usize { self.width }
	fn height(&self) -> usize { self.height }
}

pub trait ProgramClone {
	fn clone_box(&self) -> Box<dyn Program>;
}

impl<T> ProgramClone for T
where
	T: ?Sized + Program + Clone + 'static, 
{
	fn clone_box(&self) -> Box<dyn Program> {
		Box::new(self.clone())
	}
}

impl Clone for Box<dyn Program> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

pub trait ButtonClone {
	fn clone_box(&self) -> Box<dyn Button>;
}

impl<T> ButtonClone for T
where
	T: ?Sized + Button + Clone + 'static, 
{
	fn clone_box(&self) -> Box<dyn Button> {
		Box::new(self.clone())
	}
}

impl Clone for Box<dyn Button> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}



// pub struct ProgramBuilder<B>
// where 
// 	B: Button,
// {
// 	width: usize, 
// 	height: usize, 
// 	position: Position,
// 	buttons: Vec<Box<B>>, 
// }

// impl<B> ProgramBuilder<B>
// where 
// 	B: Button,
// {
// 	pub fn with_dimensions(width: usize, height: usize) -> Self {
// 		Self {
// 			width, 
// 			height, 
// 			position: Position::new(0, 0), 
// 			buttons: Vec::new(), 
// 		}
// 	}

// 	pub fn add_button(mut self, button: B) -> Self {
// 		self.buttons.push(Box::new(button));
// 		self
// 	}
// }

// pub trait MinimumDimensions {
// 	fn minimum_dimensions 

#[derive(Clone)]
struct OkButton {
	name: &'static str, 
	callbacks: Vec<ControlFlow>,
	position: Position, 
}

impl OkButton {
	fn new(name: &'static str, callbacks: Vec<ControlFlow>, position: Position) -> Self {
		Self {
			name, 
			callbacks,
			position, 
		}
	}
}

impl Rect for OkButton {
	fn width(&self) -> usize { 6 }
	fn height(&self) -> usize { 3 }
}

impl Positioned for OkButton {
	fn position(&self) -> Position { self.position }
}

impl AsFrame for OkButton {
	fn as_frame(&self) -> Frame {
 		FrameBuilder::with_dimensions(self.width(), self.height())
 			.add_border()
 			.add_text_box(
 				&TextBox::new("OK", self.width().saturating_sub(2), self.height().saturating_sub(2), Position::new(1, 1))
 					.alignment(Alignment::Center), 
 			)
 			.to_frame()
 	}
}

impl Button for OkButton {
	fn name(&self) -> &'static str {
		self.name
	}

	fn callback(&self) -> Vec<ControlFlow> {
		self.callbacks.clone()
	}
}

#[derive(Clone, Debug)]
struct CancelButton {
	name: &'static str, 
	position: Position, 
}

impl CancelButton {
	fn new(name: &'static str, position: Position) -> Self {
		Self {
			name,  
			position, 
		}
	}
}

impl Rect for CancelButton {
	fn width(&self) -> usize { 10 }
	fn height(&self) -> usize { 3 }
}

impl Positioned for CancelButton {
	fn position(&self) -> Position { self.position }
}

impl AsFrame for CancelButton {
	fn as_frame(&self) -> Frame {
 		FrameBuilder::with_dimensions(self.width(), self.height())
 			.add_border()
 			.add_text_box(
 				&TextBox::new("CANCEL", self.width().saturating_sub(2), self.height().saturating_sub(2), Position::new(1, 1))
 					.alignment(Alignment::Center), 
 			)
 			.to_frame()
 	}
}

impl Button for CancelButton {
	fn name(&self) -> &'static str {
		self.name
	}

	fn callback(&self) -> Vec<ControlFlow> {
		vec![ControlFlow::Close(ProgramId::This)]
	}
}

#[derive(Clone)]
pub struct Alert {
	width: usize, 
	height: usize,
	position: Position,  
	message: String, 
	cursor: Cursor,
	buttons: Buttons, 
	button_schema: ButtonSchema,
}

impl Alert {
	pub fn new(message: &str, callbacks: Vec<ControlFlow>, width: usize, height: usize, position: Position) -> Self {
		Self {
			message: message.to_string(), 
			width, 
			height, 
			position,
			cursor: Cursor::default(),
			buttons: Buttons::new(vec![
				Box::new(OkButton::new(
					"ok_button", 
					callbacks,
					Position::new(1, height - 3 - 1)
				)),
				Box::new(CancelButton::new(
					"cancel_button", 
					Position::new(width - 7 - 4, height - 1 - 3)
				))
			]),
			button_schema: ButtonSchema::new(
				vec![
					ButtonGroup::new(
						"buttons", 
						"cancel_button",
						vec!["ok_button", "cancel_button"], 
						KeyBindings::new(vec![
							(Key::LEFT, Binding::Prev), 
							(Key::RIGHT, Binding::Next),
						]), 
						true
					)
				], 
				"buttons", 
			),
		}
	}
}

impl Rect for Alert {
	fn width(&self) -> usize { self.width }
	fn height(&self) -> usize { self.height }
}

impl Positioned for Alert {
	fn position(&self) -> Position { self.position }
}

impl AsFrame for Alert {
	fn as_frame(&self) -> Frame {
 		FrameBuilder::with_dimensions(self.width(), self.height())
 			.add_border()
 			.add_text_box(
 				&TextBox::new(&self.message, self.width().saturating_sub(4), self.height().saturating_sub(2), Position::new(2, 1))
 					.alignment(Alignment::Center), 
 			)
 			.add_buttons(&self.buttons)
 			.to_frame()
 	}
}

impl Interact for Alert {
	fn handle_input(&mut self, key: Key) -> Vec<ControlFlow> {
		let selected_button = self.button_schema.handle_input(key);

		self.cursor.go_to_button(&self.buttons.get(selected_button));

		if key.key_down() {
			match key.key_code() {
				Some(KeyCode::Enter) => return self.buttons.get(selected_button).callback(), 
				_ => (), 
			}
		}

		vec![ControlFlow::Continue]
	}
}

pub trait DisplayCursor: Positioned {
	fn cursor(&mut self) -> &mut Cursor; 

	fn display_cursor(&mut self) {
		let position = self.position();
		let cursor = self.cursor().add_position(position);
		// self.cursor().toggle_visibility();
		cursor.display();
	}
}

impl DisplayCursor for Alert {
	fn cursor(&mut self) -> &mut Cursor {
		&mut self.cursor
	}
}

impl Initialize for Alert {
	fn button_schema(&mut self) -> &mut ButtonSchema {
		&mut self.button_schema
	}
}

impl Program for Alert { }

#[derive(Clone)]
struct CloseButton {
	name: &'static str,
	position: Position,
	host_width: usize, 
	host_height: usize, 
	host_position: Position,  
}

impl CloseButton {
	fn new(name: &'static str, position: Position, host_width: usize, host_height: usize, host_position: Position) -> Self {
		Self {
			name,
			position,
			host_width, 
			host_height, 
			host_position,
		}
	}
}

impl Rect for CloseButton {
	fn width(&self) -> usize { 1 }
	fn height(&self) -> usize { 1 }
}

impl Positioned for CloseButton {
	fn position(&self) -> Position { self.position }
}

impl AsFrame for CloseButton {
	fn as_frame(&self) -> Frame {
		'X'.as_frame()
	}
}

impl Button for CloseButton {
	fn name(&self) -> &'static str {
		self.name
	}

	fn callback(&self) -> Vec<ControlFlow> {
		let alert_width = 22;
		let alert_height = 9;
		let alert_position = Position::new(self.host_width / 2 - alert_width / 2, self.host_height / 2 - alert_height / 2); 

		vec![ControlFlow::Open(
			Box::new(Alert::new(
				"Are you sure you want to quit this program?", 
				vec![ControlFlow::Close(ProgramId::Parent)], 
				alert_width, 
				alert_height, 
				alert_position
			)), 
			DependencyStatus::Dependent
		)]
	}
}


#[derive(Clone)]
pub struct BasicProgram {
	width: usize, 
	height: usize, 
	position: Position, 
	name: String, 
	cursor: Cursor,
	buttons: Buttons,
	button_schema: ButtonSchema,
}

impl BasicProgram {
	pub fn new(name: &str, width: usize, height: usize, position: Position) -> Self {
		Self {
			width, 
			height, 
			position, 
			name: name.to_string(), 
			cursor: Cursor::default(),
			buttons: Buttons::new(
				vec![Box::new(
					CloseButton::new(
						"close_button", 
						Position::new(width - 2, 1),
						width, 
						height,
						position,
					)
				)]
			),
			button_schema: ButtonSchema::new(
				vec![
					ButtonGroup::new(
						"top_buttons", 
						"close_button", 
						vec!["close_button"], 
						KeyBindings::empty(), 
						false
					)
				], 
				"top_buttons", 
			),
		}
	}
}

impl Rect for BasicProgram {
	fn width(&self) -> usize { self.width }
	fn height(&self) -> usize { self.height }
}

impl Positioned for BasicProgram {
	fn position(&self) -> Position { self.position }
}

impl AsFrame for BasicProgram {
	fn as_frame(&self) -> Frame {
		FrameBuilder::with_dimensions(self.width(), self.height())
			.add_border()
			.add_framing(Framing::Horizontal(self.width()), Position::new(0, 2))
			.add_framing(Framing::Vertical(3), Position::new(self.width().saturating_sub(3), 0))
			.add_text_box(
				&TextBox::new(&self.name, self.width().saturating_sub(4), 1, Position::new(1, 1))
 					.alignment(Alignment::Left), 
 			)
 			.add_buttons(&self.buttons)
 			.to_frame()
 	}
}

impl Interact for BasicProgram {
	fn handle_input(&mut self, key: Key) -> Vec<ControlFlow> {
		let selected_button = self.button_schema.handle_input(key);

		self.cursor.go_to_button(&self.buttons.get(selected_button));

		if key.key_down() {
			match key.key_code() {
				Some(KeyCode::Enter) => {
					return self.buttons.get(selected_button).callback();
				}
				_ => (), 
			}
		}

		vec![ControlFlow::Continue]
	}
}

impl Initialize for BasicProgram {
	fn button_schema(&mut self) -> &mut ButtonSchema {
		&mut self.button_schema
	}
}

impl DisplayCursor for BasicProgram {
	fn cursor(&mut self) -> &mut Cursor {
		&mut self.cursor
	}
}

impl Program for BasicProgram { }

#[derive(Clone)]
pub struct GhostButton(&'static str);

impl Rect for GhostButton {
	fn width(&self) -> usize { 0 }
	fn height(&self) -> usize { 0 }
}

impl Positioned for GhostButton {
	fn position(&self) -> Position { Position::new(0, 0) }
}

impl AsFrame for GhostButton {
	fn as_frame(&self) -> Frame {
		Frame::new(0, 0)
	}
}

impl Button for GhostButton {
	fn name(&self) -> &'static str { self.0 }
	fn callback(&self) -> Vec<ControlFlow> { vec![ControlFlow::Continue] }
}

#[derive(Clone)]
pub struct TextEditor {
	width: usize, 
	height: usize, 
	position: Position, 
	name: String, 
	cursor: Cursor,
	buttons: Buttons,
	button_schema: ButtonSchema,
	text_box: TextBox,
}

impl TextEditor {
	pub fn new(name: &str, width: usize, height: usize, position: Position) -> Self {
		Self {
			width, 
			height, 
			position, 
			name: name.to_string(), 
			cursor: Cursor::default(),
			buttons: Buttons::new(vec![
				Box::new(
					CloseButton::new(
						"close_button", 
						Position::new(width - 2, 1),
						width, 
						height,
						position,
					)
				), 
				Box::new(GhostButton("ghost_button")),
			]),
			button_schema: ButtonSchema::new(
				vec![
					ButtonGroup::new(
						"top_buttons", 
						"close_button", 
						vec!["close_button"], 
						KeyBindings::new(vec![
							(Key::CTRL_LEFT, Binding::GoToButtonGroup("typing")), 
						]), 
						false
					), 
					ButtonGroup::new(
						"typing", 
						"ghost_button", 
						vec!["ghost_button"], 
						KeyBindings::new(vec![
							(Key::CTRL_RIGHT, Binding::GoToButtonGroup("top_buttons")), 
						]), 
						false
					)
				], 
				"typing", 
			),
			text_box: TextBox::new("", width - 4, height - 4, Position::new(2, 3)),
		}
	}
}

impl Rect for TextEditor {
	fn width(&self) -> usize { self.width }
	fn height(&self) -> usize { self.height }
}

impl Positioned for TextEditor {
	fn position(&self) -> Position { self.position }
}

impl AsFrame for TextEditor {
	fn as_frame(&self) -> Frame {
		FrameBuilder::with_dimensions(self.width(), self.height())
			.add_border()
			.add_framing(Framing::Horizontal(self.width()), Position::new(0, 2))
			.add_framing(Framing::Vertical(3), Position::new(self.width().saturating_sub(3), 0))
			.add_text_box(
				&TextBox::new(&self.name, self.width().saturating_sub(4), 1, Position::new(1, 1))
 					.alignment(Alignment::Center), 
 			)
 			.add_text_box(&self.text_box)
 			.add_buttons(&self.buttons)
 			.to_frame()
 	}
}

impl Interact for TextEditor {
	fn handle_input(&mut self, key: Key) -> Vec<ControlFlow> {
		let selected_button = self.button_schema.handle_input(key);

		if selected_button != "ghost_button" {
			self.cursor.go_to_button(&self.buttons.get(selected_button));
		}

		if key.key_down() {
			match key.key_code() {
				Some(KeyCode::Enter) => {
					return self.buttons.get(selected_button).callback();
				}
				_ => (), 
			}

			if selected_button != "ghost_button" {
				return vec![ControlFlow::Continue];
			}

			let control_flows = self.text_box.handle_input(key);

			for control_flow in control_flows {
				match control_flow {
					ControlFlow::SetCursorSizeTo(width, height) => self.cursor.set_size_to(width, height), 
					ControlFlow::MoveCursorToPosition(position) => self.cursor.go_to_position(position),
					_ => (), 
				}
			}


		}

		vec![ControlFlow::Continue]
	}
}

impl Initialize for TextEditor {
	fn button_schema(&mut self) -> &mut ButtonSchema {
		&mut self.button_schema
	}
}

impl DisplayCursor for TextEditor {
	fn cursor(&mut self) -> &mut Cursor {
		&mut self.cursor
	}
}

impl Program for TextEditor { }