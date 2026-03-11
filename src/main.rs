// use text_os::{BasicProgram, Cursor, Frame, Position, AsFrame, FrameBuilder, InputManager, Interact, ControlFlow};
use text_os::text_box::{TextBox};
use text_os::Position;
use text_os::frame::AsFrame;
use text_os::text_box::wrap_text;
use text_os::key::{Key, KeyCode};

use std::hash::{Hash, DefaultHasher, Hasher};

fn main() {
    use text_os::{Alert, ControlFlow, DisplayCursor};

    // use text_os::drop_down::DropDown;

    // let mut dd = DropDown::new(
    //     vec![
    //         ("Option 1", 1), 
    //         ("Option 2", 2), 
    //         ("Option 3", 3), 
    //     ], 
    //     15, 
    //     4, 
    //     Position::new(0, 0), 
    // );

    // dd.open = true;

    // println!("{}", dd.as_frame());

    // loop {}

    // let up_right = Key::new(true, Some(KeyCode::Up), '\0', &[text_os::key::ControlKeyState::RightControlPressed]);
    // let up_left = Key::new(true, Some(KeyCode::Up), '\0', &[text_os::key::ControlKeyState::LeftControlPressed]);

    // println!("{}", calculate_hash(&up_right) == calculate_hash(&up_left));

    // fn calculate_hash<T: Hash>(t: &T) -> u64 {
    //     let mut s = DefaultHasher::new();
    //     t.hash(&mut s);
    //     s.finish()
    // }

    // // println!("{}", up_right == up_left);
    // loop {}

    // let mut r = Recency::new(0);

    // println!("{r:?}");
    // r.add(1);
    // r.add(2);
    // r.add(3);
    // println!("{r:?}");
    // r.visit(1);
    // println!("{r:?}");
    // println!("focus: {:?}", r.focus());
    // r.remove(3);
    // println!("{r:?}");
    // println!("focus: {:?}", r.focus());
    // r.remove(1);
    // println!("{r:?}");
    // println!("focus: {:?}", r.focus());
    // r.visit(1);
    // println!("{r:?}");
    // println!("focus: {:?}", r.focus());
    // r.visit(0);
    // println!("{r:?}");
    // println!("focus: {:?}", r.focus());


    // let mut p = ProgramTree::new(0);
    // println!("{p:#?}");
    // p.add_child(1, 0);
    // println!("{p:#?}");
    // p.add_child(2, 1);
    // println!("{p:#?}");
    // p.add_child(3, 2);
    // println!("{p:#?}");


    // println!("{:#?}", p.remove_node(1));
    // println!("{p:#?}");

    // println!("{:#?}", p.remove_node(10));
    // println!("{p:#?}");

    // println!();



    // let mut a = Alert::new(0, "Help!!!!", vec![ControlFlow::Close], 21, 8, Position::new(0, 0));

    // println!("{}", a.as_frame());
    // a.display_cursor();

    // let mut id_manager = IDManager::new();

    // for i in 0..10 {
    //     let id = id_manager.generate();

    //     // if i % 3 == 0 {
    //     //     id_manager.free(id);
    //     //     println!("freed: {id}");
    //     // } else {

    //     //     println!("{}", id);
    //     // }
    // }

    // id_manager.free(0);
    // println!("{:?}", id_manager);
    // id_manager.free(9);
    // println!("{:?}", id_manager);
    // println!("{}", id_manager.generate());
    // println!("{}", id_manager.generate());
    // println!("{:?}", id_manager);
    // println!("{:?}", id_manager);


    // println!("{:?}", a.);

    // for ss in 0..35 {
    //     let sst = side_scroll_text("Something is kinda awesome here", ss);
    //     println!("{sst}");
    // }

    // println!("{}", wrap_text("Somet\nhing feels a little off right now...", 10));

    // let mut tb = TextBox::new("Some\nthing is kinda awesome here, right?.", 10, 5, Position::new(0, 0));
    // println!("{:?}", tb.positioned_text());

    // tb.set_scroll(2);
    // tb.set_side_scroll(2);
    // println!("{}", (&tb).as_frame());

    // tb.change_side_scroll(2);
    // tb.position_text();

    // println!("{}", (&tb).as_frame());

    // loop {}

    // let mut frame = Frame::new(3, 3);
    // println!("{:?}", frame);
    // println!("{:?}", '┼'.as_frame());
    // frame.read(Position::new(1, 1));
    // frame.add_frame(&'┼', Position::new(1, 1));
    // println!("{:?}", frame);

    // let mut frame = FrameBuilder::with_dimensions(3, 3).add_border();
    // let mut frame = frame.to_frame();
    // println!("{:?}", frame);
    // println!("{:?}", '┼'.as_frame());
    // frame.read(Position::new(1, 1));
    // frame.add_frame(&'┼', Position::new(1, 1));
    // println!("{:?}", frame);

    // let mut v = vec!['┼', '┼', '┼'];


    // println!("{}", v.len());

    // TEN, TOP RIGHT!!!!!!!

    use text_os::interface::Interface;
    use text_os::{InputManager, BasicProgram, DependencyStatus, TextEditor};

    let bp = TextEditor::new("basic text editor test", 42, 20, Position::new(0, 0));
    // // let a = Alert::new(
    // //     "What the hell...", 
    // //     vec![ControlFlow::Open(Box::new(
    // //         Alert::new(
    // //             "How the hell...", 
    // //             vec![ControlFlow::Continue],
    // //             19, 
    // //             7,
    // //             Position::new(1, 1),
    // //         )
    // //     ), DependencyStatus::Dependent)], 
    // //     22, 
    // //     9, 
    // //     Position::new(0, 0),
    // // );

    let mut interface = Interface::new(Box::new(bp));

    InputManager::start(interface);

    // interface.add_program(Box::new(bp), input_manager.cursor());

    // input_manager.start(interface);

    println!("DONE");



    // use win32console::console::WinConsole;
    use win32console::input::*;

    // println!("What now?");

    use win32console::console::{WinConsole, ConsoleTextAttribute};
    use win32console::structs::console_color::ConsoleColor;
    // let len = 100;
    // let current_pos = WinConsole::output().get_cursor_position().unwrap();
    // WinConsole::output().fill_with_char(current_pos, len, ' ').unwrap();

    // for i in 0..len{
    //     let mut pos = current_pos.clone();
    //     pos.x += i as i16;
    //     let color = ConsoleTextAttribute::COMMON_LVB_REVERSE_VIDEO; // Apply colors to the characters
    //     WinConsole::output().set_text_attribute(ConsoleTextAttribute::COMMON_LVB_REVERSE_VIDEO);
    //     WinConsole::output().set_text_attribute(11);    

    //     WinConsole::output().fill_with_char(pos, 1, 'x');
    // }

    // let old_attributes = WinConsole::output().get_text_attribute().unwrap();
    // println!("start {:#?} end", old_attributes);

    // use text_os::TextEditor;

    use win32console::structs::coord::Coord;

    // let mut cursor = Cursor::new(2, 1, Position::new(0, 0));

    // let cursor_frame = cursor.as_frame();

    let WHITE_BACKGROUND: u16 = 112 + ConsoleTextAttribute::BACKGROUND_INTENSITY;

    // loop {
    //     WinConsole::output().clear();
    //     println!("HELLO");
    //     print!("{}", cursor);
    //     // cursor.position.x += 1;
    //     std::io::stdin().read_line(&mut String::new()).unwrap();
    //     // println!("{}", cursor_frame);
    // }
    // println!("{} what", ConsoleTextAttribute::BACKGROUND_GREEN + ConsoleTextAttribute::BACKGROUND_BLUE + ConsoleTextAttribute::BACKGROUND_RED);
    // WinConsole::output().write_utf8(b"RGB");

    // let attributes : [u16; 3] = [7, ConsoleColor::White.as_background_color(), 112];
    // WinConsole::output().write_output_attribute(&attributes, Coord::ZERO);



    // println!("{}", align_text(&wrap_text("SOMETHING IS wrong\n here, I guess, I just want to find the           answer.", 13), Alignment::Center, 13));

    // InputManager::start(TextEditor::new(8, 6));

}

