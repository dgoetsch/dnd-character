use crate::core::ability_score::Ability;
use crate::core::Damage;

pub enum Scope {
    Path(Vec<String>),
    Character,
}

pub enum Modifier {
    ArmorClass { bonus: isize },
    Spellcasting { bonus: isize },
    Ability { ability: Ability, bonus: isize },
    Attack { bonus: isize, scope: Scope },
    DamageBonus { damage: Damage, scope: Scope },
}
