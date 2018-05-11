mod item;
use item::Item;

fn main() {
    let item = Item::new();

    println!("{}", item);
}