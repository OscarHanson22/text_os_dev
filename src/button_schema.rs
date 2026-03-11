use crate::Button;
use crate::key::{Key, KeyCode};

use std::collections::HashMap;

type ButtonName = &'static str;
type ButtonGroupName = &'static str;

pub trait ButtonInteract {
	fn handle_input(&mut self, key: Key) -> &'static str;
}

#[derive(Clone)]
pub struct Buttons {
	buttons: HashMap<ButtonName, Box<dyn Button>>,
}

impl Buttons {
	pub fn new(buttons: Vec<Box<dyn Button>>) -> Self {
		Self {
			buttons: buttons.into_iter()
				.map(|button| (button.name(), button))
				.collect(),
		}
	}

	pub fn get(&self, button_name: ButtonName) -> &Box<dyn Button> {
		match self.buttons.get(button_name) {
			Some(boxed_button) => return &boxed_button,
			None => panic!("Button not found: {}.", button_name),
		}
	}

	// less difficult to keep track of but goes with the other methods removed. 
	// pub fn add(&mut self, button: Box<dyn Button>) {
	// 	if self.buttons.contains_key(button.name()) {
	// 		panic!("`button` ({}) already exists in buttons.", button.name());
	// 	}

	// 	self.insert(button.name(), button);
	// }
}

impl<'a> IntoIterator for &'a Buttons {
    type Item = <&'a HashMap<ButtonName, Box<dyn Button>> as IntoIterator>::Item;
    type IntoIter = <&'a HashMap<ButtonName, Box<dyn Button>> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        (&self.buttons).into_iter()
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Binding {
	Next, 
	Prev, 
	GoToButton(ButtonName),
	GoToButtonGroup(ButtonGroupName), 
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

	// hard to keep track of 
	// pub fn add(&mut self, key_code: KeyCode, binding: Binding) {
	// 	self.bindings.insert(key_code, binding);
	// }
}

#[derive(Clone, Debug)]
pub struct ButtonGroup {
	name: ButtonGroupName,
	buttons: Vec<ButtonName>, 
	pub bindings: KeyBindings,
	cycle: bool, 
	current_button: usize, 
}

impl ButtonGroup {
	pub fn new(name: ButtonGroupName, start_button_name: ButtonName, buttons: Vec<ButtonName>, bindings: KeyBindings, cycle: bool) -> Self {
		let start_button_index = match buttons.iter().position(|button_name| *button_name == start_button_name) {
			Some(start_button_index) => start_button_index,
			None => panic!("`start_button_name` ({start_button_name}) not found in `buttons` ({buttons:#?})."),
		};

		Self {
			name, 
			buttons, 
			bindings, 
			cycle,
			current_button: start_button_index, 
		}
	}

	// will probably just use the ghost button in the future. 
	// pub fn empty(name: ButtonGroupName) -> Self {
	// 	Self {
	// 		name, 
	// 		buttons: Vec::new(), 
	// 		bindings: KeyBindings::empty(), 
	// 		cycle: false, 
	// 		current_button: 0, 
	// 	}
	// }

	/// Returns the name of the button group. 
	pub fn name(&self) -> ButtonGroupName {
		&self.name
	}

	/// Returns the current button's name.
	pub fn current_button(&self) -> ButtonName {
		self.buttons[self.current_button]
	}

	/// Set the current button to the button with `button_name`. 
	pub fn go_to_button(&mut self, button_name: ButtonName) {
		self.current_button = match self.buttons.iter().position(|name| *name == button_name) {
			Some(index) => index,
			None => panic!("Button: \"{}\" not found in button group.", button_name),
		};
	} 

	/// Sets the current button to the next one in buttons. 
	///
	/// If cycle is on, it will return to the first button if the last button was currently selected. 
	pub fn next(&mut self) {
		if self.cycle && self.current_button == self.buttons.len() - 1 {
			self.current_button = 0;
		} else if self.current_button < self.buttons.len() - 1 {
			self.current_button += 1;
		}
	}

	/// Sets the current button to the previous one in buttons. 
	///
	/// If cycle is on, it will return to the last button if the first button was currently selected. 
	pub fn prev(&mut self) {
		if self.cycle && self.current_button == 0 {
			self.current_button = self.buttons.len() - 1;
		} else if self.current_button > 0 {
			self.current_button -= 1;
		}
	}
}

impl ButtonInteract for ButtonGroup {
	/// Returns the name of the button group the hosting button schema should select. 
	fn handle_input(&mut self, key: Key) -> ButtonGroupName {
		if !key.key_down() {
			return self.name;
		}

		if let Some(binding) = self.bindings.get(key) {
			match binding {
				Binding::Next => self.next(),
				Binding::Prev => self.prev(), 
				Binding::GoToButton(button_name) => self.go_to_button(button_name),
				Binding::GoToButtonGroup(button_group_name) => return button_group_name,
			}
		}

		self.name
	}
}

#[derive(Clone, Debug)]
/// A representation of how input should select buttons.  
pub struct ButtonSchema {
	button_groups: HashMap<ButtonGroupName, ButtonGroup>,
	current_group: ButtonGroupName,
}

impl ButtonSchema {
	/// Creates and returns a new button schema. 
	pub fn new(button_groups: Vec<ButtonGroup>, current_group: ButtonGroupName) -> Self {
		Self {
			button_groups: button_groups.into_iter()
				.map(|button_group| (button_group.name(), button_group))
				.collect(),
			current_group, 
		}
	}

	// /// Creates and returns an empty button schema.
	// pub fn empty() -> Self {
	// 	Self {
	// 		button_groups: HashMap::new(), 
	// 		current_group: "", 
	// 	}
	// }

	/// Returns the current button group, i.e. the button group that is receiving input. 
	pub fn current_button_group(&mut self) -> &mut ButtonGroup {
		match self.button_groups.get_mut(&self.current_group){
			Some(button_group) => button_group, 
			None => panic!("`ButtonGroup` not found in `ButtonSchema`: \"{}\".", &self.current_group),
		}
	}

	// mutable addition methods are difficult to keep track of when using a base program.
	// pub fn add_button_group(&mut self, button_group: ButtonGroup) {
	// 	if self.button_groups.contains_key(button_group.name()) {
	// 		panic!("`button_groups` already has a button group named: \"{}\".", button_group.name());
	// 	}

	// 	self.button_groups.insert(button_group.name(), button_group); 
	// }
}

impl ButtonInteract for ButtonSchema {
	fn handle_input(&mut self, key: Key) -> ButtonName {
		self.current_group = self.current_button_group().handle_input(key);
		self.current_button_group().current_button()
	}
}