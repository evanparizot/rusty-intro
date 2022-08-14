#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red, Blue
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let (mut red, mut blue) =(0, 0);

        for color in &self.shirts {
            match color {
                ShirtColor::Red => red += 1,
                ShirtColor::Blue => blue += 1
            }
        }

        if red > blue {ShirtColor::Red} else {ShirtColor::Blue}
    }
}
fn main() {
    println!("Hello, world!");
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref_one = Some(ShirtColor::Red);
    let giveaway_one = store.giveaway(user_pref_one);
    println!("The user with preference {:?} gets {:?}", user_pref_one, giveaway_one);

    let user_pref_two = None;
    let giveaway_two = store.giveaway(user_pref_two);
    println!("The user with preference {:?} gets {:?}", user_pref_two, giveaway_two);
}
