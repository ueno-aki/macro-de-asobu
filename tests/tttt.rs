use macors_de_asobu::from_num;

#[derive(Debug)]
#[from_num(i32,usize)]
pub enum Foods {
    CurryRice,
    Sushi,
    FriedChicken,
    HamburgSteak = 0o12,
    FrenchFries,
    Ramen,
    Yakiniku,
    RiceOmlet = 0xfe,
    Pizza,
    FriedRice
}

#[test]
fn main() {
    println!("{:?}",Foods::from(0o16));
}