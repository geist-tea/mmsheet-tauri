use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CharSheet {
    charinfo: CharInfo,
    abilities: Abilities,
    defense: Defense,
    skills: Vec<Skill>,
    offense: Vec<Offense>,
    advantages: Vec<Advantage>,
    powers: Vec<Power>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CharInfo {
    name: String,
    player: String,
    identity: String,
    secret: bool,
    group: String,
    base: String,
    powerlevel: i32,
    exp: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Abilities {
    strength: i32,
    agility: i32,
    fighting: i32,
    awareness: i32,
    stamina: i32,
    dexterity: i32,
    intellect: i32,
    presence: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Defense {
    dodge: i32,
    parry: i32,
    fortitude: i32,
    toughness: i32,
    will: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Skill {
    name: String,
    ability: String,
    ranks: i32,
    expertise: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Offense {
    name: String,
    otype: OType,
    ranks: i32,
    damage: i32,
    other: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum OType {
    Close,
    Ranged,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Advantage {
    info: AdvantageInfo,
    ranks: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AdvantageInfo {
    name: String,
    ranked: bool,
    summary: String,
    details: String,
    atype: AType,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum AType {
    General,
    Skill,
    Combat,
    Fortune,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Power {
    name: String,
    effects: Vec<PowerEffect>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PowerEffect {
    name: String,
    cost: i32,
    description: String,
    descriptors: String,
    extras: String,
    extrasrankcost: i32,
    extrasflatcost: i32,
    flaws: String,
    flawscost: i32,
    ranks: i32,
}

impl CharSheet {
    pub fn new() -> CharSheet {
        CharSheet {
            charinfo: CharInfo {
                name: String::from("New Hero"),
                player: String::from(""),
                identity: String::from(""),
                secret: false,
                group: String::from(""),
                base: String::from(""),
                powerlevel: 10,
                exp: 0,
            },
            abilities: Abilities {
                strength: 0,
                agility: 0,
                fighting: 0,
                awareness: 0,
                stamina: 0,
                dexterity: 0,
                intellect: 0,
                presence: 0,
            },
            defense: Defense {
                dodge: 0,
                parry: 0,
                fortitude: 0,
                toughness: 0,
                will: 0,
            },
            skills: vec![
                Skill {
                    name: String::from("Acrobatics"),
                    ability: String::from("Agility"),
                    ranks: 0,
                    expertise: false,
                },
                Skill {
                    name: String::from("Athletics"),
                    ability: String::from("Strength"),
                    ranks: 0,
                    expertise: false,
                },
                Skill {
                    name: String::from("Deception"),
                    ability: String::from("Presence"),
                    ranks: 0,
                    expertise: false,
                },
                Skill {
                    name: String::from("Insight"),
                    ability: String::from("Awareness"),
                    ranks: 0,
                    expertise: false,
                },
                Skill {
                    name: String::from("Intimidation"),
                    ability: String::from("Presence"),
                    ranks: 0,
                    expertise: false,
                },
                Skill {
                    name: String::from("Investigation"),
                    ability: String::from("Intelligence"),
                    ranks: 0,
                    expertise: false,
                },
                Skill {
                    name: String::from("Perception"),
                    ability: String::from("Awareness"),
                    ranks: 0,
                    expertise: false,
                },
                Skill {
                    name: String::from("Persuasion"),
                    ability: String::from("Presence"),
                    ranks: 0,
                    expertise: false,
                },
                Skill {
                    name: String::from("Sleight of Hand"),
                    ability: String::from("Dexterity"),
                    ranks: 0,
                    expertise: false,
                },
                Skill {
                    name: String::from("Stealth"),
                    ability: String::from("Agility"),
                    ranks: 0,
                    expertise: false,
                },
                Skill {
                    name: String::from("Technology"),
                    ability: String::from("Intelligence"),
                    ranks: 0,
                    expertise: false,
                },
                Skill {
                    name: String::from("Treatment"),
                    ability: String::from("Intelligence"),
                    ranks: 0,
                    expertise: false,
                },
                Skill {
                    name: String::from("Vehicles"),
                    ability: String::from("Dexterity"),
                    ranks: 0,
                    expertise: false,
                },
            ],
            offense: Vec::new(),
            advantages: Vec::new(),
            powers: Vec::new(),
        }
    }
}
