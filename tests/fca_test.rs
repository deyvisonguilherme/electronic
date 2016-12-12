extern crate eletronic;
use eletronic::fca::condutancia::*;

#[test]
fn condutancia_test() {
    let x = CondutanciaEletrica::f_condutancia_eletrica(125.);
    let y = CondutanciaElemento::f_condutancia_elemento(125., 125.);

    assert_eq!(0.008, x);
    assert_eq!(250., y);
}