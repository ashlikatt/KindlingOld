use std::fmt::Display;

use websocket;
use websocket::ClientBuilder;
use websocket::ws::Sender;

use crate::{code_line::CodeLine, compile::DFCompile};

pub struct Program {
    lines: Vec<CodeLine>,
    owner: Option<String>
}
impl Program {
    pub fn compile_program(&self) -> Vec<String> {
        self.lines.iter().map(|x| {
            format!(
                r##"/give @p ender_chest{{display:{{Name:'{{"extra":[{{"italic":false,"color":"#FF8855","text":"Compiled "}},{{"italic":false,"color":"dark_gray","text":"» "}},{{"italic":false,"color":"#FFCC99","text":"{1}"}}],"text":""}}'}},PublicBukkitValues:{{"hypercube:codetemplatedata":'{{"author":"{0}","name":"&x&f&f&8&8&5&5Compiled &8» &x&f&f&c&c&9&9{1}","version":1,"code":"{2}"}}'}}}}"##, 
                self.owner.as_ref().map_or_else(|| "Kindling", |n| n),
                x.name(),
                x.compile()
            )
        } ).collect()
    }
    pub fn compile_program_ws(&self) {
        let mut client = ClientBuilder::new("ws://localhost:31371/codeutilities/item").unwrap()
            .connect_insecure()
            .unwrap();
        for l in self.lines.iter() {
            let message = format!(
                r#"{{"type":"template","data":"{{\"data\":\"{2}\",\"author\":\"{0}\",\"name\":\"§x§f§f§8§8§5§5Compiled §8» §x§f§f§c§c§9§9{1}\"}}","source":"Kindling"}}"#, 
                self.owner.as_ref().map_or_else(|| "Kindling", |n| n),
                l.name(),
                l.compile()
            );
            client.send_message(&websocket::Message::text(message)).expect("Something went wrong while sending message to recode");
            let _ = client.recv_message().unwrap();
        }
    }
    pub fn new() -> Self {
        Self { lines: vec![], owner: None }
    }
    pub fn new_from(lines: Vec<CodeLine>) -> Self {
        Self { lines, owner: None }
    }
}