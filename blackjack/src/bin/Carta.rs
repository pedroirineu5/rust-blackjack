use std::fmt;
use strum_macros::EnumIter;
use rand::seq::SliceRandom;
use rand::thread_rng;
use strum::IntoEnumIterator;

struct player{
    pontos: u8,
    mao: Vec<Carta>,
}

struct Carta{
    numero: Numero,
    naiper: Naipe,
}

enum Numero {
    As,
    Dois=2,
    Tres=3,
    Quatro=4,
    Cinco=5,
    Seis=6,
    Sete=7,
    Oito=8,
    Nove=9,
    Dez=10,
    Valete=10,
    Rainha=10,
    Rei=10,
}

enum Naipe{
    Copas,
    Ouros,
    Espadas,
    Paus,
}


