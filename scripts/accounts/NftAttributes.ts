// Define the structure matching your Rust program
export class NftAttributes {
  class: number;
  hp: number;
  energy: number;
  exp: number;
  allocated_point: number;
  attack: number;
  defense: number;
  speed: number;
  max_hp: number;
  max_energy: number;

  constructor(fields: {
    class: number;
    hp: number;
    energy: number;
    exp: number;
    allocated_point: number;
    attack: number;
    defense: number;
    speed: number;
    max_hp: number;
    max_energy: number;
  }) {
    this.class = fields.class;
    this.hp = fields.hp;
    this.energy = fields.energy;
    this.exp = fields.exp;
    this.allocated_point = fields.allocated_point;
    this.attack = fields.attack;
    this.defense = fields.defense;
    this.speed = fields.speed;
    this.max_hp = fields.max_hp;
    this.max_energy = fields.max_energy;
  }
}

// Define the schema for Borsh deserialization
export const NftAttributesSchema = new Map([
  [
    NftAttributes,
    {
      kind: "struct",
      fields: [
        ["class", "u8"],
        ["hp", "u16"],
        ["energy", "u16"],
        ["exp", "u16"],
        ["allocated_point", "u16"],
        ["attack", "u16"],
        ["defense", "u16"],
        ["speed", "u16"],
        ["max_hp", "u16"],
        ["max_energy", "u16"],
      ],
    },
  ],
]);
