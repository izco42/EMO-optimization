/*
pendiente inicializar poblacion aleatoria en base a la definicion del problema y
la definificion de la funcion de aptitud , asi como la asignacion de la carga de la particula
en base a su aptitud , asi como modificar cual es la mejor y peor particula en base a la aptitud?
  COMPLETAD0?  */

/*generalizar la creacion de
best and worst particle a partir de la definicion del problema (minimizar, maximizar) asi como las dimensiones de las particulas
PENDIENTE
*/
mod emo;
mod particle;
use colored::*;
use emo::emo::Emo;
use particle::particle::Particle;
use rand::*;
use std::time::Instant;

fn main() {
    let start = Instant::now();
    let population = initialize_population(20, 4, (1.0, 
    5.0));

    // let mut e = Emo::new(population, booth, 20, "min".to_string());
    let mut e  = Emo::with_constraints(population, panel_solar, 20, "max".to_string(),rules);
    let best = e.run().unwrap();


    println!("");
    println!("{}", format!("Optimization finished").on_purple());
    println!("{}", format!("Best particle:").on_green());
    best.print();
    println!("{}", format!("Worst particle:").on_red());
    e.show_worst();

    let duration = start.elapsed();
    println!(
        "Time elapsed in expensive_function() is: {:?} s",
        duration.as_secs()
    );
}

// fn rosenberg(p: &Particle) -> f64 {
//     let total =
//         (1.0 - p.position[0]).powi(2) + 100.0 * (p.position[1] - p.position[0].powi(2)).powi(2);
//     total
// }

fn initialize_population(num_p: usize, num_d: usize, range: (f64, f64)) -> Vec<Particle> {
    let mut population = Vec::with_capacity(num_p);
    let mut rng = rand::thread_rng();

    for _ in 0..num_p {
        let mut position = Vec::with_capacity(num_d);
        for _ in 0..num_d {
            let x = rng.gen_range(range.0..range.1);
            position.push(x);
        }
        let particle = Particle::new(position);
        population.push(particle);
    }
    population
}

// fn booth(p: &Particle) -> f64 {
//     (p.position[0] + 2.0 * p.position[1] - 7.0).powi(2)
//         + (2.0 * p.position[0] + p.position[1] - 5.0).powi(2)
// }

fn panel_solar(p: &Particle) -> f64 {
    let w = p.position[0];
    let l = p.position[1];
    let t = p.position[2];
    let phi = p.position[3].to_radians();
    let i = 1000.0;

    let hrs = 24.0;
    let jsc = 350.0;
    let eel = jsc * (w * l * phi.cos() / w * l * t) * w * l * phi.cos() * hrs;
    let esol = i * w * l * phi.cos() * hrs;

    eel / esol
}

fn rules(position : Vec<f64>) -> Vec<f64> {

    let mut newp = vec![];
    let w = if position[0] < 1.0 { 1.0 }else if position[0] > 5.0 {5.0} else { position[0] }; 
    newp.push(w);

    let l = if position[1] < 1.0 { 1.0 }else if position[1] > 5.0 {5.0} else { position[1] }; 
    newp.push(l);

    let t = if position[2] < 0.2 { 0.2 }else if position[2] > 0.5 {0.5} else { position[2] };
    newp.push(t);

    let phi = if position[3] < 0.0 { 0.1 }else if position[3] > 360.0 {360.0} else { position[3] };
    newp.push(phi);
    newp
}
