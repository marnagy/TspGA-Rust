use std::io;
use std::fmt::{Display, Result, Formatter};

extern crate rand;
use rand::prelude::*;

fn main() {
    let points_num = 100;

    let mut temp_points: Vec<Point> = Vec::new();
    for _ in 0..points_num {
        let temp_point = Point {
            x: random::<f64>() * (10 as f64),
            y: random::<f64>() * (10 as f64),
        };
        temp_points.push(temp_point);
    }
    let points = temp_points.clone();

    println!("Points");
    for p in &points {
        println!("{},{}", p.x, p.y);
    }
    println!();

    let ngen: i32 = 5_000; //load_number();
    let cx_prob: f64 = 0.7;
    let mut_prob: f64 = 0.2;
    let pop_size: i32 = 50;

    let mut curr_gen: i32 = 0;

    let mut pop: Vec<Vec<usize>> = create_pop(pop_size, points_num as usize);
    pop.sort();
    pop.reverse();
    let mut fits_objs: Vec<(f64, f64)>;
    let mut fits: Vec<f64>;
    let mut objs: Vec<f64>;
    let mut best: f64;
    let mut avg: f64;
    while curr_gen <= ngen {
        fits_objs = pop.iter().map(|p| fitness(&p, &points)).collect();
        fits = fits_objs.iter().map(|fo| fo.0).collect();
        objs = fits_objs.iter().map(|fo| fo.1).collect();

        best = min(&objs);
        avg = average(&objs);
        println!("Gen: {}: best: {}; average: {}", curr_gen, best, avg);

        let mating_pool = tournament_selecion(&pop, fits);
        pop = mutate(crossover(mating_pool, cx_prob), mut_prob);
        pop.sort();
        curr_gen += 1;
    }
}

fn load_number() -> i32 {
    let mut number = String::new();
    println!("Input number of generations:");
    io::stdin()
        .read_line(&mut number)
        .expect("Failed to read max gen number.");
    number.trim().parse().unwrap()
}

fn min(vec: &Vec<f64>) -> f64 {
    if vec.len() == 0 {
        panic!("Cannot find min in empty vector");
    }
    let mut res: f64 = vec[0];
    for i in 0..vec.len() {
        if vec[i] < res {
            res = vec[i];
        }
    };
    res
}

fn average(vec: &Vec<f64>) -> f64 {
    let mut res = 0 as f64;
    for num in vec {
        res += num;
    };
    res / vec.len() as f64
}

fn random_order(max_index: usize) -> Vec<usize> {
    let mut res: Vec<usize> = Vec::new();
    while res.len() < max_index {
        let rand_index: usize = random::<usize>() % max_index;
        if !res.contains(&rand_index) {
            res.push(rand_index);
        }
    }
    res
}

fn create_pop(amount: i32, max_index: usize) -> Vec<Vec<usize>> {
    let mut pop: Vec<Vec<usize>> = Vec::new();
    for _ in 0..amount {
        pop.push(random_order(max_index));
    }
    pop
}

fn mutate(crossed_over: Vec<Vec<usize>>, mut_prob: f64) -> Vec<Vec<usize>> {
    let mut next_gen: Vec<Vec<usize>> = Vec::new();
    for i in 0..crossed_over.len() {
        if random::<f64>() < mut_prob {
            next_gen.push(swap_mut(&crossed_over[i]));
        } else {
            next_gen.push(crossed_over[i].clone());
        }
    }
    next_gen
}

fn swap_mut(ind_c: &Vec<usize>) -> Vec<usize> {
    let mut ind = ind_c.clone();
    let i1 = random::<usize>() % ind.len();
    let mut i2 = random::<usize>() % ind.len();
    while i1 == i2 {
        i2 = random::<usize>() % ind.len();
    }
    let temp = ind[i1];
    ind[i1] = ind[i2];
    ind[i2] = temp;
    ind.clone()
}

fn crossover(mating_pool: Vec<Vec<usize>>, cx_prob: f64) -> Vec<Vec<usize>> {
    let mut next_gen: Vec<Vec<usize>> = Vec::new();
    for i in 0..(mating_pool.len() as f64 / 2 as f64).floor() as usize {
        let tup = one_point_cx(mating_pool[i].clone(), mating_pool[i + 1].clone());

        next_gen.push(tup.0);
        next_gen.push(tup.1);
    }
    next_gen
}

fn one_point_cx(p1: Vec<usize>, p2: Vec<usize>) -> (Vec<usize>, Vec<usize>) {
    let cross_index = random::<usize>() % p1.len();
    let mut o1: Vec<usize> = Vec::new();
    let mut o2: Vec<usize> = Vec::new();
    for i in 0..p1.len() {
        if i < cross_index {
            o1.push(p1[i]);
            o2.push(p2[i]);
        } else {
            o1.push(p2[i]);
            o2.push(p1[i]);
        }
    }
    (o1, o2)
}

fn fitness(individual: &Vec<usize>, points: &Vec<Point>) -> (f64, f64) {
    let mut res = 0 as f64;
    let length = individual.len();
    for i in 0..(length - 1) {
        res += get_distance(&points[individual[i]], &points[individual[i + 1]]);
    }
    res += get_distance(&points[0], &points[length - 1]);
    (1 as f64 / res, res)
}

fn get_distance(p1: &Point, p2: &Point) -> f64 {
    ((p1.x - p2.x).powi(2) + (p1.y - p2.y).powi(2)).sqrt()
}

fn tournament_selecion(population: &Vec<Vec<usize>>, fits: Vec<f64>) -> Vec<Vec<usize>> {
    let mut mating_pool: Vec<Vec<usize>> = Vec::new();
    for _ in 0..population.len() {
        let mut mating_candidate: usize = 0 as usize;
        for i in 0..3 {
            let rand_index = random::<usize>() % population.len();
            if i == 0 {
                mating_candidate = rand_index;
            } else {
                if fits[mating_candidate as usize] < fits[rand_index as usize] {
                    mating_candidate = rand_index as usize;
                }
            }
        }
        mating_pool.push(population[mating_candidate].clone());
    }
    mating_pool
}

struct Point {
    x: f64,
    y: f64,
}

impl Clone for Point {
    fn clone(&self) -> Self {
        *self
    }
}
impl Copy for Point {}

impl Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
