use macors_de_asobo::FromNum;

#[derive(FromNum,Debug)]
pub enum Foods {
    CurryRice,
    Sushi,
    FriedChicken,
    HamburgSteak = 13,
    FrenchFries,
    Ramen,
    Yakiniku,
    RiceOmlet,
    Pizza,
    FriedRice
}
#[test]
fn main() {
    println!("{:?}",Foods::from_usize(16));
}