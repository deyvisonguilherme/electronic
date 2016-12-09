struct CE { r: f64 }
trait Formula{ 
    fn f_CE(&self) -> f64;
    fn f_MA(&self) -> f64;
}