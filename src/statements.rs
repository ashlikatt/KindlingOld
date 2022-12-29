use crate::{serialization::DFSerializable, values::{Selector, ParameterList}};

pub enum Statement {
    PlayerEvent(PlayerEventType),
    EntityEvent(EntityEventType),
    Function{ name: String, parameters: ParameterList },
    Process{ name: String, parameters: ParameterList },
    PlayerAction { action: String, parameters: ParameterList, selector: Selector },
    EntityAction { action: String, parameters: ParameterList, selector: Selector },
    SetVariable { action: String, parameters: ParameterList },
    GameAction { action: String, parameters: ParameterList },
    Control { action: String, parameters: ParameterList },
    SelectObject { action: String, subaction: Option<String>, parameters: ParameterList, not: bool },
    IfVariable { action: String, parameters: ParameterList, not: bool },
    IfPlayer { action: String, parameters: ParameterList, selector: Selector, not: bool },
    IfEntity { action: String, parameters: ParameterList, selector: Selector, not: bool },
    IfGame { action: String, parameters: ParameterList, not: bool },
    Else, Close, CloseRepeat,
    Repeat { action: String, subaction: Option<String>, parameters: ParameterList, not: bool },
    CallFunction { name: String, parameters: ParameterList },
    CallProcess { name: String, parameters: ParameterList }
}

impl DFSerializable for Statement {
    fn serialize(&self) -> String {
        match self {
            Statement::PlayerAction { action, parameters, selector } |
            Statement::EntityAction { action, parameters, selector } => {
                format!(
                    r#"{{"id":"block","block":"{}","args":{{"items":[{}]}},"action":"{}","target":"{}"}}"#,
                    self.technical_name(),
                    parameters.serialize(),
                    action,
                    selector.serialize()
                )
            }
            Statement::SetVariable { action, parameters } |
            Statement::GameAction { action, parameters } |
            Statement::Control { action, parameters } => {
                format!(
                    r#"{{"id":"block","block":"{}","args":{{"items":[{}]}},"action":"{}"}}"#,
                    self.technical_name(),
                    parameters.serialize(),
                    action
                )
            }
            Statement::CallFunction { name, parameters } |
            Statement::CallProcess { name, parameters } => {
                format!(
                    r#"{{"id":"block","block":"{}","args":{{"items":[{}]}},"data":"{}"}}"#,
                    self.technical_name(),
                    parameters.serialize(),
                    name
                )
            }
            Statement::SelectObject { action, subaction, parameters, not } => {
                format!(
                    r#"{{"id":"block","block":"select_obj","args":{{"items":[{}]}},"action":"{}","subAction":"{}","inverted":"{}"}}"#,
                    parameters.serialize(),
                    action,
                    subaction.as_ref().map_or_else(|| "", |a| &a),
                    if *not { "NOT" } else { "" }
                )
            }
            Statement::IfVariable { action, parameters, not } |
            Statement::IfGame { action, parameters, not } => {
                format!(
                    r#"{{"id":"block","block":"{}","args":{{"items":[{}]}},"action":"{}","inverted":"{}"}},{{"id":"bracket","direct":"open","type":"norm"}}"#,
                    self.technical_name(),
                    parameters.serialize(),
                    action,
                    if *not { "NOT" } else { "" }
                )
            }
            Statement::IfPlayer { action, parameters, selector, not } |
            Statement::IfEntity { action, parameters, selector, not } => {
                format!(
                    r#"{{"id":"block","block":"{}","args":{{"items":[{}]}},"action":"{}","inverted":"{}","target":"{}"}},{{"id":"bracket","direct":"open","type":"norm"}}"#,
                    self.technical_name(),
                    parameters.serialize(),
                    action,
                    if *not { "NOT" } else { "" },
                    selector.serialize()
                )
            }
            Statement::Else => String::from(r#"{"id":"block","block":"else"},{"id":"bracket","direct":"open","type":"norm"}"#),
            Statement::Close => String::from(r#"{"id":"bracket","direct":"close","type":"norm"}"#),
            Statement::CloseRepeat => String::from(r#"{"id":"bracket","direct":"close","type":"repeat"}"#),
            Statement::Repeat { action, subaction, parameters, not } => {
                format!(
                    r#"{{"id":"block","block":"repeat","args":{{"items":[{}]}},"action":"{}",subAction:"{}","inverted":"{}"}},{{"id":"bracket","direct":"open","type":"repeat"}}"#,
                    parameters.serialize(),
                    action,
                    subaction.as_ref().map_or_else(|| "", |a| &a),
                    if *not { "NOT" } else { "" }
                )
            }
            Statement::PlayerEvent(e) |
            Statement::EntityEvent(e) => { 
                format!(
                    r#"{{"id":"block","block":"{}","args":{{"items":[]}},"action":"{}"}}"#,
                    self.technical_name(),
                    e
                )
            }
            Statement::Function {name, parameters } |
            Statement::Process {name, parameters } => {
                format!(
                    r#"{{"id":"block","block":"{}","args":{{"items":[{}]}},"data":"{name}"}}"#,
                    self.technical_name(),
                    parameters.serialize()
                )
            }
        }
    }
}
impl Statement {
    fn technical_name(&self) -> &str {
        match self {
            Statement::PlayerAction { action, parameters, selector } => "player_action",
            Statement::EntityAction { action, parameters, selector } => "entity_action",
            Statement::SetVariable { action, parameters } => "set_var",
            Statement::GameAction { action, parameters } => "game_action",
            Statement::Control { action, parameters } => "control",
            Statement::SelectObject { action, subaction, parameters, not } => "select_obj",
            Statement::IfVariable { action, parameters, not } => "if_var",
            Statement::IfPlayer { action, parameters, selector, not } => "if_player",
            Statement::IfEntity { action, parameters, selector, not } => "if_entity",
            Statement::IfGame { action, parameters, not } => "if_game",
            Statement::Else => "else",
            Statement::Close | Statement::CloseRepeat => "bracket",
            Statement::Repeat { action, subaction, parameters, not } => "repeat",
            Statement::CallFunction { name, parameters } => "call_func",
            Statement::CallProcess { name, parameters } => "start_process",
            Statement::PlayerEvent(_) => "event",
            Statement::EntityEvent(_) => "entity_event",
            Statement::Function { name, parameters } => "func",
            Statement::Process { name, parameters } => "process",
        }
    }
    pub fn name(&self) -> String{
        match self {
            Statement::PlayerAction { action, parameters, selector } => format!("Player Action: {}", action),
            Statement::EntityAction { action, parameters, selector } => format!("Entity Action: {}", action),
            Statement::SetVariable { action, parameters } => format!("Set Variable Action: {}", action),
            Statement::GameAction { action, parameters } => format!("Game Action: {}", action),
            Statement::Control { action, parameters } => format!("Control: {}", action),
            Statement::SelectObject { action, subaction, parameters, not } => format!("Select Object: {}", action),
            Statement::IfVariable { action, parameters, not } => format!("If Variable: {}", action),
            Statement::IfPlayer { action, parameters, selector, not } => format!("If Player: {}", action),
            Statement::IfEntity { action, parameters, selector, not } => format!("If Entity: {}", action),
            Statement::IfGame { action, parameters, not } => format!("If Game: {}", action),
            Statement::Else => String::from("Else"),
            Statement::Close | Statement::CloseRepeat => String::from("Close Bracket"),
            Statement::Repeat { action, subaction, parameters, not } => format!("Repeat: {}", action),
            Statement::CallFunction { name, parameters } => format!("Call: {}", name),
            Statement::CallProcess { name, parameters } => format!("Start Action: {}", name),
            Statement::PlayerEvent(n) => format!("Player Event: {}", n),
            Statement::EntityEvent(n) => format!("Entity Event: {}", n),
            Statement::Function { name, parameters } => format!("Function: {}", name),
            Statement::Process { name, parameters } => format!("Process: {}", name),
        }
    }
}


pub type PlayerEventType = String;
pub type EntityEventType = String;
impl DFSerializable for bool {
    fn serialize(&self) -> String {
        String::from(if *self {
            "True"
        } else {
            "False"
        })
    }
}