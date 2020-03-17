use itertools::Itertools;

#[derive(Debug)]
struct Spell {
    cost: u32,
    mana: u32,
    damage: u32,
    armour: u32,
    hp: u32,
    rounds: u32
}

#[derive(Debug)]
struct Player {
    hp: i32,
    damage: u32,
    armour: u32,
}

impl Spell {
    fn magic_missle() -> Self {
        Spell {
            cost: 53,
            mana: 0,
            damage: 4,
            armour: 0,
            hp: 0,
            rounds: 0,
        }
    }

    fn drain() -> Self {
        Spell {
            cost: 73,
            mana: 0,
            damage: 2,
            armour: 0,
            hp: 2,
            rounds: 0
        }
    }
    fn shield() -> Self {
        Spell {
            cost: 113,
            mana: 0,
            damage: 0,
            armour: 7,
            hp: 2,
            rounds: 6
        }
    }

    fn poison() -> Self {
        Spell {
            cost: 173,
            mana: 0,
            damage: 3,
            armour: 0,
            hp: 0,
            rounds: 6
        }
    }

    fn recharge() -> Self {
        Spell {
            cost: 229,
            mana: 101,
            damage: 0,
            armour: 0,
            hp: 0,
            rounds: 5
        }
    }
}

impl Player {
    fn new(hp: i32, damage: u32, armour: u32) -> Self {
        Player { hp, damage, armour }
    }

    pub fn equip_item(&mut self, item: &Spell) {
        self.damage += item.damage;
        self.armour += item.armour;
    }

    fn get_damage_against(&self, enemy: &Player) -> u32 {
        std::cmp::max(1, self.damage.saturating_sub(enemy.armour))
    }
}

fn does_player_wins(mut player: Player, mut enemy: Player) -> bool {
    let player_damage = player.get_damage_against(&enemy) as i32;
    let enemy_damage = enemy.get_damage_against(&player) as i32;
    loop {
        enemy.hp -= player_damage;
        if enemy.hp <= 0 {
            return true;
        }

        player.hp -= enemy_damage;
        if player.hp <= 0 {
            return false;
        }
    }
}

fn part_1() -> u32 {
    0
}

fn part_2() -> u32 {
    0
}

fn main() {
    let enemy = Player::new(51, 9, 0);
    println!("First puzzle: {}", part_1());
    println!("Second puzzle: {}", part_2());
}

#[cfg(test)]
mod day21 {
    use super::*;

    #[test]
    fn test_part_1() {
        let boss = Player::new(12, 7, 2);
        let player = Player::new(8, 5, 5);
        assert_eq!(true, does_player_wins(player, boss));

        let boss = Player::new(103, 9, 2);
        let mut player = Player::new(100, 0, 0);
        player.equip_item(&Item { cost: 10, damage: 5, armour: 0 });
        player.equip_item(&Item { cost: 13, damage: 0, armour: 1 });
        player.equip_item(&Item { cost: 25, damage: 1, armour: 0 });
        player.equip_item(&Item { cost: 100, damage: 3, armour: 0 });

        assert_eq!(false, does_player_wins(player, boss));
    }

    #[test]
    fn test_player_get_damage_against() {
        let boss = Player::new(100, 100, 100);
        let player = Player::new(10, 10, 10);
        assert_eq!(1, player.get_damage_against(&boss));
        assert_eq!(90, boss.get_damage_against(&player));
    }
}
