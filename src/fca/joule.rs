// Cálculo de potência elétrica
pub fn f_joule(i: f64, r: f64, t: f64) -> f64 {
    i.powf(2.0) * r * t
}

// Converter Joule para Wats
pub fn joule_to_wats(j: f64, s: f64) -> f64 {
    j / s
}