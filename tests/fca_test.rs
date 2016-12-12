extern crate electronic;
use electronic::fca::*;
// use electronic::fca::joule::*;
// use eletronic::fca::potencia::*;

#[test]
fn test_condutancia_eletrica() {
    let negocio = resistencia::f_condutancia_eletrica(125.); 
     assert_eq!( 0.008 , negocio );
}

#[test]
fn test_condutancia_elemento(){
    let negocio = resistencia::f_condutancia_elemento(125., 125., 125.);
    assert_eq!( 250.0, negocio );
}

#[test]
fn test_resistencia(){
    let negocio = resistencia::f_resistencia(125., 125.);
    assert_eq!(24., negocio);
}

#[test]
fn test_resistencia_serie() {
    unimplemented!();
}

#[test]
fn test_resistencia_paralelo() {
    unimplemented!();
}

#[test]
fn test_resistencia_combinado() {
    let vec1:Vec<f64> = vec![1., 2., 3., 4., 5.];
    let vec2:Vec<f64> = vec![6., 7., 8., 9., 10.];
    let negocio = resistencia::f_resistencia_combinado(vec1, vec2);
    println!("{:?}", negocio);
    assert_eq!(55., negocio);
}

#[test]
fn name() {
    unimplemented!();
}
