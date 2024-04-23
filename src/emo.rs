/*
1. Inicializar particulas random (la carga de la particula es dada por la funcion de aptitud)
2. Evaluar la funcion de aptitud de cada particula
3. Guardar la mejor particula
4. Las demas particulas se mueven hacia la mejor particula
   se calcula la fuerza neta de cada particula y con eso su nueva posicion en el espacio
6. Se repite el proceso hasta que se cumpla un criterio de parada
7. Se retorna la mejor particula
*/
#[allow(dead_code)]
pub mod emo {
    use crate::particle::particle::Particle;
    //use colored::*;

    pub struct Emo {
        particles: Vec<Particle>,
        best_particle: Particle,
        worst_particle: Particle,
        aptitude: fn(p: &Particle) -> f64,
        max_iterations: i32,
        problem: String,
        constraints : fn(Vec<f64>) -> Vec<f64>
    }

    impl Emo {
        pub fn new(
            particles: Vec<Particle>,
            aptitude: fn(p: &Particle) -> f64,
            max_iterations: i32,
            problem: String,
        ) -> Emo {
            let dimension = particles[0].position.len();
            Emo {
                particles,
                aptitude,
                max_iterations,
                best_particle: initialize_best(dimension, &problem),
                worst_particle: initialize_worst(dimension, &problem),
                problem,
                constraints : filter_constraints
            }
        }

        pub fn with_constraints(
            particles: Vec<Particle>,
            aptitude: fn(p: &Particle) -> f64,
            max_iterations: i32,
            problem: String,
            constraints : fn(Vec<f64>) -> Vec<f64>
        ) -> Emo {
            let dimension = particles[0].position.len();
            Emo {
                particles,
                aptitude,
                max_iterations,
                best_particle: initialize_best(dimension, &problem),
                worst_particle: initialize_worst(dimension, &problem),
                problem,
                constraints
            }
        }
        
        pub fn show_particles(&self) {
            for p in &self.particles {
                println!("{:?}", p.print());
            }
        }

        pub fn show_worst(&self) {
            println!("{:?}", self.worst_particle.print());
        }

        pub fn update_fitness(&mut self) {
            self.best_particle.position = (self.constraints)(self.best_particle.position.clone());
            self.best_particle.charge = (self.aptitude)(&self.best_particle);

            self.worst_particle.position = (self.constraints)(self.worst_particle.position.clone());
            self.worst_particle.charge = (self.aptitude)(&self.worst_particle);

            for p in &mut self.particles {
                p.position = (self.constraints)(p.position.clone());
                p.charge = (self.aptitude)(p);

            }
        }

        pub fn update_best_worst(&mut self) {
            if self.problem == "min"{
                for p in &self.particles {
                    if p.charge < self.best_particle.charge {
                        //println!("Updating best with charge: {}", p.charge);
                        self.best_particle = p.clone();
                    }
                    if p.charge > self.worst_particle.charge {
                        //println!("Updating worst with charge: {}", p.charge);
                        self.worst_particle = p.clone();
                    }
                }
            }
            else{
                for p in &self.particles {
                    if p.charge > self.best_particle.charge {
                        //println!("Updating best with charge: {}", p.charge);
                        self.best_particle = p.clone();
                    }
                    if p.charge < self.worst_particle.charge {
                        //println!("Updating worst with charge: {}", p.charge);
                        self.worst_particle = p.clone();
                    }
                }
            }
        }

        pub fn run(&mut self) -> Result<Particle, &'static str> {
            for it in 0..self.max_iterations {
                self.update_fitness();
                self.update_best_worst();
                for p in &mut self.particles {
                    if &p.position == &self.best_particle.position {
                        continue;
                    }
                    let d = euclidean_distance(p.clone(), self.best_particle.clone()).unwrap();
                    let mut new_position = vec![];
                    for i in 0..p.position.len() {
                        let mut f = force(p.clone(), self.best_particle.clone(), d, i).unwrap();
                        f = normalize(f, d).unwrap();
                        let mut a =
                        acceleration(p.clone(), self.best_particle.clone(), d, i, f).unwrap();
                        a = normalize(a, d).unwrap();
                        let mut new_component =
                        cal_component(p.clone(), a, i, it as f64 + 1.0).unwrap();
                        new_component = normalize(new_component.clone(), d).unwrap();
                        //p.position[i] = new_component;
                        new_position.push(new_component);
                    }
                    new_position = (self.constraints)(new_position);
                    p.update_position(new_position);
                }
            }
            Ok(self.best_particle.clone())
        }
    }

    pub fn force(p1: Particle, p2: Particle, distance: f64, d: usize) -> Result<f64, &'static str> {
        let force = p1.charge * p2.charge * (p2.position[d] - p1.position[d]) / distance.powi(3);
        if !force.is_nan() {
            Ok(force)
        } else {
            Err("Force is NaN")
        }
    }

    pub fn euclidean_distance(p1: Particle, p2: Particle) -> Result<f64, &'static str> {
        let mut d = 0.0;
        for i in 0..p1.position.len() {
            d += (p2.position[i] - p1.position[i]).powi(2);
        }
        if d < 0.0 {
            d = d * -1.0;
        }
        if !d.sqrt().is_nan() {
            Ok(d.sqrt())
        } else {
            Err("Distance is NaN")
        }
    }

    pub fn acceleration(
        p1: Particle,
        p2: Particle,
        distance: f64,
        d: usize,
        force: f64,
    ) -> Result<f64, &'static str> {
        let a = force * (p2.position[d] - p1.position[d]) / distance;
        if !a.is_nan() {
            Ok(a)
        } else {
            Err("Acceleration is NaN")
        }
    }

    pub fn cal_component(p: Particle, a: f64, d: usize, t: f64) -> Result<f64, &'static str> {
        let new_component = p.position[d] + 0.5 * a * t.powi(2);
        if !new_component.is_nan() {
            Ok(new_component)
        } else {
            Err("New component is NaN")
        }
    }

    fn normalize(component: f64, magnitude: f64) -> Result<f64, &'static str> {
        if magnitude != 0.0 {
            Ok(component / magnitude)
        } else {
            Err("Magnitude is 0")
        }
    }
    fn initialize_best(dimension: usize, problem: &str) -> Particle {
        if problem == "min" {
            let best = Particle::new(vec![9999.0; dimension]);
            return best;
        } else {
            let best = Particle::new(vec![1.0; dimension]);
            return best;
        }
    }

    fn initialize_worst(dimension: usize, problem: &str) -> Particle {
        if problem == "min" {
            let worst = Particle::new(vec![1.0; dimension]);
            return worst;
        } else {
            let worst = Particle::new(vec![9999.0; dimension]);
            return worst;
        }
    }

    fn filter_constraints(position: Vec<f64>) -> Vec<f64> {
        position
    }
}
