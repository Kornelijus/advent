mod day1;
mod day2;
mod day6;

fn main() {
    print!(
        concat!(
            "Advent of Code 2021 🎄 \n",
            "Day   Solution \n",
            "1     {:?} \n",
            "2     {:?} \n",
            "6     {:?} \n",
        ),
        day1::solution(),
        day2::solution(),
        day6::solution()
    );
}
