
struct Item {
    cost: u32,
    damage: u32,
    armour: u32
}

struct Player {
    hp: u32,
    damage: u32,
    armour: u32
}

impl Item {
    fn weapon(cost: u32, damage: u32) -> Self {
        Item{cost, damage, armour:0}
    }

    fn armour(cost: u32, armour: u32) -> Self {
        Item{cost, damage: 0, armour}
    }

    fn ring(cost: u32, damage: u32, armour: u32) -> Self {
        Item{cost, damage, armour}
    }
}


impl Player {
    fn new(hp: u32, damage: u32, armour: u32) -> Self {
        Player{hp, damage, armour}
    }

    pub fn equip_item(&mut self, item: &Item) {
        self.damage += item.damage;
        self.armour += item.armour;
    }
    
    pub fn fight_against(&self, boss: &Player) -> bool {
        true
    }
}



fn part_1() -> u32{
    0
}

fn part_2() -> u32 {
    0
}


fn main() {
    let weapons = vec![Item::weapon(8, 4),
        Item::weapon(10, 5),
        Item::weapon(25, 6),
        Item::weapon(40, 7),
        Item::weapon(74, 8)];

    let armours = vec![
        Item::armour(13, 1),
        Item::armour(31, 2),
        Item::armour(53, 3),
        Item::armour(75, 4),
        Item::armour(102, 5),];

    let rings = vec![
        Item::ring(25, 1, 0),
        Item::ring(50, 2, 0),
        Item::ring(100, 3, 0),
        Item::ring(20, 0, 1),
        Item::ring(40, 0, 2),
        Item::ring(80, 0, 3)];

    let boss = Player::new(103, 9 , 2);

    println!("First puzzle: {}", part_1());
    println!("Second puzzle: {}", part_2());
}
