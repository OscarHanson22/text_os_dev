use std::collections::HashMap;

use crate::{Position, AsFrame, Interact, Program, ControlFlow, ProgramId, DependencyStatus, Key};
use crate::frame::Frame;

pub struct Dimensions {
	pub width: usize, 
	pub height: usize, 
}

impl Dimensions {
	pub fn new(width: usize, height: usize) -> Self {
		Self {
			width, 
			height, 
		}
	}
}

/// A "System Program" that only opens when attempting to navigate between programs using ctrl + [ or ], much like alt tab.
/// Is opened by the Interface only when ctrl + [ or ] is pressed and closes when ctrl is released. 
/// Should open, display program names (not itself) and icons (add in future for fun lol)
/// or this could be a taskbar at the bottom, but i like the idea of the navigator program more. 
/// or use alt + [ or ], where alt opens the navigator, 
pub struct ProgramNavigator {

/*

┌──────────────────────────┐
│                          │
│        Text Editor       │
│  ┌────┐  ┌────┐  ┌────┐  │
│  │ Fs │  │ Te │  │pSto│  │
│  └────┘  └────┘  └────┘  │
│                          │
└──────────────────────────┘
Ctrl + [ pressed
┌──────────────────────────┐
│                          │
│        File System       │
│          ┌────┐  ┌────┐  │
│          │ Fs │  │ Te │  │
│          └────┘  └────┘  │
│                          │
└──────────────────────────┘

*/



pub struct Interface {
	programs: HashMap<usize, Box<dyn Program>>, 
	program_: HashMap<usize, (Dimensions, Position)>,
	program_tree: ProgramTree, 
	root: usize,
	recency: Recency, 
	id_manager: IDManager,
}

impl Interface {
	pub fn new(mut entry_point: Box<dyn Program>) -> Self {
		entry_point.initialize();

		let mut id_manager = IDManager::new();
		let entry_point_id = id_manager.generate();

		let mut programs = HashMap::new();
		programs.insert(entry_point_id, entry_point);

		Self {
			programs, 
			program_tree: ProgramTree::new(entry_point_id),
			root: entry_point_id,
			recency: Recency::new(entry_point_id),
			id_manager,
		}
	}

	/// Visits the program at `program_id`. 
	/// 
	/// The selected program will intercept input, have focus, and become the current program. 
	pub fn visit(&mut self, program_id: usize) {
		self.recency.visit(program_id);
	}

	/// Returns the current program. 
	pub fn current_program(&mut self) -> Option<&mut Box<dyn Program>> {
		let focus = self.recency.focus();
		self.get_program_mut(focus)
	}

	/// Adds the `program` to the interface and returns its id. 
	///
	/// Program id is handled automatically by the `id_manager`.
	/// 
	/// Program tree is handled in the Interact trait implementation, where the hierarchy is clear. 
	pub fn add_program(&mut self, program: Box<dyn Program>) -> usize {
		let id = self.id_manager.generate();
		self.programs.insert(id, program);
		id
	}

	/// Returns a mutable borrow to the program with `program_id`.
	pub fn get_program_mut(&mut self, program_id: usize) -> Option<&mut Box<dyn Program>> {
		self.programs.get_mut(&program_id)
	}

	/// Returns an immutable borrow to the program with `program_id`.
	pub fn get_program(&self, program_id: usize) -> Option<&Box<dyn Program>> {
		self.programs.get(&program_id)
	}

	/// Removes the program with `program_id`, removes its child programs, and frees all of the ids. 
	pub fn remove_program(&mut self, program_id: usize) {
		let _ = self.programs.remove(&program_id);
		self.id_manager.free(program_id);
		self.recency.remove(program_id);

		// remove all child programs
		for child_id in self.program_tree.remove_node(program_id) {
			self.remove_program(child_id);
		}
	}
}

impl Interact for Interface {
	fn handle_input(&mut self, key: Key) -> Vec<ControlFlow> {
		if self.programs.len() == 0 {
			return vec![ControlFlow::Close(ProgramId::This)];
		}

		// if key.is_down() {
		// 	match key.key_code {
		// 		Some(KeyCode::Alt) => self.program_navigator.open


		let current_program_id = self.recency.focus();
		let control_flows = self.current_program().expect("Should never fail").handle_input(key);

		for control_flow in control_flows {
			if self.programs.len() == 0 {
				return vec![ControlFlow::Close(ProgramId::This)];
			}

			match control_flow {
				ControlFlow::Close(program_id) => {
					match program_id {
						ProgramId::This => self.remove_program(current_program_id),

						ProgramId::Parent => {
							if let Some(parent_id) = self.program_tree.parent_of(current_program_id) {
								self.remove_program(parent_id);
							}
						}

						_ => (), // handle removing a child later if needed
					}
				}

				ControlFlow::Open(mut program, dependency_status) => {
					program.initialize();
					let new_program_id = self.add_program(program);
					self.recency.add(new_program_id);
					self.recency.visit(new_program_id);

					match dependency_status {
						DependencyStatus::Independent => self.program_tree.add_node(new_program_id),
						DependencyStatus::Dependent => self.program_tree.add_child(new_program_id, current_program_id),
					}
				}

				_ => (), 
			};
		}

		vec![ControlFlow::Continue]
	}
}

impl AsFrame for Interface {
	fn as_frame(&self) -> Frame {
		let (program_widths, program_heights): (Vec<usize>, Vec<usize>) = self.programs.iter()
			.map(|(id, program)| (program.width(), program.height()))
			.unzip();

		let max_width = program_widths.into_iter().max().unwrap_or(0);
		let max_height = program_heights.into_iter().max().unwrap_or(0);

		let mut frame = Frame::new(max_width, max_height);

		for &program_id in &self.recency { // order is good, but the `background program` will cover up quite a few programs. 
			if let Some(program) = self.get_program(program_id) {
				let _ = frame.add_frame(&program.as_frame(), program.position());
			}
		}

		frame
	}
}

#[derive(Debug)]
pub struct ProgramTreeNode {
	parent: Option<usize>, 
	children: Vec<usize>, 
}

impl ProgramTreeNode {
	/// Returns a new program tree node with the specified `parent` and `children`.
	pub fn new(parent: Option<usize>, children: Vec<usize>) -> Self {
		Self {
			parent, 
			children, 
		}
	}

	/// Returns a reference to the parent of the program tree node. 
	pub fn parent(&self) -> &Option<usize> {
		&self.parent
	}

	/// Returns a reference to the children of the program tree node. 
	pub fn children(&self) -> &[usize] {
		&self.children
	}

	/// Adds a `child` to the program tree node.
	pub fn add_child(&mut self, child: usize) {
		self.children.push(child);
	}

	/// Removes a `child` from the program tree node.
	pub fn remove_child(&mut self, child: usize) {
		if let Some(child_index) = self.children.iter().position(|&c| c == child) {
			self.children.remove(child_index);
		}
	}
}

#[derive(Debug)]
pub struct ProgramTree { // or more aptly--program forest...
	tree: HashMap<usize, ProgramTreeNode>,
}

impl ProgramTree {
	/// Creates and returns a program tree with the specified `root`. 
	pub fn new(root: usize) -> Self {
		let mut tree = HashMap::new();
		tree.insert(root, ProgramTreeNode::new(None, Vec::new()));

		Self {
			tree,
		}
	}

	/// Adds an independent `node` to the program tree. 
	pub fn add_node(&mut self, node: usize) {
		if self.tree.contains_key(&node) {
			return;
		}

		self.tree.insert(node, ProgramTreeNode::new(None, Vec::new()));
	}

	/// Returns the parent `of_node` if it can be found. 
	pub fn parent_of(&self, of_node: usize) -> Option<usize> {
		if let Some(node) = self.tree.get(&of_node) {
			*node.parent()
		} else {
			None
		}
	}

	/// Adds a `child` `to_node`. 
	pub fn add_child(&mut self, child: usize, to_node: usize) {
		// prevents a node from being added twice. 
		if self.tree.contains_key(&child) {
			return;
		}

		self.tree.entry(to_node).and_modify(|node| node.add_child(child));
		self.tree.insert(child, ProgramTreeNode::new(Some(to_node), Vec::new()));
	}

	/// Removes `node` and all its children from the program tree, returning all of the removed children (not including the original node, just children). 
	pub fn remove_node(&mut self, node: usize) -> Vec<usize> {
		let mut removed_children = Vec::new();

		if let Some(removed_node) = self.tree.remove(&node) {
			// remove the node's parent's child (which is the node)
			if let Some(parent) = removed_node.parent() {
				self.tree.entry(*parent).and_modify(|parent| parent.remove_child(node));
			}

			// remove the children of node from the tree recursively
			for &child_of_node in removed_node.children() {
				removed_children.push(child_of_node);
				removed_children.extend_from_slice(&self.remove_node(child_of_node));
			}
		}

		removed_children
	}
}

#[derive(Debug)]
pub struct IDManager {
	ids: Vec<bool>, 
}

impl IDManager {
	pub fn new() -> Self {
		Self {
			ids: Vec::new(), 
		}
	}

	/// Finds the next available id and returns it. 
	pub fn generate(&mut self) -> usize {
		if let Some(id) = self.ids.iter().position(|&id_taken| !id_taken) {
			self.ids[id] = true;
			id
		} else {
			self.ids.push(true);
			self.ids.len() - 1
		}
	}

	/// Frees the specified id, if they id cannot be found, it currently does nothing (may panic in future versions)
	/// 
	/// If the value cannot be found, it was never generated (and not returned previously).
	/// 
	/// Automatically manages size by popping off the highest id generated when it is freed. 
	pub fn free(&mut self, id: usize) {
		let last_index = self.ids.len() - 1;

		if id == last_index {
			self.ids.pop();
		} else if id < last_index {
			self.ids[id] = false;
		}
	}
}

#[derive(Debug)]
pub struct Recency {
	data: Vec<usize>,
}

impl Recency {
	pub fn new(entry: usize) -> Self {
		Self {
			data: vec![entry], 
		}
	}

	pub fn focus(&self) -> usize {
		self.data[self.data.len() - 1]
	}

	pub fn visit(&mut self, element: usize) {
		if element == self.focus() {
			return;
		}

		if let Some(index) = self.data.iter().position(|&e| e == element) {
			let _ = self.data.remove(index);
			self.data.push(index);
		}
	}

	pub fn add(&mut self, element: usize) {
		self.data.push(element);
	}

	pub fn remove(&mut self, element: usize) {
		if self.data.len() == 1 {
			return;
		}

		if let Some(index) = self.data.iter().position(|&e| e == element) {
			let _ = self.data.remove(index);
		}
	}
}

impl<'a> IntoIterator for &'a Recency {
    type Item = <&'a Vec<usize> as IntoIterator>::Item;
    type IntoIter = <&'a Vec<usize> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        (&self.data).into_iter()
    }
}
