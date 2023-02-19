#[cfg(test)]

use crate::{code_line::CodeLine, statements::Statement, values::{Selector, ParameterList, Text, Value}};

use crate::{program::Program, values::{Tag, Location}, params::ParamBuilder};




#[test]
fn test_empty() {
    let p = Program::new_from(vec![]);
    assert_eq!(p.compile_program(), Vec::<String>::new());
}

#[test]
fn test_simple() {
    let p = Program::new_from(vec![
        CodeLine::new_from(vec![ Statement::PlayerEvent(String::from("Join")) ])
    ]);
    for l in p.compile_program() {
        println!("{}", l);
    }
}

#[test]
fn test_simple2() {
    let p = Program::new_from(vec![
        CodeLine::new_from(vec![ 
            Statement::PlayerEvent(String::from("Join")),
            Statement::PlayerAction { action: String::from("SendMessage"), parameters: [Some(Value::Text(Text(String::from("§a%default joined!")))), None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None], selector: Selector::AllPlayers },
        ])
    ]);
    for l in p.compile_program() {
        println!("{}", l);
    }
}

#[test]
fn test_recode() {
    let p = Program::new_from(vec![
        CodeLine::new_from(vec![ 
            Statement::PlayerEvent(String::from("Join")),
            Statement::PlayerAction { action: String::from("SendMessage"), parameters: [Some(Value::Text(Text(String::from("§a%default joined!")))), None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None], selector: Selector::AllPlayers },
        ])
    ]);
    p.compile_program_ws();
}

#[test]
fn test_recode2() {
    let p = Program::new_from(vec![
        CodeLine::new_from(vec![ 
            Statement::PlayerEvent(String::from("Join")),
            Statement::IfPlayer { action: String::from("HasPermission"), parameters: ParamBuilder::new().tag(Tag{name: String::from("Permission"), option: String::from("Developer")}).complete_unchecked(), selector: Selector::Default, not: false },
            Statement::PlayerAction { action: String::from("SendMessage"), parameters: ParamBuilder::new().param(Value::Text(Text(String::from("§e[DEV] §a%default joined!")))).complete_unchecked(), selector: Selector::AllPlayers },
            Statement::Close,
            Statement::Else,
            Statement::PlayerAction { action: String::from("SendMessage"), parameters: ParamBuilder::new().param(Value::Text(Text(String::from("§a%default joined!")))).complete_unchecked(), selector: Selector::AllPlayers },
            Statement::Close
        ])
    ]);
    p.compile_program_ws(25);
}


#[test]
fn test_recode3() {
    let p = Program::new_from(vec![
        CodeLine::new_from(vec![ 
            Statement::PlayerEvent(String::from("Join")),
            Statement::IfPlayer { action: String::from("HasPermission"), parameters: ParamBuilder::new().tag(Tag{name: String::from("Permission"), option: String::from("Developer")}).complete_unchecked(), selector: Selector::Default, not: false },
            Statement::PlayerAction { action: String::from("SendMessage"), parameters: ParamBuilder::new().param(Value::Text(Text(String::from("§e[DEV] §a%default joined!")))).complete_unchecked(), selector: Selector::AllPlayers },
            Statement::Close,
            Statement::Else,
            Statement::PlayerAction { action: String::from("SendMessage"), parameters: ParamBuilder::new().param(Value::Text(Text(String::from("§a%default joined!")))).complete_unchecked(), selector: Selector::AllPlayers },
            Statement::Close
        ]), 
        CodeLine::new_from(vec![ 
            Statement::PlayerEvent(String::from("RightClick")),
            Statement::IfPlayer { action: String::from("IsLookingAt"), parameters: ParamBuilder::new().tag(Tag{name: String::from("Fluid Mode"), option: String::from("Ignore fluids")}).param(Value::Location(Location { x:25., y:49., z:27., pitch:0., yaw:0. })).complete_unchecked(), selector: Selector::Default, not: false },
            Statement::PlayerAction { action: String::from("SendMessage"), parameters: ParamBuilder::new().param(Value::Text(Text(String::from("§6Let's go!")))).complete_unchecked(), selector: Selector::Default },
            Statement::Close,
        ]), 
    ]);
    p.compile_program_ws(25);
}