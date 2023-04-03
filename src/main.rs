mod rank;
mod squad;
mod soldier;
mod soldier_extent;

use soldier_extent::{SoldierExtent, DefaultSoldierType};
use soldier::Engineer;
use squad::Squad;

fn main() {
    
    // Ekstensja
    let mut soldiers = SoldierExtent::new();

    // Atrybut klasowy
    println!("A soldier's default rank is: {}", DefaultSoldierType::DEFAULT_RANK);

    let john_key = create_soldier!(soldiers, "John");
    // Overloading
    let joe_key = create_soldier!(soldiers, "Joe", Engineer);

    // Shadowing defined in src/soldier.rs for calculate_pay and get/set rank and name

    // Ekstensja - trwałość
    soldiers.save_all("soldiers.json");
    let mut soldiers2 = SoldierExtent::new();
    soldiers2.load_from_file("soldiers.json");

    println!("Soldiers1: {:?}", soldiers.all);
    println!("Soldiers2 (from file): {:?}", soldiers2.all);

    // Metoda przesłonięta (przez klasy StandardSoldier i Engineer)
    println!("Pay of engineer: {}", soldiers.get(joe_key).unwrap().calculate_pay());
    println!("Pay of soldier at rank: {}, {}", soldiers.get(john_key).unwrap().get_rank(), soldiers.get(john_key).unwrap().calculate_pay());
    
    // Metoda klasowa (new)
    let mut alpha_squad = Squad::new("Alpha".to_string(), john_key);

    let bob_key = create_soldier!(soldiers, "Bob");

    alpha_squad.add_member(joe_key);
    alpha_squad.add_member(bob_key);

    println!("Squad: {:?}", alpha_squad);
    // Atrybut powtarzalny
    println!("Squad's members: {:?}", alpha_squad.get_members());

}
