/*
cada particula tiene un vector n-dimensional (cada dimension es una variable del problema)
debe haber metodos para actualizar su posicion en el espacio de busqueda , que no es mas que otra solucion del problema
*/
pub mod particle {
    use colored::*;

    #[derive(Clone, Debug)]
    pub struct Particle {
        pub position: Vec<f64>,
        pub charge: f64,
    }

    #[allow(dead_code)]
    impl Particle {
        pub fn new(position: Vec<f64>) -> Particle {
            Particle {
                position,
                charge: 0.0,
            }
        }
        pub fn print(&self) {
            println!(
                "{}",
                format!("(P:{:?},CH:{:?})", self.position, self.charge).blue()
            );
        }
    }
}
