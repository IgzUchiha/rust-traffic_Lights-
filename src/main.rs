use std::thread::sleep;
use std::time::Duration;


fn main() {
 let mut green = 0;
 let mut yellow = 0;
 let mut red = 0;


loop{
if green > 0{
    println!("Go!({} seconds remaining", green);
    green -= 1;
} else if yellow > 0{
    println!("Slow Down!({} seconds remaining", yellow);
    yellow -= 1;
} else if red > 0{
    println!("Stop!({} seconds remaining", red);
    red -= 1;
} else {
    println!("Traffic light is broken");
}
 sleep(Duration::from_secs(1));

if green == 0 && yellow == 0 && red == 0 {
    green = green_light_duration();
    yellow = yellow_light_duration();
    red = red_light_duration();
}

}
}



fn green_light_duration() -> u32 {
    30
}
fn yellow_light_duration() -> u32 {
    10
}
fn red_light_duration() -> u32 {
    60
}