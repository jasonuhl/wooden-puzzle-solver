fn main() {
    let mut found = false;
    wooden_puzzle_solver::stream_solutions(|solution_text| {
        print!("{}", solution_text);
        found = true;
    });

    if !found {
        println!("No solution found.");
    }
}
