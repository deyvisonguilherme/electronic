// Condutância elétrica, capacidade de uma corrente de percorrer um determinado condutor.
// R= representa a resistência de um elemento a passagem da corrente elétrica.
pub fn f_condutancia_eletrica(r: f64) -> f64 { 1. / r }

// Condutância de um elemento, avaliando as variaveis de tamanho, área transversal,
// e condutividade especificado do elemento.
pub fn f_condutancia_elemento(o: f64, a: f64, l: f64) -> f64 { o * ( a / l) }

// Definição de resistência de um circuito utilizando a lei de OHM
pub fn f_resistencia(u: f64, i: f64) -> f64 { u / i }

// Somatória de resistores em série
pub fn f_resistencia_serie(r1: f64, r2: f64, r3: f64, r4: f64) -> f64 { r1 + r2 + r3 + r4 }

// Somatória de resistores em paralelo
// pub fn f_resisencia_paralelo(r1 :f64, r2: f64, r3: f64, r4: f64) -> f64 {
//     1 / (( 1 / r1) + (1 / r2) + (1 / r3) + (1 / r4) )
// }

// Resistência de circuitos combinados
pub fn f_resistencia_combinado(v1: Vec<f64>, v2: Vec<f64>) -> f64 {
    let mut sum_v1: f64 = 0.0;
    let mut sum_v2: f64 = 0.0;

    for x in 0..10{
        sum_v1 += v1[x];
    }

    for x in 0..10{
        sum_v2 += v2[x];
    }

    sum_v1 + sum_v2
}