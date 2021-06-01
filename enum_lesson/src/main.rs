use std::fs::File;

fn main() {
    enum DispenserItem {
        Empty,
        Ammo(u8),
        Things(String, i32),
        Place { x: i32, y: i32 },
    }

    use DispenserItem::*;
    let item = Empty;
    let item_place = Place { x: 24, y: 48 };
    match item_place {
        DispenserItem::Place { x, y } => println!("{} {}", x, y),
        _ => println!("Somthing else"),
    }

    if let DispenserItem::Place { x, y } = item_place {
        println!("{} {}", x, y);
    }

    let res = File::open("foo");
    match res {
        Ok(f) => println!("Success"),
        Err(e) => println!("Failed"),
    }
}
