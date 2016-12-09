/*
.-----------------------------------------------------------.
|        Descrição das abreviações das bibliotecas          |
|            FCC => Fórmulas de Corrente Contínua           |
|            FCA => Fórmulas de Corrente Alternada          |
|         FCE => Fórmulas Para Circuitos Eletrônicos        |
|                   TRA =>Transistores                      |
|   GCT => Grandezas Básicas de Circuitos com Transistores  |
|           FPT =>Fórmulas Práticas Para Transistores       |
|                     OSC => Osciladores                    |
+-----------------------------------------------------------+
*/
pub mod fcc;
pub mod fca;
pub mod fce;
pub mod tra;
pub mod gct;
pub mod fpt;
pub mod osc;
#[test]
fn condutancia_test() {
    use fca::condutancia;
    use fca::condutancia::FormulaCondutanciaEletrica;
    use fca::condutancia::FormulaCondutanciaElemento;

    let x = condutancia::CondutanciaEletrica { r: 125. };
    let y = condutancia::CondutanciaElemento { r: 125., a: 125. };

    assert_eq!(0.008, x.f_condutancia_eletrica());
    assert_eq!(250., y.f_condutancia_elemento());
}