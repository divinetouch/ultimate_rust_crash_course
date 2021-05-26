const STARTING_MISSILES: i32 = 8;
const READY_AMOUNT: i32 = 2;

use variables::greet;

fn main() {
    let mut missiles = STARTING_MISSILES;
    let ready: i32 = READY_AMOUNT;
    println!("Firing {} of my {} missiles...", ready, missiles);
    missiles = missiles - ready;
    println!("{} missiles left", missiles);

    greet();

    let x = 5_u16;
    let y = 3.14_f32;

    println!("{} {}", x, y);

    // compound types
    let info = (1, 3.3, 999);
    // let jets = info.0;
    // let fuel = info.1;
    // let ammo = info.2;

    let (jets, fuel, ammo) = info;

    println!("{} {} {}", jets, fuel, ammo);
}

fn do_stuff(qty: f64, oz: f64) -> f64 {
    qty * oz // tail expression, automatically return
}
