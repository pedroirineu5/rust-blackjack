use dialoguer::{theme::ColorfulTheme, Select};


#[derive(Debug)]
struct Carta{
    naipe: Naipe,
    numero: Numero,
}

#[derive(Debug)]
enum Naipe{
    Coracao,
    Paus,
    Espadas,
    Ouros,
}

#[derive(Debug)]
enum Numero{
    Dois,
    Tres,
    Quatro,
    Cinco,
    Seis,
    Sete,
    Oito,
    Nove,
    Valete,
    Rainha,
    Rei,
    As,
}

fn pegar_opcao_modo_de_jogo() -> usize {
    let selecao = &["Modo Solo", "2v2 (42)"];

    let selecao = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Escolha uma opção:")
        .default(0)
        .items(&selecao[..])
        .interact()
        .unwrap();

    selecao
}

fn pegar_opcao_players_humanos_adicionais() -> usize {
    let selecao = &["0","1", "2","3"];

    let selecao = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Escolha uma opção(SÃO JOGADORES EXTRAS PARA JOGAR A PARTIDA): ")
        .default(0)
        .items(&selecao[..])
        .interact()
        .unwrap();

    selecao
}

fn main(){
    pegar_opcao_players_humanos_adicionais();
    pegar_opcao_modo_de_jogo();


}