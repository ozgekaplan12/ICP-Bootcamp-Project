struct Army {
    name: String,
    soldiers: u32,
}

impl Army {
    fn new(name: &str, soldiers: u32) -> Army {
        Army {
            name: String::from(name),
            soldiers,
        }
    }

    fn attack(&mut self, enemy: &mut Army, damage: u32) {
        println!(
            "{} army is dealing {} damage to {} army!",
            self.name, damage, enemy.name
        );
        enemy.soldiers = enemy.soldiers.saturating_sub(damage);
        println!("Remaining soldiers in {} army: {}", enemy.name, enemy.soldiers);
    }
}

fn main() {
    // Create instances of two armies
    let mut army1 = Army::new("Blue", 100);
    let mut army2 = Army::new("Red", 120);

    println!("The war is beginning!");

    // Continue the war as long as both armies have soldiers
    while army1.soldiers > 0 && army2.soldiers > 0 {
        // Perform attacks in turns
        army1.attack(&mut army2, 20);
        army2.attack(&mut army1, 15);
    }

    // Determine the winner
    if army1.soldiers > 0 {
        println!("The Blue army is victorious!");
    } else if army2.soldiers > 0 {
        println!("The Red army is victorious!");
    } else {
        println!("The war ended in a draw!");
    }
}
