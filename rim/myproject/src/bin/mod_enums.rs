

/*
ENUMs

way to create custom types

Why enums are useful
    ice hockey program
    different positions
    center, wing, defense, goalie

    player can only be one of these at a time.
    can list (enumerate) all possible values
*/

// enum variants
/*
Enums that hold additional data. Hold mutiple pieces of data.
look like a struct.  can hold named data.
*/

enum Clock {
    Sundial(u8),
    Digital(u8,u8),
    Analog(u8,u8,u8),
}

// syntax for def

enum HockeyPosition {
    Center,
    Wing,
    Defense,
    Goalie,
}

fn next_player(position: HockeyPosition) {

}

fn tell_time(clock: Clock) {
    match clock {
        Clock::Sundial(hours) =>
            println!("It is about {} o'clock", hours),
        Clock::Analog(hours, minutes, seconds) => {
            println!(
                "It is {} minutes and {} seconds past {} o'clock",
                minutes, seconds, hours,
            );
        },
        Clock::Digital(hours, minutes) =>
            println!("It is {} minutes past {}", minutes, hours),
    }
}

fn main() {
    let position = HockeyPosition::Defense;
    next_player(position);

    // CLOCK
    tell_time(Clock::Analog(9, 25, 45));
}