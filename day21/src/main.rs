use itertools::Itertools;

#[derive(Debug)]
struct Item {
    cost: u32,
    damage: u32,
    armour: u32,
}

#[derive(Debug)]
struct Player {
    hp: i32,
    damage: u32,
    armour: u32,
}

impl Item {
    fn weapon(cost: u32, damage: u32) -> Self {
        Item {
            cost,
            damage,
            armour: 0,
        }
    }

    fn armour(cost: u32, armour: u32) -> Self {
        Item {
            cost,
            damage: 0,
            armour,
        }
    }

    fn ring(cost: u32, damage: u32, armour: u32) -> Self {
        Item {
            cost,
            damage,
            armour,
        }
    }
}

impl Player {
    fn new(hp: i32, damage: u32, armour: u32) -> Self {
        Player { hp, damage, armour }
    }

    pub fn equip_item(&mut self, item: &Item) {
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

fn part_1(weapons: &[Item], armours: &[Item], rings: &[Item]) -> u32 {
    let mut costs = std::collections::HashSet::new();
    for weapon in weapons.iter() {
        for armour in armours.iter() {
            for ring in rings.iter().combinations(2) {
                let mut player = Player::new(100, 0, 0);
                let boss = Player::new(103, 9, 2);
                player.equip_item(weapon);
                player.equip_item(armour);
                player.equip_item(ring[0]);
                player.equip_item(ring[1]);
                if does_player_wins(player, boss) {
                    let cost = weapon.cost + armour.cost + ring[0].cost + ring[1].cost;

                    costs.insert(cost);
                }
            }
        }
    }
    *costs.iter().min().unwrap()
}

fn part_2(weapons: &[Item], armours: &[Item], rings: &[Item]) -> u32 {
    let mut costs = std::collections::HashSet::new();
    for weapon in weapons.iter() {
        for armour in armours.iter() {
            for ring in rings.iter().combinations(2) {
                let mut player = Player::new(100, 0, 0);
                let boss = Player::new(103, 9, 2);
                player.equip_item(weapon);
                player.equip_item(armour);
                player.equip_item(ring[0]);
                player.equip_item(ring[1]);
                if !does_player_wins(player, boss) {
                    let cost = weapon.cost + armour.cost + ring[0].cost + ring[1].cost;

                    costs.insert(cost);
                }
            }
        }
    }
    *costs.iter().max().unwrap()
}

fn main() {
    let weapons = vec![
        Item::weapon(8, 4),
        Item::weapon(10, 5),
        Item::weapon(25, 6),
        Item::weapon(40, 7),
        Item::weapon(74, 8),
    ];

    let armours = vec![
        Item::armour(0, 0),
        Item::armour(13, 1),
        Item::armour(31, 2),
        Item::armour(53, 3),
        Item::armour(75, 4),
        Item::armour(102, 5),
    ];

    let rings = vec![
        Item::ring(0, 0, 0),
        Item::ring(0, 0, 0),
        Item::ring(25, 1, 0),
        Item::ring(50, 2, 0),
        Item::ring(100, 3, 0),
        Item::ring(20, 0, 1),
        Item::ring(40, 0, 2),
        Item::ring(80, 0, 3),
    ];

    println!("First puzzle: {}", part_1(&weapons, &armours, &rings));
    println!("Second puzzle: {}", part_2(&weapons, &armours, &rings));
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
