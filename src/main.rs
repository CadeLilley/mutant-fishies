use std::cmp::Ordering;
use std::cmp::min;
use std::cmp::max;

use rand::{thread_rng, Rng};


fn main() {
    let mut population = generate_initial_population(); // [Solution; 100]
    //let mut best_mutation = Solution{..Default::default()};

    for i in 0..20000000 {
    //for _ in 0..ITERATIONS {
        let sorted_population = grade_population(population); // Returns sorted by grade
        let best_mutation = sorted_population[0]; // Saves best mutation
        if i%1000 == 0 {
            print!("{}: ", best_mutation.grade);
            for m in best_mutation.sequence.iter() {
                print!("{}", m);
            }
            println!("");
        }
        population = mutate(sorted_population);
    }
}

fn mutate(mut population: [Solution; 100]) -> [Solution; 100] {
    let mut rng = thread_rng();

    // Save the top 10 for the next generation


    // Mate some of the top 10 together
    // This creates 20 of the population since mate returns two progeny
    for i in (10..30).step_by(2) {
        let result = mate(population[rng.gen_range(0..10)], population[rng.gen_range(0..10)]);
        population[i] = result.0;
        population[i+1] = result.1;
    }

    // Iterate and mate one of the top and one of the bottom together
    // Creates 20 of the population since mate returns two progeny
    for i in (30..50).step_by(2) {
        let result = mate(population[rng.gen_range(0..10)], population[rng.gen_range(90..100)]);
        population[i] = result.0;
        population[i+1] = result.1;
    }

    // Randomly mutate the next 40 percent of the population
    // Creates 40 of the population
    for _ in 50..90 {
        let mut rng = thread_rng();
        match rng.gen_range(0..3) {
            0 => {
                let mut_index = rng.gen_range(0..30);
                population[mut_index] = point_mutation(population[mut_index]);
            },
            1 => {
                let mut_index = rng.gen_range(0..30);
                population[mut_index] = swap_mutation(population[mut_index]);
            },
            2 => {
                let mut_index = rng.gen_range(0..30);
                population[mut_index] = reverse_mutation(population[mut_index]);
            },
            _ => println!("Error in RNG generation")
        }
    }

    // Randomly generate the last ten of the population
    let temp_gen = generate_initial_population();
    for i in 90..100 {
        population[i] = temp_gen[i];
    }

    return population
}

fn mate(mut sol_one: Solution, mut sol_two: Solution) -> (Solution, Solution) {
    let mut rng = thread_rng();
    let random_indexes = [rng.gen_range(0..30), rng.gen_range(0..30)];
    let start = min(random_indexes[0], random_indexes[1]);
    let end = max(random_indexes[0], random_indexes[1]);

    for i in start..end {
        let temp_val = sol_one.sequence[i];
        sol_one.sequence[i] = sol_two.sequence[i];
        sol_two.sequence[i] = temp_val;
    }

    return (sol_one, sol_two)
}

fn point_mutation(mut sol: Solution) -> Solution {
    let mut rng = thread_rng();
    let index = rng.gen_range(0..30);
    let direction = rng.gen_range(0..4);
    sol.sequence[index] = direction;
    return sol
}

fn swap_mutation(mut sol: Solution) -> Solution {
    let mut rng = thread_rng();
    let i_one = rng.gen_range(0..30);
    let i_two = rng.gen_range(0..30);

    let temp_move = sol.sequence[i_one];
    sol.sequence[i_one] = sol.sequence[i_two];
    sol.sequence[i_two] = temp_move;

    return sol
}

fn reverse_mutation(mut sol: Solution) -> Solution {
    let mut rng = thread_rng();
    let i_one = rng.gen_range(0..30);
    let i_two = rng.gen_range(0..30);
    let start = min(i_one, i_two);
    let end = min(i_one, i_two);

    for i in (0..end-start).step_by(2) {
        let temp = sol.sequence[start];
        sol.sequence[start+i] = sol.sequence[end-i];
        sol.sequence[end-i] = temp;
    }
    return sol
}

fn generate_initial_population() -> [Solution; 100] {
    let mut rng = thread_rng();
    let mut population = [Solution{..Default::default()}; 100];

    for solution in 0..100 {
        for index in 0..30 {
            population[solution].sequence[index] = rng.gen_range(0..3);
        }
    }

    return population
}

fn grade_population(mut population: [Solution; 100]) -> [Solution; 100] {
    for i in 0..100 {
        population[i].grade = grade_solution(population[i].sequence);
    }
    population.sort_by(|a, b| a.grade.partial_cmp(&b.grade).unwrap());
    population.reverse();
    return population
}

fn grade_solution(sequence: [usize; 30]) -> f64 {
    let mut score = 0.0;
    let mut fish_map = [
    [-2.0,  1.0,  5.0,  3.0],
    [ 5.0, -2.0, -2.0, -2.0],
    [ 3.0,  1.0,  3.0,  5.0],
    [ 1.0, -2.0,  5.0, -2.0],
    [-2.0,  3.0, -2.0,  5.0]
    ];
    let mut pos = (0, 0);

    // 0, 1, 2, 3 => Left, Right, Up, Down respectively
    for m in sequence.iter() {
        match m {
            0 => {
                match pos.0 {
                    0 => pos.0 = 3,
                    _ => pos.0 = pos.0 - 1
                };
                score = score + fish_map[pos.1][pos.0];
            },
            1 => {
                match pos.0 {
                    3 => pos.0 = 0,
                    _ => pos.0 = pos.0 + 1
                }
                score = score + fish_map[pos.1][pos.0];
            },
            2 => {
                match pos.1 {
                   0 => pos.1 = 4,
                   _ => pos.1 = pos.1 - 1
               }
               score = score + fish_map[pos.1][pos.0];
            },
            3 => {
                match pos.1 {
                    4 => pos.1 = 0,
                    _ => pos.1 = pos.1 + 1
                }
                score = score + fish_map[pos.1][pos.0];
            },
            _ => println!("Move not possible in grade_solution()")
        }

        match fish_map[pos.1][pos.0] > 0.0 {
            true => fish_map[pos.1][pos.0] = fish_map[pos.1][pos.0] * 0.9,
            false => fish_map[pos.1][pos.0] = fish_map[pos.1][pos.0] * 1.1
        }
    }

    return score
}

#[derive(Copy, Clone)]
struct Solution {
    grade: f64,
    sequence: [usize; 30]
}

impl Default for Solution {
    fn default() -> Solution {
        return Solution { grade: 0.0, sequence: [0; 30] }
    }
}

impl Ord for Solution {
    fn cmp(&self, other: &Self) -> Ordering {
        self.grade.partial_cmp(&other.grade).unwrap()
    }
}


impl PartialOrd for Solution {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(Some(self.cmp(other)).unwrap())
    }
}


impl PartialEq for Solution {
    fn eq(&self, other: &Self) -> bool {
        self.grade == other.grade
    }
}

impl Eq for Solution { }
