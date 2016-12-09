struct CE { r: f64 }
trait Formula{ 
    fn f_ce(&self) -> f64;
    fn f_ma(&self) -> f64;
}