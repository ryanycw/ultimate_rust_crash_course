const STARTING_MISSILES: i32 = 8;
const READY_AMOUNT: i32 = 2;

fn main() {
    READY_AMOUNT = 1;
    //let temp: i32;
    let (missiles, ready) = (STARTING_MISSILES, READY_AMOUNT);
    println!("Firing {} of my {} missiles...", ready, missiles);

    //missiles = missiles - ready;
    println!("{} missiles left", missiles - ready);
}
