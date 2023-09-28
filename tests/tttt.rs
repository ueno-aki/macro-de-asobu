use macors_de_asobu::FromNum;

#[derive(FromNum,Debug)]
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
    println!("{:?}",Foods::from_usize(13));
}