mod solver;
use solver::solve;

fn main() {
    let solutions = solve(vec![2,3,100,10,6], 294);
    println!("{:?}", solutions);
}
