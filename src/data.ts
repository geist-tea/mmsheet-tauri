export interface CharSheet {
    charinfo: CharInfo;
    abilities: Abilities;
    defense: Defense;
    skills: Skill[];
    offense: Offense[];
    advantages: Advantage[];
    powers: Power[];
}

export interface CharInfo {
    name: string;
    player: string;
    identity: string;
    secret: boolean;
    group: string;
    base: string;
    powerlevel: number;
    exp: number;
}

export interface Abilities {
    strength: number;
    agility: number;
    fighting: number;
    awareness: number;
    stamina: number;
    dexterity: number;
    intellect: number;
    presence: number;
}

export interface Defense {
    dodge: number;
    parry: number;
    fortitude: number;
    toughness: number;
    will: number;
}

export interface Offense {
    name: string,
    otype: string,
    ranks: number,
    damage: number,
    other: string,
}

export interface Skill {
    name: string;
    ability: string;
    ranks: number;
    expertise: boolean;
}

export interface Advantage {
    info: AdvantageInfo;
    ranks: number;
}

export interface AdvantageInfo {
    name: string;
    ranked: boolean;
    summary: string;
    details: string;
}

export interface Power {
    name: string;
    effects: Array<PowerEffect>;
}

export interface PowerEffect {
    name: string;
    cost: number;
    description: string;
    descriptors: string;
    extras: string;
    extrasrankcost: number;
    extrasflatcost: number;
    flaws: string;
    flawscost: number;
    ranks: number;
}

export function NewSheet() {
  return {
            charinfo: {
                name: "",
                player: "",
                identity: "",
                secret: false,
                group: "",
                base: "",
                powerlevel: 10,
                exp: 0,
            },
            abilities: {
                strength: 0,
                agility: 0,
                fighting: 0,
                awareness: 0,
                stamina: 0,
                dexterity: 0,
                intellect: 0,
                presence: 0,
            },
    defense: {
      dodge: 0,
      parry: 0,
      fortitude: 0,
      toughness: 0,
      will: 0,
    },
            skills: [
                {
                    name: "Acrobatics",
                    ability: "Agility",
                    ranks: 0,
                    expertise: false,
                },
                {
                    name: "Athletics",
                    ability: "Strength",
                    ranks: 0,
                    expertise: false,
                },
                {
                    name: "Deception",
                    ability: "Presence",
                    ranks: 0,
                    expertise: false,
                },
                {
                    name: "Insight",
                    ability: "Awareness",
                    ranks: 0,
                    expertise: false,
                },
                {
                    name: "Intimidation",
                    ability: "Presence",
                    ranks: 0,
                    expertise: false,
                },
                {
                    name: "Investigation",
                    ability: "Intelligence",
                    ranks: 0,
                    expertise: false,
                },
                {
                    name: "Perception",
                    ability: "Awareness",
                    ranks: 0,
                    expertise: false,
                },
                {
                    name: "Persuasion",
                    ability: "Presence",
                    ranks: 0,
                    expertise: false,
                },
                {
                    name: "Sleight of Hand",
                    ability: "Dexterity",
                    ranks: 0,
                    expertise: false,
                },
                {
                    name: "Stealth",
                    ability: "Agility",
                    ranks: 0,
                    expertise: false,
                },
                {
                    name: "Technology",
                    ability: "Intelligence",
                    ranks: 0,
                    expertise: false,
                },
                {
                    name: "Treatment",
                    ability: "Intelligence",
                    ranks: 0,
                    expertise: false,
                },
                {
                    name: "Vehicles",
                    ability: "Dexterity",
                    ranks: 0,
                    expertise: false,
                },
            ],
            offense: [],
            advantages: [],
            powers: [],
        }
  }
