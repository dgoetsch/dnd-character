use crate::core::ability_score::ModifiedAbilityScores;
use crate::core::effect::Effect;
use crate::core::roll::{CheckRoll, DamageRoll, DamageRollScope};
use crate::core::Damage;

#[derive(Debug, Clone)]
pub enum AttackRange {
    Melee,
    Range { normal: isize, long: isize },
}

#[derive(Debug, Clone)]
pub struct WeaponAttack {
    name: String,
    range: AttackRange,
    attack: CheckRoll,
    damage: DamageRoll,
}

impl WeaponAttack {
    pub fn new<T: Into<String>>(
        name: T,
        range: AttackRange,
        attack: CheckRoll,
        damage: DamageRoll,
    ) -> WeaponAttack {
        WeaponAttack {
            name: name.into(),
            range,
            attack,
            damage,
        }
    }

    fn apply_all(&self, effects: &Vec<Effect>) -> WeaponAttack {
        let mut new: WeaponAttack = self.clone();
        for effect in effects {
            new = new.apply(effect.clone());
        }
        new
    }

    fn apply(&self, effect: Effect) -> WeaponAttack {
        match effect {
            Effect::Damage { damage, scope } => match scope {
                DamageRollScope::Attack => self.with_extra_damage(damage),
                _ => self.clone(),
            },
            _ => self.clone(),
        }
    }

    fn with_extra_damage(&self, additional: Damage) -> WeaponAttack {
        let mut attack = self.clone();
        attack.damage = attack.damage.with_extra_damage(additional);
        attack
    }

    fn with_extra_check(&self, additional: CheckRoll) -> WeaponAttack {
        let mut attack = self.clone();
        attack.attack = attack.attack.merge(additional);
        attack
    }
}
