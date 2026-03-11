use win32console::structs::input_event::KeyEventRecord;

// use win32console::structs::input_event::ControlKeyState as ControlKeyStateHolder;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ControlKeyState {
	RightAltPressed = 0x0001,
	LeftAltPressed = 0x0002,
	RightControlPressed = 0x0004,
	LeftControlPressed = 0x0008,
	ShiftPressed = 0x0010,
	NumLockOn = 0x0020,
	ScrollLockOn = 0x0040,
	CapsLockOn = 0x0080,
	EnhancedKey = 0x0100,
}

#[derive(Copy, Clone, Debug, Eq)]
pub struct Key {
	key_down: bool, 
	key_code: Option<KeyCode>,
	character: char, 
	control_key_state: u32, 
}

impl PartialEq for Key {
    fn eq(&self, other: &Self) -> bool {
        self.key_down == other.key_down &&
        self.key_code == other.key_code && 
        self.character == other.character &&
        self.is_alt_pressed() && other.is_alt_pressed() || 
		self.is_control_pressed() && other.is_control_pressed() ||
		self.is_shift_pressed() && other.is_shift_pressed() ||
		self.is_num_lock_on() && other.is_num_lock_on() ||
		self.is_caps_lock_on() && other.is_caps_lock_on() ||
		self.is_scroll_lock_on() && other.is_scroll_lock_on() ||
		self.is_enhanced_key() && other.is_enhanced_key()
	}
}

use std::hash::{DefaultHasher, Hash, Hasher};

impl Hash for Key {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.key_down.hash(state);
        self.key_code.hash(state);
        self.character.hash(state);
        self.is_alt_pressed().hash(state);
		self.is_control_pressed().hash(state);
		self.is_shift_pressed().hash(state);
		self.is_num_lock_on().hash(state);
		self.is_caps_lock_on().hash(state);
		self.is_scroll_lock_on().hash(state);
		self.is_enhanced_key().hash(state);
	}
}

impl Key {
	pub const UP: Key = Key { key_down: true, key_code: Some(KeyCode::Up), character: '\0', control_key_state: ControlKeyState::EnhancedKey as u32 };
	pub const DOWN: Key = Key { key_down: true, key_code: Some(KeyCode::Down), character: '\0', control_key_state: ControlKeyState::EnhancedKey as u32 };
	pub const LEFT: Key = Key { key_down: true, key_code: Some(KeyCode::Left), character: '\0', control_key_state: ControlKeyState::EnhancedKey as u32 };
	pub const RIGHT: Key = Key { key_down: true, key_code: Some(KeyCode::Right), character: '\0', control_key_state: ControlKeyState::EnhancedKey as u32 };
	// shift + arrow key is already used by the command prompt... :( maybe find a way to "fix" that...
	// pub const SHIFT_UP: Key = Key { key_down: true, key_code: Some(KeyCode::Up), character: '\0', control_key_state: ControlKeyState::EnhancedKey as u32 + ControlKeyState::ShiftPressed as u32};
	// pub const SHIFT_DOWN: Key = Key { key_down: true, key_code: Some(KeyCode::Down), character: '\0', control_key_state: ControlKeyState::EnhancedKey as u32 + ControlKeyState::ShiftPressed as u32};
	// pub const SHIFT_LEFT: Key = Key { key_down: true, key_code: Some(KeyCode::Left), character: '\0', control_key_state: ControlKeyState::EnhancedKey as u32 + ControlKeyState::ShiftPressed as u32};
	// pub const SHIFT_RIGHT: Key = Key { key_down: true, key_code: Some(KeyCode::Right), character: '\0', control_key_state: ControlKeyState::EnhancedKey as u32 + ControlKeyState::ShiftPressed as u32};
	pub const CTRL_UP: Key = Key { key_down: true, key_code: Some(KeyCode::Up), character: '\0', control_key_state: ControlKeyState::EnhancedKey as u32 + ControlKeyState::RightControlPressed as u32/* + ControlKeyState::LeftControlPressed as u32*/ };
	pub const CTRL_DOWN: Key = Key { key_down: true, key_code: Some(KeyCode::Down), character: '\0', control_key_state: ControlKeyState::EnhancedKey as u32 + ControlKeyState::RightControlPressed as u32/* + ControlKeyState::LeftControlPressed as u32*/ };
	pub const CTRL_LEFT: Key = Key { key_down: true, key_code: Some(KeyCode::Left), character: '\0', control_key_state: ControlKeyState::EnhancedKey as u32 + ControlKeyState::RightControlPressed as u32/* + ControlKeyState::LeftControlPressed as u32*/ };
	pub const CTRL_RIGHT: Key = Key { key_down: true, key_code: Some(KeyCode::Right), character: '\0', control_key_state: ControlKeyState::EnhancedKey as u32 + ControlKeyState::RightControlPressed as u32/* + ControlKeyState::LeftControlPressed as u32*/ };
	
	pub const ESCAPE: Key = Key { key_down: true, key_code: Some(KeyCode::Escape), character: '\u{1b}', control_key_state: 0 };

	pub fn new(key_down: bool, key_code: Option<KeyCode>, character: char, control_key_state: &[ControlKeyState]) -> Self {
		Self {
			key_down, 
			key_code, 
			character, 
			control_key_state: control_key_state.iter().map(|&state| state as u32).sum(),
		}
	}

	pub fn from(key_event_record: KeyEventRecord) -> Self {
		Self {
			key_down: key_event_record.key_down, 
			key_code: KeyCode::from(key_event_record.virtual_key_code),
			character: key_event_record.u_char, 
			control_key_state: key_event_record.control_key_state.get_state(),
		} 
	}

	pub fn is_down(&self) -> bool {
		self.key_down
	}

	pub fn key_code(&self) -> &Option<KeyCode> {
		&self.key_code
	}

	pub fn character(&self) -> char {
		self.character
	}

	pub fn has_state(&self, control_key_state: ControlKeyState) -> bool {
		(control_key_state as u32 & self.control_key_state) != 0
	}

	pub fn get_state(&self) -> u32 {
		self.control_key_state
	}

	pub fn is_alt_pressed(&self) -> bool {
		self.has_state(ControlKeyState::RightAltPressed) || self.has_state(ControlKeyState::LeftAltPressed)
	}

	pub fn is_control_pressed(&self) -> bool {
		self.has_state(ControlKeyState::RightControlPressed) || self.has_state(ControlKeyState::LeftControlPressed)
	}

	pub fn is_shift_pressed(&self) -> bool {
		self.has_state(ControlKeyState::ShiftPressed)
	}

	pub fn is_num_lock_on(&self) -> bool {
		self.has_state(ControlKeyState::NumLockOn)
	}

	pub fn is_caps_lock_on(&self) -> bool {
		self.has_state(ControlKeyState::CapsLockOn)
	}

	pub fn is_scroll_lock_on(&self) -> bool {
		self.has_state(ControlKeyState::ScrollLockOn)
	}

	pub fn is_enhanced_key(&self) -> bool {
		self.has_state(ControlKeyState::EnhancedKey)
	}
}

use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

impl KeyCode {
	pub fn from(virtual_key_code: u16) -> Option<Self> {
		FromPrimitive::from_u16(virtual_key_code)
	}
}

#[repr(u16)]
#[derive(FromPrimitive, Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub enum KeyCode {
	Cancel = 0x03,
	Backspace = 0x08,
	Tab = 0x09,
	Clear = 0x0C,
	Enter = 0x0D, 
	Shift = 0x10, 
	Control = 0x11, 
	Alt = 0x12, 
	Pause = 0x13, 
	CapsLock = 0x14, 
	Escape = 0x1B, 
	Space = 0x20, 
	PageUp = 0x21, 
	PageDown = 0x22, 
	End = 0x23, 
	Home = 0x24,
	Left = 0x25, 
	Up = 0x26, 
	Right = 0x27, 
	Down = 0x28, 
	Select = 0x29, 
	Print = 0x2A, 
	Execute = 0x2B, 
	PrintScreen = 0x2C, 
	Insert = 0x2D, 
	Delete = 0x2E, 
	Help = 0x2F, 
	RightParenthesisOr0 = 0x30, 
	ExclamationPointOr1 = 0x31, 
	AtSymbolOr2 = 0x32, 
	HashtagOr3 = 0x33, 
	DollarSignOr4 = 0x34, 
	PercentSignOr5 = 0x35, 
	CaretOr6 = 0x36, 
	AmpersandOr7 = 0x37, 
	AsteriskOr8 = 0x38, 
	LeftParenthesisOr9 = 0x39, 
	A = 0x41,
	B = 0x42,
	C = 0x43,
	D = 0x44,
	E = 0x45,
	F = 0x46,
	G = 0x47,
	H = 0x48,
	I = 0x49,
	J = 0x4A,
	K = 0x4B,
	L = 0x4C,
	M = 0x4D,
	N = 0x4E,
	O = 0x4F,
	P = 0x50,
	Q = 0x51,
	R = 0x52,
	S = 0x53,
	T = 0x54,
	U = 0x55,
	V = 0x56,
	W = 0x57,
	X = 0x58,
	Y = 0x59,
	Z = 0x5A,
	LeftWindows = 0x5B,
	RightWindows = 0x5C, 
	Applications = 0x5D, 
	Sleep = 0x5F, 
	Numpad0 = 0x60,
	Numpad1 = 0x61, 
	Numpad2 = 0x62, 
	Numpad3 = 0x63, 
	Numpad4 = 0x64, 
	Numpad5 = 0x65,   
	Numpad6 = 0x66, 
	Numpad7 = 0x67, 
	Numpad8 = 0x68, 
	Numpad9 = 0x69, 
	Multiply = 0x6A, 
	Add = 0x6B, 
	Separator = 0x6C, 
	Subtract = 0x6D, 
	Decimal = 0x6E, 
	Divide = 0x6F, 
	F1 = 0x70,
	F2 = 0x71,
	F3 = 0x72,
	F4 = 0x73,
	F5 = 0x74,
	F6 = 0x75,
	F7 = 0x76,
	F8 = 0x77,
	F9 = 0x78,
	F10 = 0x79,
	F11 = 0x7A,
	F12 = 0x7B,
	F13 = 0x7C,
	F14 = 0x7D,
	F15 = 0x7E,
	F16 = 0x7F,
	F17 = 0x80,
	F18 = 0x81,
	F19 = 0x82,
	F20 = 0x83,
	F21 = 0x84,
	F22 = 0x85,
	F23 = 0x86,
	F24 = 0x87,
	Numlock = 0x90,
	ScrollLock = 0x91, 
	LeftShift = 0xA0, 
	RightShift = 0xA1, 
	LeftControl = 0xA2,
	RightControl = 0xA3, 
	LeftAlt = 0xA4, 
	RightAlt = 0xA5, 
	BrowserBack = 0xA6, 
	BrowserForwar = 0xA7, 
	BrowserRefresh = 0xA8, 
	BrowserStop = 0xA9, 
	BrowserSearch = 0xAA, 
	BrowserFavorites = 0xAB, 
	BrowserHome = 0xAC, 
	VolumeMute = 0xAD, 
	VolumeDown = 0xAE, 
	VolumeUp = 0xAF, 
	MediaNextTrack = 0xB0,
	MediaPreviousTrack = 0xB1, 
	MediaStop = 0xB2, 
	MediaPlayPause = 0xB3,
	LaunchMail = 0xB4, 
	SelectMedia = 0xB5, 
	LaunchApp1 = 0xB6, 
	LaunchApp2 = 0xB7, 
	SemiColonOrColon = 0xBA, 
	EqualsOrPlus = 0xBB, 
	CommaOrLeftAngleBracket = 0xBC, 
	MinusOrUnderscore = 0xBD, 
	PeriodOrRightAngleBracket = 0xBE, 
	SlashOrQuestionMark = 0xBF,
	BacktickOrTilde = 0xC0, 
	LeftBracketOrBrace = 0xDB,
	BackSlashOrVerticalBar = 0xDC, 
	RightBracketOrBrace = 0xDD, 
	SingleOrDoubleQuote = 0xDE, 
	OEM102 = 0xE2, 
	Process = 0xE5, 
	Packet = 0xE7, 
	Attention = 0xF6, 
	CrSel = 0xF7, 
	ExSel = 0xF8, 
	EraseEOF = 0xF9, 
	Play = 0xFA, 
	Zoom = 0xFB, 
	OEMClear = 0xFE, 
}