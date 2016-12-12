extern crate eletronic;
use eletronic::fca::*;
use eletronic::fca::joule::*;
// use eletronic::fca::potencia::*;

#[test]
fn test_condutancia_eletrica() {
    let negocio = condutancia::f_condutancia_eletrica(125.0); 
     assert_eq!( 0.008 , negocio );
}

#[test]
fn test_condutancia_elemento(){
    let negocio = condutancia::f_condutancia_elemento(125.0, 125.0);
    assert_eq!( 250.0, negocio );
}

#[test]
fn test_resistencia(){
    let negocio = condutancia::f_resistencia(125.0, 125.0);
    assert_eq!(24., negocio);
}

#[test]
fn test_resistencia_serie() {
    unimplemented!();
}

#[test]
fn test_resisencia_paralelo() {
    unimplemented!();
}

#[test]
fn test_resistencia_combinado() {
    unimplemented!();
}

#[test]
fn name() {
    unimplemented!();
}
