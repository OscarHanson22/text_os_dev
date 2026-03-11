use crate::{Position, Cursor};
use crate::frame::{Frame, AsFrame, FrameBuilder};
use crate::text_box::TextBox;


pub trait ProgramInteract {
	fn handle_input(&self, key: Key) -> Vec<ControlFlow>;
}

pub trait ProgramElementInteract { // for text boxes, buttons, dropdowns etc...
	fn handle_input(&self, key: Key, program: &mut Box<dyn Program>) -> Vec<ControlFlow>;
	// needs access to the program so that it can get and change height width, position, cursor info, etc, element schemas
}

pub trait ProgramElement: ProgramElementInteract + AsFrame + Rect + Hash {
	fn can_have_focus(&self) -> bool; // determines whether the program element can be interacted with (for static text boxes)
	// or should static objects just not be put in a schema
	// fn name(&self) -> &'static str; name method is annoying because is becomes part of the constructor
}

pub enum Comparison {
	LargerThan, 
	SmallerThan, 
}

pub enum Alignment {
	LeftIn, 
	CenteredIn, 
	RightIn, 
}

pub struct AreaTree {
	// defines the area hierarchy such that resizing one area will resize all of its children. 

pub struct AreaBuilder { 
		

// Area deals with actual numbers, not proportions
pub struct Area {
	dimensions: Dimensions,  
	minimum_size: Dimensions, 


pub struct Placement {
	element: &'static str, 
	alignment: Alignment, 
	in_area

pub enum SizingRule {
	Placement(&'static str, Alignment, isize, &'static str), // Placement("text_box", Alignment::CenteredIn, 2, "area_a"); 

	Size(usize, Comparison, &'static str), // Size(2, Comparison::SmallerThan, "area_a");
	SharesBorder(&'static str, 
	Group(usize, 

pub struct ProgramA {
	// width, 
	// height, 
	// position // will be moved to the interface?
	cursor, 
	program_elements, 
	program_element_schema, 


pub struct ProgramElements {
	elements: HashMap<&'static str, (ProgramElement, SizingDimensions, SizingPosition)>,
}

// pub struct Placement {
// 	placements: HashMap<&'static str, (SizingDimensions, SizingPosition)>,
// }

// impl Placement {
// 	new 

impl ProgramElements {
	pub fn new(elements: Vec<(&'static str, ProgramElement, SizingDimensions, SizingPosition)>) -> Self {
		let elements: HashMap<&'static str, (ProgramElement, SizingDimensions, SizingPosition)> = elements.into_iter()
			.map(|(name, program_element, sizing_dimensions, sizing_position)| 
				(name, (program_element, sizing_dimensions, sizing_position))
			)
			.collect();

		Self {
			elements, 
		}
	}

	pub fn get_element(&self, name: &'static str) -> ProgramElement {
		match self.elements.get(name) {
			Some((element, _, _)) => return element,
			None => panic!("ProgramElement: \"{}\" not found in program elements.", name);
		}
	}

	pub fn get_dimensions(&self, name: &'static str) -> SizingDimensions {
		match self.elements.get(name) {
			Some((_, sizing_dimensions, _)) => return sizing_dimensions,
			None => panic!("ProgramElement: \"{}\" not found in program elements.", name);
		}
	}

	pub fn get_position(&self, name: &'static str) -> SizingPosition {
		match self.elements.get(name) {
			Some((_, _, sizing_position)) => return sizing_position,
			None => panic!("ProgramElement: \"{}\" not found in program elements.", name);
		}
	}
}

impl<'a> IntoIterator for &'a ProgramElements {
    type Item = <&'a HashMap<&'static str, (ProgramElement, SizingDimensions, SizingPosition)> as IntoIterator>::Item;
    type IntoIter = <&'a HashMap<&'static str, (ProgramElement, SizingDimensions, SizingPosition)> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        (&self.buttons).into_iter()
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Binding {
	Next, 
	Prev, 
	GoToElement(&'static str),
	GoToElementGroup(&'static str), 
}

#[derive(Clone, Debug)]
pub struct KeyBindings {
	bindings: HashMap<Key, Binding>, 
}

impl KeyBindings {
	pub fn new(bindings: Vec<(Key, Binding)>) -> Self {
		Self {
			bindings: bindings.into_iter().collect(),
		}
	}

	pub fn empty() -> Self {
		Self::new(Vec::new())
	}

	pub fn get(&self, key: Key) -> Option<Binding> {
		self.bindings.get(&key).copied()
	}
}

pub struct ProgramElementGroup { 
	name: &'static str,
	elements: Vec<&'static str>, 
	key_bindings: KeyBindings,
	cycle: bool, 
	current_element: usize, 
}

let mut buttons = ProgramElementGroup::named("buttons")
	.with_elements(vec!["resize_button", "minimize_button", "close_button"])
	.with_key_bindings(KeyBindings::new(vec![
		(Key::RIGHT, Binding::Next),
		(Key::Left, Binding::Prev),
	]))
	.start_on_element("resize_button");

impl ProgramElementGroup {
	pub fn new(name: &'static str, elements: Vec<&'static str>, key_bindings: KeyBindings, cycle: bool, current_element: usize) -> Self {
		Self {
			name, 
			elements, 
			key_bindings, 
			cycle, 
			current_element
		}
	}

	// pub fn name(&self) -> &'static str {
	// 	self.name
	// }

	pub fn is_empty(&self) -> bool {
		self.elements.len() == 0
	}

	pub fn named(name: &'static str) -> Self {
		Self {
			name, 
			elements: Vec::new(), 
			key_bindings: KeyBindings::empty(), 
			cycle: false, 
			current_element: 0, 
		}
	}

	pub fn with_elements(mut self, elements: Vec<&'static str>) -> Self {
		self.elements = elements;
		self
	}

	pub fn with_key_bindings(mut self, key_bindings: KeyBindings) -> Self {
		self.key_bindings = KeyBindings;
		self
	}

	pub fn cycle(mut self, cycle: bool) -> Self {
		self.cycle = cycle;
		self
	}

	pub fn start_on_element(mut self, element_name: &'static str) -> Self {
		self.go_to_element(element_name);
		self
	}

	pub fn current_element(&self) -> &'static str {
		if self.elements.len() == 0 || self.current_element > self.elements.len() - 1 {
			panic!("Current element index: \"{}\" is too large for elements: \"{:?}\" with length: \"{}\".", self.current_element, self.elements, self.elements.len());
		}

		self.current_element;
	} 

	/// Set the current element to the elements with `element_name`. 
	pub fn go_to_element(&mut self, element_name: ButtonName) {
		self.current_element = match self.elements.iter().position(|name| *name == element_name) {
			Some(index) => index,
			None => panic!("Program Element: \"{element_name}\" not found in elements: \"{:?}\"", self.elements),
		};
	} 

	/// Sets the current element to the next one in elements. 
	///
	/// If cycle is on, it will return to the first element if the last element was currently selected. 
	pub fn next(&mut self) {
		if self.cycle && self.current_element == self.elements.len() - 1 {
			self.current_element = 0;
		} else if self.current_element < self.elements.len() - 1 {
			self.current_element += 1;
		}
	}

	/// Sets the current element to the previous one in elements. 
	///
	/// If cycle is on, it will return to the last element if the first element was currently selected. 
	pub fn prev(&mut self) {
		if self.cycle && self.current_element == 0 {
			self.current_element = self.elements.len() - 1;
		} else if self.current_element > 0 {
			self.current_element -= 1;
		}
	}

	pub fn handle_input(&mut self, key: Key) -> &'static str {
		if let Some(binding) = self.bindings.get(key) {
			match binding {
				Binding::Next => self.next(),
				Binding::Prev => self.prev(), 
				Binding::GoToElement(element_name) => self.go_to_button(element_name),
				Binding::GoToElementGroup(element_group_name) => return element_group_name,
			}
		}

		self.name
	}
}

// impl ProgramElementInteract for ProgramElementGroup {
// 	fn handle_input(&self, key: Key, program: &mut Box<dyn Program>) -> &'static str

pub struct ProgramElementSchema {
	program_element_groups: HashMap<&'static str, ProgramElementGroup>,
	current_group: &'static str,
}

impl ProgramElementSchema {
	pub fn new(program_element_groups: Vec<ProgramElementGroup>, start_on_program_element_group: &'static str) -> Self {
		if program_element_groups.len() == 0 {
			panic!("Cannot instantiate a Program Element Schema with no elements.");
		}

		Self {
			program_element_groups: program_element_groups.into_iter().collect(),
			current_group: start_on_program_element_group, 
		}
	}

	pub fn with_groups(program_element_groups: Vec<(&'static str, ProgramElementGroup)) -> Self {
		if program_element_groups.len() == 0 {
			panic!("Cannot instantiate a Program Element Schema with no elements.");
		}

		Self::new(program_element_groups, program_element_groups[0])
	}

	pub fn start_on_group(mut self, group_name: &'static str) -> Self { 
		if !self.program_element_groups.contains_key(program_element_group_name) {
			panic!("Program element group name: \"{program_element_group_name}\" not found in program element groups.");
		}

		self.current_group = group_name;
		self
	}

	pub fn current_group(&mut self) -> &mut ProgramElementGroup {


	pub fn handle_input(&mut self, key: Key) -> &'static str {
		let group_name = self.current_group().handle_input(key);

		if !self.program_element_groups.contains_key(group_name) {
			panic!("Program element group: \"{group}\" not found in groups: \"{:?}\"." self.program_element_groups);
		}

		self.current_group = group_name;
		self.current_button_group().current_button()
	}
}


// pub struct Placement {
// 	elements: HashMap<&'static str, >,
// }

pub struct Proportion(f64);

impl Proportion {
	pub fn new(proportion: f64) -> Self {
		if proportion < 0.0 || proportion > 1.0 {
			panic!("Proportion should be in range [0.0, 1.0], received: {proportion}");
		}

		Self(proportion)
	}

	pub fn of(&self, amount: usize) -> usize {
		(amount as f64 / self.0).round() as usize
	}
}

pub struct SizingPosition {
	x: Sizing, 
	y: Sizing, 
}

impl SizingPosition {
	pub fn new(x: Sizing, y: Sizing) -> Self {
		Self {
			x, 
			y, 
		}
	}

	pub fn to_position(&self, x: usize, y: usize) -> Position {
		Position::new(self.x.from(x), self.y.from(y))
	}
}

pub struct SizingDimensions {
	width: Sizing, 
	height: Sizing, 
}

impl SizingDimensions {
	pub fn new(width: Sizing, height: Sizing) -> Self {
		Self {
			width, 
			height, 
		}
	}

	pub fn to_dimensions(&self, width: usize, height: usize) -> Dimensions {
		(self.width.from(width), self.height.from(height))
	}
}

pub enum Sizing {
	Dynamic(Proportion), // proportion // for items that should change size with the program
	Static(usize), // size // for items that should stay the same size regardless of program size changes
}

impl Sizing {
	pub fn from(&self, amount: usize) -> usize {
		match self {
			Self::Dynamic(proportion) => proportion.of(amount),
			Self::Static(size) => size, 
		}
	}
}

#[derive(Copy, Clone, Debug)]
pub enum Direction {
	Vertical,
	Horizontal,
}

#[derive(Copy, Clone, Debug)]
pub enum Spacing {
	Even(Direction, Proportion), // [O   O   O]
	EvenWithOutside(Direction, Proportion), // [ O  O  O ] true even spacing
}

/*

pub struct ProgramElements {
	elements: HashMap<Box<dyn ProgramElements>, Position>> // ugh Box
enables

pub struct ScrollView // enables a scrollable view for other program elements?


pub struct TextBox
pub struct ScrollableTextBox
pub struct TextButton
pub struct BorderedButton <- note buttons no longer implement Button trait YAY!
pub struct ChooseDropDown
pub struct DropDown

also the issue with drop downs is that they are collections of program elements themselves, so they should probably use an inner schema as well to keep things more simple. 

they all implement ProgramElement

button schema should become program element schema, and handle how stuff should be interacted with and moved between
next is figuring out placement
also make the cursor more dynamic (let programs customize the shape and individual pixels of the cursor (for highlighting and such)

*/

pub trait ProgramElementInteract { // for text boxes, buttons, dropdowns etc...
	fn handle_input(&self, key: Key, program: &mut Box<dyn Program>) -> Vec<ControlFlow>;
	// needs access to the program so that it can get and change height width, position, cursor info, etc, element schemas
}



// to be implemented for interactive textboxes, button schemas, dropdowns, buttons themselves

// pub trait Cursorable {
// 	fn fit_cursor(&self, cursor: &mut Cursor); // would also need to know about the cursor. 
// 	// better to wrap with input so the struct doesn't have to store the info
// }

// maybe a different kind of interact for program building blocks

pub struct DropDown<T> {
	width: usize, 
	closed_height: usize,  
	position: Position, 
	options: Vec<(&'static str, T)>,
	selected_option: usize,
	pub open: bool, 
}

impl<T> DropDown<T> {
	pub fn new(options: Vec<(&'static str, T)>, width: usize, closed_height: usize, position: Position) -> Self {
		Self {
			width, 
			closed_height,
			position, 
			options, 
			selected_option: 0,
			open: false,
		}
	}
}

impl<T> AsFrame for DropDown<T> {
	fn as_frame(&self) -> Frame {
		if self.open {
			let mut frame_builder = FrameBuilder::with_dimensions(self.width, self.closed_height + self.options.len() * (self.closed_height - 2))
				.add_frame('v'.as_frame(), Position::new(2, 1))
				.add_text_box(&TextBox::new(
					self.options[self.selected_option].0, 
					self.width - 6, 
					self.closed_height - 2, 
					Position::new(4, 1),
				))
				.add_border();

			for (index, (name, _option)) in self.options.iter().enumerate() {
				frame_builder = frame_builder.add_text_box(&TextBox::new(
					name, 
					self.width - 6, 
					self.closed_height - 2, 
					Position::new(4, self.closed_height - 1 + index * (self.closed_height - 2)),
				));
			}

			frame_builder.to_frame()
		} else {
			FrameBuilder::with_dimensions(self.width, self.closed_height)
				.add_frame('>'.as_frame(), Position::new(2, 1))
				.add_text_box(&TextBox::new(
					self.options[self.selected_option].0, 
					self.width - 6, 
					self.closed_height - 2, 
					Position::new(4, 1),
				))
				.add_border()
				.to_frame()
		}
	}
}

/*
// Choose dropdown
pub struct ChooseDropDown { ... }
┌──────────────────┐
│ > Item X         │ highlighted on select
└──────────────────┘


┌──────────────────┐
│ v Item 1         │
│ Item 1           │ // highlighted when opened
│ Item 2; longer   │
│ Item 3           │
│ Item 4           │
└──────────────────┘

// multiple select?
// probably not, would look weird and would doesn't have a use case
┌──────────────────┐
│ v Item 1         │
│ - Item 1         │ // highlighted when opened
│   Item 2; longer │
│ - Item 3         │
│   Item 4         │
└──────────────────┘

*/




// find a way to make easy to read and make placement schemes
// also find a way to make keybinds system
// interface passes keybind list to program, program adds its own to the system's and uses them (to prevent replacements. ?)


pub struct Placement {
	elements: Vec<Frame>,
	spacing: Spacing, 
}

impl Placement {
	pub fn new(elements: Vec<Frame>, spacing: Spacing) -> Self {
		Self {
			elements,
			spacing, 
		}
	}

	pub fn positions(self, from_position: Position) -> Vec<Frame, Position> {
		let amount_of_spaces = match &self.spacing {
			Spacing::EvenSpacing(_) => self.elements.len() - 1, 
			Spacing::EvenSpacingWithOutside(_) => self.elements + 1, 
		};

		let length = self.spacing.1;

		match &self.spacing {
			spacing

		elements.into_iter().map(|frame| (frame, Position::new(
	}
} 

impl IntoIter for &Placement {

impl<'a> IntoIterator for &'a Placement {
    type Item = <&'a Vec<(Frame, Position)> as IntoIterator>::Item;
    type IntoIter = <&'a Vec<(Frame, Position)> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        (self.elements.iter().map(|frame| (frame)).into_iter()
    }
}


pub fn add_placement(&mut self, placement: Placement) -> Self {


/*

Generic dropdown
dropdown to side
dropdown to dropdown

pub struct DropDown {
	name: &'static str, 
	buttons: Vec<Button> 
}

// program is responsible for placements via Placement (to be implemented above...)

*/

pub struct ButtonElement {
	name: &'static str, 
	message: String, 
	width: usize, 
	height: usize, 
	callback: Vec<ControlFlow>, 
}

impl ButtonElement {
	pub fn new(name: &'static str, message: String, width: usize, height: usize, callback: Vec<ControlFlow>) -> Self {
		Self {
			name, 
			message, 
			width, 
			height, 
			callback, 
		}
	}
}

pub enum Button {
	BorderedButton(ButtonElement),
	TextButton(ButtonElement),
}

impl AsFrame for Button {
	fn as_frame(&self) -> Frame {
		match self {
			Button::BorderedButton(button_element) => 
				FrameBuilder::with_dimensions(width, height)
					.add_frame(TextBox::new().as_frame(), 

