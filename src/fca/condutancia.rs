#[allow(dead_code)]
pub struct CondutanciaEletrica { 
    r: f64
}
#[allow(dead_code)]
pub struct CondutanciaElemento {
    r: f64,
    a: f64
}
#[allow(dead_code)]
impl CondutanciaEletrica { 
    pub fn f_condutancia_eletrica(r: f64) -> f64 { 1. / r } 
}
#[allow(dead_code)]
impl CondutanciaElemento {
     pub fn f_condutancia_elemento(r: f64, a: f64) -> f64 { r + a }
}