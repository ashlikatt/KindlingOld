use crate::{code_line::CodeLine, statements::Statement, values::{Selector, ParameterList, Text, Value}};
#[cfg(test)]
use crate::program::Program;




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