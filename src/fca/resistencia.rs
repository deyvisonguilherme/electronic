// Condutância elétrica, capacidade de uma corrente de percorrer um determinado condutor.
// R= representa a resistência de um elemento a passagem da corrente elétrica.
pub fn f_condutancia_eletrica(r: f64) -> f64 {
    1. / r
}

// Condutância de um elemento, avaliando as variaveis de tamanho, área transversal,
// e condutividade especificado do elemento.
pub fn f_condutancia_elemento(o: f64, a: f64, l: f64) -> f64 {
    o * (a / l)
}

// Definição de resistência de um circuito utilizando a lei de OHM
pub fn f_resistencia(u: f64, i: f64) -> f64 {
    u / i
}

// Somatória de resistores em série
pub fn f_resistencia_serie(v1: Vec<f64>) -> f64 {
    let mut result: f64 = 0.0;
    for x in 0..5 {
        result += v1[x];
    }
    result
}

// Somatória de resistores em paralelo
pub fn f_resisencia_paralelo(v1: Vec<f64>) -> f64 {
    let mut result: f64 = 0.0;
    for x in 0..5 {
        result += 1.0 / v1[x];
    }
    result
}

// Resistência de circuitos combinados
pub fn f_resistencia_combinado(v1: Vec<f64>, v2: Vec<f64>) -> f64 {
    let mut sum_v1: f64 = 0.0;
    let mut sum_v2: f64 = 0.0;

    for x in 0..5 {
        sum_v1 += v1[x];
    }

    for x in 0..5 {
        sum_v2 += v2[x];
    }

    sum_v1 + sum_v2
}