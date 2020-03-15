
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
    println!("First puzzle: {}", part_1());
    println!("Second puzzle: {}", part_2());
}
