#[allow(dead_code)]
pub struct Joule{
    i: f64, // Intensidade da corrente
    r: f64, // Resistência do condutor
    t: f64  // Tempo pelo qual a corrente o condutor
}

#[allow(dead_code)]
impl Joule {
    pub fn f_joule(i: f64, r: f64, t: f64) -> f64 { i.powf(2.0) * r * t }
}