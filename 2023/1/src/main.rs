mod puzzle;
mod utils;

use puzzle::AsCallibValues;

fn main() {
    let values = utils::read_lines("input").as_callibration();
    println!("{}", values.iter().sum::<u32>());
}
