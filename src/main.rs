mod solver;
use solver::solve;

fn main() {
    let solutions = solve(vec![2,3,100, 10], 30);
    println!("{:?}", solutions);
}
