extern crate eletronic;
use eletronic::fca::condutancia::*;
use eletronic::fca::joule::*;
use eletronic::fca::potencia::*;
use eletronic::fca::resistencia::*;

#[test]
fn condutancia_test() {
    let x = CondutanciaEletrica::f_condutancia_eletrica(125.);
    let y = CondutanciaElemento::f_condutancia_elemento(125., 125.);

    assert_eq!(0.008, x);
    assert_eq!(250., y);
}

#[test]
fn joule_test(){
    let x = Joule::f_joule(2., 2., 3.);
    assert_eq!(24., x);
}

#[test]
fn potencia_test(){
    let x = Potencia::f_potencia(2., 3.);
    assert_eq!(0.6666666666666666, x);
}
