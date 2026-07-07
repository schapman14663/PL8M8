fn main() {
    println!("Hello, world!");
}

enum Rounding_Type {
    smart,
    always_down,
    always_up
}

enum Units {
    metric,
    imperial
}

//rounding down is floor division
//ex floor(122.7/2.5) = 49
//
//you can get the number following the decimal (in the above example: 0.08) by doing the followin:
//(X % Y)/ Y
//then to round down subtract this from the original division.
fn round_down(weight: f32, incrememnt: f32) -> f32 {

}

//rounding up is ceiling division
//ex ceil(122.7/2.5) = 50
//as above, but to always round up, you can add (1 - result) to the original division.
fn round_up(weight: f32, increment: f32) -> f32 {

}

//smart division in this case means to the nearest integer regardless of direction.
//the rules above apply, but for smart division we would run the rounding up check, then if (1 -
//result) is above 0.5 apply the round up rule, otherwise apply the round down down rule.
fn smart_round(weight: f32, increment: f32) -> f32 {

}

//standard barbell is 20kg or 45lbs and additional weight would be added to each side.
//as such weight added to the bar is always halved. Ex 80kg added to the bar is 40kg each side. 
//similarly a 2.5kg progression would actually be 1.25kg on each side of the bar.

