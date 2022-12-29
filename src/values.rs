use std::collections::HashMap;

use crate::serialization::DFSerializable;

pub enum Value {
    Text(Text),
    Number(Number),
    Location(Location),
    Vector(Vector),
    Sound(Sound),
    Particle(Particle),
    Potion(Potion),
    Variable(Variable),
    GameValue(GameValue),
    Item(Item)
}

pub type DFNum = f32;
pub type DFNumPrecise = f64;
pub type ParameterList = [Option<Value>; 27];


pub struct Text(pub String);
pub struct Number(pub DFNum);
pub struct Location {
    pub x: DFNum,
    pub y: DFNum,
    pub z: DFNum,
    pub pitch: DFNum,
    pub yaw: DFNum
}
pub struct Vector {
    pub x: DFNumPrecise,
    pub y: DFNumPrecise,
    pub z: DFNumPrecise
}
pub struct Particle {
    pub particle: String,
    pub amount: u64,
    pub color: Option<(u8, u8, u8)>,
    pub variation_color: Option<DFNum>,
    pub material: Option<String>,
    pub motion: Option<Vector>,
    pub variation_motion: Option<DFNum>,
    pub roll: Option<DFNum>,
    pub size: Option<DFNum>,
    pub variation_size: Option<DFNum>,
    pub spread: (DFNum, DFNum),

}

pub struct Potion {
    pub effect: PotionEffect,
    pub ticks: u64,
    pub level: i16
}
pub enum PotionEffect {
    Absorption, ConduitPower, DolphinGrace, FireResistance, Haste, HealthBoost, HeroOfTheVillage,
    InstantHealth, Invisibility, JumpBoost, Luck, NightVision, Regeneration, Resistance, Saturation,
    SlowFalling, Speed, Strength, WaterBreathing, BadLuck, BadOmen, Blindness, Darkness, Glowing,
    Hunger, InstantDamage, Levitation, MiningFatigue, Nausea, Poison, Slowness, Weakness, Wither
}
impl DFSerializable for PotionEffect {
    fn serialize(&self) -> String {
        String::from(match self {
            PotionEffect::Absorption => "Absorption",
            PotionEffect::ConduitPower => "Conduit Power",
            PotionEffect::DolphinGrace => "Dolphin's Grace",
            PotionEffect::FireResistance => "Fire Resistance",
            PotionEffect::Haste => "Haste",
            PotionEffect::HealthBoost => "Health Boost",
            PotionEffect::HeroOfTheVillage => "Hero of the Village",
            PotionEffect::InstantHealth => "Instant Health",
            PotionEffect::Invisibility => "Invisibility",
            PotionEffect::JumpBoost => "Jump Boost",
            PotionEffect::Luck => "Luck",
            PotionEffect::NightVision => "Night Vision",
            PotionEffect::Regeneration => "Regeneration",
            PotionEffect::Resistance => "Resistance",
            PotionEffect::Saturation => "Saturation",
            PotionEffect::SlowFalling => "Slow Falling",
            PotionEffect::Speed => "Speed",
            PotionEffect::Strength => "Strength",
            PotionEffect::WaterBreathing => "Water Breathing",
            PotionEffect::BadLuck => "Bad Luck",
            PotionEffect::BadOmen => "Bad Omen",
            PotionEffect::Blindness => "Blindness",
            PotionEffect::Darkness => "Darkness",
            PotionEffect::Glowing => "Glowing",
            PotionEffect::Hunger => "Hunger",
            PotionEffect::InstantDamage => "Instant Damage",
            PotionEffect::Levitation => "Levitation",
            PotionEffect::MiningFatigue => "Mining Fatigue",
            PotionEffect::Nausea => "Nausea",
            PotionEffect::Poison => "Poison",
            PotionEffect::Slowness => "Slowness",
            PotionEffect::Weakness => "Weakness",
            PotionEffect::Wither => "Wither",
        })
    }
}

pub struct Variable {
    pub name: String,
    pub scope: VariableScope
}
pub enum VariableScope {
    Local, Global, Saved
}
impl DFSerializable for VariableScope {
    fn serialize(&self) -> String {
        String::from(match self {
            VariableScope::Local => "local",
            VariableScope::Global => "unsaved",
            VariableScope::Saved => "saved",
        })
    }
}

pub struct GameValue {
    pub name: String,
    pub selector: Option<Selector>
}
#[derive(Copy,Clone)]
pub enum Selector {
    Selection, Default, Killer, Damager, Victim, Shooter, Projectile, LastEntity, AllPlayers, AllEntities, AllMobs
}
impl Default for Selector {
    fn default() -> Self {
        Self::Default
    }
}
impl DFSerializable for Selector {
    fn serialize(&self) -> String {
        String::from(match self {
            Selector::Selection => "Selection",
            Selector::Default => "Default",
            Selector::Killer => "Killer",
            Selector::Damager => "Damager",
            Selector::Victim => "Victim",
            Selector::Shooter => "Shooter",
            Selector::Projectile => "Projectile",
            Selector::LastEntity => "LastEntity",
            Selector::AllPlayers => "AllPlayers",
            Selector::AllEntities => "AllEntities",
            Selector::AllMobs => "AllMobs",
        })
    }
}

pub struct Sound {
    pub sound: String, // Not adding an enum for every single sound
    pub pitch: DFNum,
    pub volume: DFNum
}

// Incomplete rn
pub struct Item {
    pub material: String,
    pub count: i32,
    pub attributes: Option<Vec<Attribute>>,
    pub flags: Option<u16>,
    pub lore: Option<Vec<String>>,
    pub modeldata: Option<i64>,
    pub name: Option<String>,
    pub unbreakable: bool,
    pub string_tags: HashMap<String, String>,
    pub num_tags: HashMap<String, DFNum>,
}
pub struct Attribute {
    pub uuid: String,
    pub amount: DFNum,
    pub operation: AttributeOperation,
    pub name: String,
    pub slot: String
}
pub enum AttributeOperation {
    AddModifier, MultiplyBase, MultiplyModifier
}