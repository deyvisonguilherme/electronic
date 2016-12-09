#[allow(dead_code)]
pub struct CondutanciaEletrica { 
    pub r: f64 
}
#[allow(dead_code)]
pub struct CondutanciaElemento {
    pub r: f64,
    pub a: f64
}

pub trait FormulaCondutanciaEletrica  {  
    fn f_condutancia_eletrica(&self)  -> f64; 
}

pub trait FormulaCondutanciaElemento {
    fn f_condutancia_elemento(&self)  -> f64;
}

impl FormulaCondutanciaEletrica  for CondutanciaEletrica { 
    fn f_condutancia_eletrica(&self) -> f64 { 
        1. / self.r 
    } 
}

impl FormulaCondutanciaElemento for CondutanciaElemento {
     fn f_condutancia_elemento(&self) -> f64 {
            self.r + self.a
     }
}