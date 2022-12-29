use crate::values::{ParameterList, Value, Text, Number, Location, Vector, Sound, Potion, Variable, GameValue};

pub trait DFSerializable {
    fn serialize(&self) -> String;
}

impl DFSerializable for ParameterList {
    fn serialize(&self) -> String {
        self.iter()
        .enumerate()
        .filter_map(|(slot, x)| x.as_ref().map_or_else(|| None, |x| Some((slot, x))))
        .map(|(slot, x)| x.serialize_slot(slot))
        .collect::<Vec<String>>()
        .join(",")
    }
}

impl Value {
    fn serialize_slot(&self, slot: usize) -> String {
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
        }
    }
}