/// Show that variable cannot be used after move.

#[derive(Debug)]    // <1>
enum Cereal {       // <2>
    Barley, Millet, Rice,
    Rye, Spelt, Wheat,
}

fn main() {
    let mut grains: Vec<Cereal> = vec![];   // <3>
    grains.push(Cereal::Rye);               // <4>
    // grains is moved here.
    drop(grains);                           // <5>

    // grains cannot be used here.
    println!("{:?}", grains);               // <6>
}
