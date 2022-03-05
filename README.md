## DND Character Sheet

Helps you organize and track your character stats.

This is a toy UI written in rust, and the data model in this application is largely a predecesor to [dnd-cli](https://github.com/dgoetsch/dnd-cli).

## Execute

```
## built in options are bashid & vynne; 
## create additional characters in ~/.store/characters/ to extend
export CHARACTER_NAME="bashid" 
cargo run
```

### Features
* hp tracker
* proficiencies
* character descriptiongeneric feature damage/check rolls
* apply effects to generic fea
* display abilities
* apply feature + item effects to abilities (used everywhere)
* apply feature + item effects to saving throws
* Equipment / item effects
* Weapon attack + damage
* display skills
* spell slots tracker
* spell casting DC + attack modifier (derived + includes effects)
* generic feature ability slot tracker

#### TODO
* short / long rest
* in app dice rolls
* apply effects to skills
* styling
* refine proficiency model
* apply effects to proficiencies
* generic feature damage/check rolls
* apply effects to generic feature roles
* apply effects to health
* armor class,
* experience
* base attack melee / range (pull out of weapon/inventory)
* templates
    * class and race
    * apply generic features lik
        * spell slots
        * class features / abilities
        * proficiencies
* edit sources inline
    * add features
    * modify abilities scores
    * add inventory
    * equip / unequip