extern crate electronic;
use electronic::fca::*;
// use electronic::fca::joule::*;
// use eletronic::fca::potencia::*;

#[test]
fn test_condutancia_eletrica() {
    let negocio = resistencia::f_condutancia_eletrica(125.);
    assert_eq!(0.008, negocio);
}

#[test]
fn test_condutancia_elemento() {
    let negocio = resistencia::f_condutancia_elemento(125., 125., 125.);
    assert_eq!(125.0, negocio);
}

#[test]
fn test_resistencia() {
    let negocio = resistencia::f_resistencia(125., 125.);
    assert_eq!(1.0, negocio);
}

#[test]
fn test_resistencia_serie() {
    let vec1: Vec<f64> = vec![2.0, 4.0, 6.0, 8.0, 10.0];
    let negocio = resistencia::f_resistencia_serie(vec1);
    assert_eq!(30.0, negocio);
}

#[test]
fn test_resistencia_paralelo() {
    let vec1: Vec<f64> = vec![1.0, 3.0, 5.0, 7.0, 9.0];
    let negocio = resistencia::f_resisencia_paralelo(vec1);

    assert_eq!(1.7873015873015872, negocio);
    // 1 = 1
    // 3 = 0.33...
    // 5 = 0.2
    // 7 = 0.1428571428571429
    // 9 = 0.11111
    //

}

#[test]
fn test_resistencia_combinado() {
    let vec1: Vec<f64> = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    let vec2: Vec<f64> = vec![6.0, 7.0, 8.0, 9.0, 10.0];
    let negocio = resistencia::f_resistencia_combinado(vec1, vec2);
    assert_eq!(55., negocio);
}