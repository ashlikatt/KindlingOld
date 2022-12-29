use crate::{serialization::{DFSerializable, DFSerializableStatementContext}, values::{Selector, ParameterList}};

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
                    parameters.serialize_params(&self),
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
                    parameters.serialize_params(&self),
                    action
                )
            }
            Statement::CallFunction { name, parameters } |
            Statement::CallProcess { name, parameters } => {
                format!(
                    r#"{{"id":"block","block":"{}","args":{{"items":[{}]}},"data":"{}"}}"#,
                    self.technical_name(),
                    parameters.serialize_params(&self),
                    name
                )
            }
            Statement::SelectObject { action, subaction, parameters, not } => {
                format!(
                    r#"{{"id":"block","block":"select_obj","args":{{"items":[{}]}},"action":"{}","subAction":"{}","inverted":"{}"}}"#,
                    parameters.serialize_params(&self),
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
                    parameters.serialize_params(&self),
                    action,
                    if *not { "NOT" } else { "" }
                )
            }
            Statement::IfPlayer { action, parameters, selector, not } |
            Statement::IfEntity { action, parameters, selector, not } => {
                format!(
                    r#"{{"id":"block","block":"{}","args":{{"items":[{}]}},"action":"{}","inverted":"{}","target":"{}"}},{{"id":"bracket","direct":"open","type":"norm"}}"#,
                    self.technical_name(),
                    parameters.serialize_params(&self),
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
                    parameters.serialize_params(&self),
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
                    parameters.serialize_params(&self),
                )
            }
        }
    }
}
impl Statement {
    pub fn technical_name(&self) -> &str {
        match self {
            Statement::PlayerAction { .. } => "player_action",
            Statement::EntityAction { .. } => "entity_action",
            Statement::SetVariable { .. } => "set_var",
            Statement::GameAction { .. } => "game_action",
            Statement::Control { .. } => "control",
            Statement::SelectObject {.. } => "select_obj",
            Statement::IfVariable {.. } => "if_var",
            Statement::IfPlayer { .. } => "if_player",
            Statement::IfEntity { .. } => "if_entity",
            Statement::IfGame { .. } => "if_game",
            Statement::Else => "else",
            Statement::Close | Statement::CloseRepeat => "bracket",
            Statement::Repeat { .. } => "repeat",
            Statement::CallFunction { .. } => "call_func",
            Statement::CallProcess { .. } => "start_process",
            Statement::PlayerEvent(_) => "event",
            Statement::EntityEvent(_) => "entity_event",
            Statement::Function { .. } => "func",
            Statement::Process { .. } => "process",
        }
    }
    pub fn name(&self) -> String{
        match self {
            Statement::PlayerAction { action,.. } => format!("Player Action: {}", action),
            Statement::EntityAction { action, .. } => format!("Entity Action: {}", action),
            Statement::SetVariable { action, .. } => format!("Set Variable Action: {}", action),
            Statement::GameAction { action, .. } => format!("Game Action: {}", action),
            Statement::Control { action, .. } => format!("Control: {}", action),
            Statement::SelectObject { action, .. } => format!("Select Object: {}", action),
            Statement::IfVariable { action, .. } => format!("If Variable: {}", action),
            Statement::IfPlayer { action, .. } => format!("If Player: {}", action),
            Statement::IfEntity { action, .. } => format!("If Entity: {}", action),
            Statement::IfGame { action, .. } => format!("If Game: {}", action),
            Statement::Else => String::from("Else"),
            Statement::Close | Statement::CloseRepeat => String::from("Close Bracket"),
            Statement::Repeat { action, .. } => format!("Repeat: {}", action),
            Statement::CallFunction { name,.. } => format!("Call: {}", name),
            Statement::CallProcess { name, .. } => format!("Start Action: {}", name),
            Statement::PlayerEvent(n) => format!("Player Event: {}", n),
            Statement::EntityEvent(n) => format!("Entity Event: {}", n),
            Statement::Function { name, .. } => format!("Function: {}", name),
            Statement::Process { name, .. } => format!("Process: {}", name),
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