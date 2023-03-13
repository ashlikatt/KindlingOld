use crate::{values::{ParameterList, Value, Text, Number, Location, Vector, Sound, Potion, Variable, GameValue, Tag}, statements::Statement};

pub trait DFSerializable {
    fn serialize(&self) -> String;
}
pub trait DFSerializableStatementContext {
    fn serialize_params(&self, stmnt: &Statement) -> String;
}

impl DFSerializableStatementContext for ParameterList {
    fn serialize_params(&self, stmnt: &Statement) -> String {
        self.iter()
        .enumerate()
        .filter_map(|(slot, x)| x.as_ref().map_or_else(|| None, |x| Some((slot, x))))
        .map(|(slot, x)| x.serialize_slot(slot, stmnt))
        .collect::<Vec<String>>()
        .join(",")
    }
}

impl Value {
    fn serialize_slot(&self, slot: usize, stmnt: &Statement) -> String {
        match self {
            Value::Text(Text(t)) => format!( r#"{{"item":{{"id":"txt","data":{{"name": "{}"}}}},"slot":{}}}"#, t, slot ),
            Value::Number(Number(n)) => format!( r#"{{"item":{{"id":"num","data":{{"name": "{}"}}}},"slot":{}}}"#, n, slot ),
            Value::Location(Location{ x, y, z, pitch, yaw }) => {
                format!(
                    r#"{{"item":{{"id":"loc","data":{{"isBlock":false,"loc":{{"x":{x},"y":{y},"z":{z},"pitch":{pitch},"yaw":{yaw}}}}}}},"slot":{}}}"#, 
                    slot
                )
            }
            Value::Vector(Vector{ x, y, z }) => {
                format!(
                    r#"{{"item":{{"id":"vec","data":{{"x":{x},"y":{y},"z":{z}}}}},"slot":{}}}"#, 
                    slot
                )
            }
            Value::Sound(Sound{ sound, pitch, volume }) => {
                format!(
                    r#"{{"item":{{"id":"snd","data":{{"sound":{sound},"pitch":{pitch},"vol":{volume}}}}},"slot":{}}}"#, 
                    slot
                )
            }
            Value::Particle(_) => todo!(),
            Value::Potion(Potion{ effect, ticks, level }) => {
                format!(
                    r#"{{"item":{{"id":"pot","data":{{"pot":{},"dur":{ticks},"amp":{level}}}}},"slot":{}}}"#, 
                    effect.serialize(),
                    slot
                )
            }
            Value::Variable(Variable{ name, scope }) => {
                format!(
                    r#"{{"item":{{"id":"var","data":{{"name":{name},"scope":{}}}}},"slot":{}}}"#, 
                    scope.serialize(),
                    slot
                )
            }
            Value::GameValue(GameValue{ name, selector }) => {
                format!(
                    r#"{{"item":{{"id":"g_val","data":{{"type":{name},"target":{}}}}},"slot":{}}}"#, 
                    selector.unwrap_or_default().serialize(),
                    slot
                )
            }
            Value::Item(_) => todo!(),
            Value::Tag(Tag{ name, option, var }) => {
                let action = match stmnt {
                    Statement::PlayerEvent(n) |
                    Statement::EntityEvent(n) => n,
                    Statement::Function { .. } |
                    Statement::CallFunction { .. } |
                    Statement::CallProcess { .. } |
                    Statement::Process { .. } => "dynamic",
                    Statement::PlayerAction { action, ..  } |
                    Statement::EntityAction { action, .. } |
                    Statement::SetVariable { action, .. } |
                    Statement::GameAction { action, .. } |
                    Statement::Control { action, .. } |
                    Statement::IfVariable { action,.. } |
                    Statement::IfPlayer { action, .. } |
                    Statement::IfEntity { action, .. } |
                    Statement::IfGame { action, .. } => action,
                    Statement::Else => "else",
                    Statement::Close => "close",
                    Statement::CloseRepeat => "close_repeat",
                    Statement::Repeat { action, subaction, .. } |
                    Statement::SelectObject { action, subaction, .. } => subaction.as_ref().unwrap_or_else(|| action)
                };
                if let Some(v) = var {
                    format!(
                        r#"{{"item":{{"id":"bl_tag","data":{{"option":"{option}","tag":"{name}","action":"{}","block":"{}","variable":{{"id":"var","data":{{"name":"{}","scope":"{}"}}}}}}}},"slot":{}}}"#, 
                        action,
                        stmnt.technical_name(),
                        v.name,
                        v.scope.serialize(),
                        slot
                    )
                } else {
                    format!(
                        r#"{{"item":{{"id":"bl_tag","data":{{"option":"{option}","tag":"{name}","action":"{}","block":"{}"}}}},"slot":{}}}"#, 
                        action,
                        stmnt.technical_name(),
                        slot
                    )
                }
            }
        }
    }
}