use dialoguer::{theme::ColorfulTheme, Select};
use rand::Rng;
use rand::seq::IteratorRandom;
use std::cmp::Ordering;
use std::fmt;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

fn get_select_option_cards() -> usize {
    let selections = &["Pegar Cartas", "Sair"];

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Escolha uma opção:")
        .default(0)
        .items(&selections[..])
        .interact()
        .unwrap();

    selection
}

fn get_select_option_game_mode() -> usize {
    let selections = &["Modo Solo", "Modo 2v2(42)"];

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("ESCOLHA O MODO DE JOGO:")
        .default(0)
        .items(&selections[..])
        .interact()
        .unwrap();

    selection
}

fn get_select_option_maquinas() -> usize {
    let selections = &["1(Padrão)", "2","3"];

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Quantas maquinas:")
        .default(0)
        .items(&selections[..])
        .interact()
        .unwrap();

    selection
}

fn next_move(option: usize, cards: &mut PlayerCardList) {
    match option {
        0 => {
            cards.draw_card();
        }
        1 => {
            println!("Vez do dealer.\n");
        }
        _ => println!("Escolha não válida"),
    }
}

struct PlayerCardList(Vec<Cards>);

#[derive(EnumIter, Debug, PartialEq)]
enum Cards {
    TWO,
    THREE,
    FOUR,
    FIVE,
    SIX,
    SEVEN,
    EIGHT,
    NINE,
    TEN,
    JACK,
    QUEEN,
    KING,
    ACE,
}

#[derive(EnumIter, Debug, PartialEq)]
enum Players {
    YOU,
    MAQUINA,
}

impl fmt::Display for Players {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Cards {
    fn value(&self) -> i32 {
        match *self {
            Cards::TWO => 2,
            Cards::THREE => 3,
            Cards::FOUR => 4,
            Cards::FIVE => 5,
            Cards::SIX => 6,
            Cards::SEVEN => 7,
            Cards::EIGHT => 8,
            Cards::NINE => 9,
            Cards::TEN => 10,
            Cards::JACK => 10,
            Cards::KING => 10,
            Cards::QUEEN => 10,
            Cards::ACE => 11,
        }
    }
}

impl fmt::Display for Cards {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl<'a> PartialEq<&'a Cards> for Cards {
    fn eq(&self, other: &&'a Cards) -> bool {
        self == *other
    }
}

impl<'a> PartialEq<Cards> for &'a Cards {
    fn eq(&self, other: &Cards) -> bool {
        *self == other
    }
}

impl PlayerCardList {
    fn draw_card(&mut self) {
        let mut rng = rand::thread_rng();
        let card = Cards::iter().choose(&mut rng).unwrap();
        self.0.push(card);
    }

    fn print_first_card(&mut self) {
        let first_card = &self.0[0];
        println!(" cards:\n");
        println!("Card: {} -> Value: {}", first_card, first_card.value());
        println!("Card: {} -> Value: {}\n", "???", "???");
    }

    fn print_cards(&mut self, player: Players) {
        println!("Cards from {}:\n", player);
        for card in &self.0 {
            println!("Card: {} -> Value: {}", card, card.value());
        }
        println!();
    }// todo ajeitar a função para o modo 1v1v1, e cosnquentemente para o modo 1v1v1v1 e tbm 2v2.

    fn get_sum(&mut self) -> i32 {
        self.0.iter().map(|x| x.value()).sum()
    }
}

fn get_winner(player: i32, comparado2: i32) -> &'static str {
    if player > 21 && comparado2 > 21{
        return "Ninguém";
    }
    if player <= 21 {
        if comparado2 > 21 {
            return "VOCÊ";
        }
        match player.cmp(&comparado2) {
            Ordering::Less => "Máquina 1",
            Ordering::Greater => "You",
            Ordering::Equal => "Máquina 1",
        }
    } else {
        return "Máquina 1";
    }
}

fn get_winner_2_adicionais(player: i32, comparado2: i32, comparado3: i32) -> &'static str {
    if player > 21 && comparado2 > 21 && comparado3 > 21 {
        return "Ninguém";
    } else if player <= 21 {
        if comparado2 > 21 && comparado3 > 21 {
            return "VOCÊ";
        } else if comparado2 <= 21 && comparado3 <= 21 {
            if player > comparado2 && player > comparado3 {
                return "VOCÊ";
            } else if comparado2 > player && comparado2 > comparado3 {
                return "MAQUINA 1";
            } else if comparado3 > player && comparado3 > comparado2 {
                return "MAQUINA 2";
            }
        } else if comparado2 <= 21 {
            if player > 21 && comparado3 > 21 {
                return "MAQUINA 1";
            } else if comparado2 > player && comparado2 > comparado3 {
                return "MAQUINA 1";
            }
        } else if comparado3 <= 21 {
            if player > 21 && comparado2 > 21 {
                return "MAQUINA 2";
            } else if comparado3 > player && comparado3 > comparado2 {
                return "MAQUINA 2";
            }
        }
    }
    return " "; 
}

fn get_winner_3_adicionais(player: i32, comparado2: i32, comparado3: i32, comparado4: i32) -> &'static str {
    if player > 21 && comparado2 > 21 && comparado3 > 21 && comparado4 > 21 {
        return "Ninguém";
    } else if player <= 21 {
        if comparado2 > 21 && comparado3 > 21 && comparado4 > 21 {
            return "VOCÊ";
        } else if comparado2 <= 21 && comparado3 <= 21 && comparado4 <= 21 {
            if player > comparado2 && player > comparado3 && player > comparado4 {
                return "VOCÊ";
            } else if comparado2 > player && comparado2 > comparado3 && comparado2 > comparado4 {
                return "MAQUINA 1";
            } else if comparado3 > player && comparado3 > comparado2 && comparado3 > comparado4 {
                return "MAQUINA 2";
            } else if comparado4 > player && comparado4 > comparado2 && comparado4 > comparado3 {
                return "MAQUINA 3";
            }
        } else if comparado2 <= 21 {
            if player > 21 && comparado3 > 21 && comparado4 > 21 {
                return "MAQUINA 1";
            } else if comparado2 > player && comparado2 > comparado3 && comparado2 > comparado4 {
                return "MAQUINA 1";
            }
        } else if comparado3 <= 21 {
            if player > 21 && comparado2 > 21 && comparado4 > 21 {
                return "MAQUINA 2";
            } else if comparado3 > player && comparado3 > comparado2 && comparado3 > comparado4 {
                return "MAQUINA 2";
            }
        } else if comparado4 <= 21 {
            if player > 21 && comparado2 > 21 && comparado3 > 21 {
                return "MAQUINA 3";
            } else if comparado4 > player && comparado4 > comparado2 && comparado4 > comparado3 {
                return "MAQUINA 3";
            }
        }
    }
    return " "; // Trate este caso vazio de acordo com a lógica do seu programa
}


fn jogo_normal(){
    let mut user_cards = PlayerCardList(vec![]);
    let mut dealer_cards = PlayerCardList(vec![]);
    // user_cards.0.push(Cards::ACE);
    // user_cards.0.push(Cards::ACE);
    user_cards.draw_card();
    user_cards.draw_card();
    dealer_cards.draw_card();
    dealer_cards.draw_card();

    // Dealer
    dealer_cards.print_first_card();
    dealer_cards.print_first_card();
    loop{
        let mut rng = rand::thread_rng();
        let number = rng.gen_range(1..=100);

        if dealer_cards.get_sum() <= 11 {
            dealer_cards.draw_card();
            break;
        } 
        else if dealer_cards.get_sum() > 16 {
            dealer_cards.draw_card();
            break;
        } 
        else if dealer_cards.get_sum() == 12 {
            if number > 31 {
                dealer_cards.draw_card();
                break;
            } else {
                break;
            }
        } else if dealer_cards.get_sum() == 13 {
            if number > 39 {
                dealer_cards.draw_card();
                break;
            } else {
                break;
            }
        } else if dealer_cards.get_sum() == 14 {
            if number > 56 {
                dealer_cards.draw_card();
                break;
            } else {
                break;
            }
        } else if dealer_cards.get_sum() == 15 {
            if number > 58 {
                dealer_cards.draw_card();
                break;
            } else {
                break;
            }
        } else if dealer_cards.get_sum() == 16 {
            if number > 62 {
                dealer_cards.draw_card();
                break;
            } else {
                break;
            }
        }
    }

    // Player
    loop {
        user_cards.print_cards(Players::YOU);
        let sum = user_cards.get_sum();
        println!("SUM: {sum}\n");
        if sum >= 21 {
            break;
        }
        let opt = get_select_option_cards();
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        if opt == 1 && sum < 17 {
            println!("You need at least a score of 17 to proceed.\nYour current score is {sum}");
            continue;
        } else if opt == 1 {
            break;
        }
        next_move(opt, &mut user_cards);
    }

    dealer_cards.print_cards(Players::MAQUINA);
    println!("SUM DEALER: {}\n", dealer_cards.get_sum());

    user_cards.print_cards(Players::YOU);
    println!("SUM PLAYER: {}\n", user_cards.get_sum());

    let winner = get_winner(user_cards.get_sum(), dealer_cards.get_sum());
    print!("{winner} won the game!");
}


fn jogo_normal_2_maquinas(){
    let mut user_cards = PlayerCardList(vec![]);
    let mut dealer_cards = PlayerCardList(vec![]);
    let mut dealer2_cards = PlayerCardList(vec![]);
    // user_cards.0.push(Cards::ACE);
    // user_cards.0.push(Cards::ACE);
    user_cards.draw_card();
    user_cards.draw_card();
    dealer_cards.draw_card();
    dealer_cards.draw_card();
    dealer2_cards.draw_card();
    dealer2_cards.draw_card();

    // MAQUINA1
    dealer_cards.print_first_card();
    //CÓDIGO DE IA
    loop{
        let mut rng = rand::thread_rng();
        let number = rng.gen_range(1..=100);

        if dealer_cards.get_sum() <= 11 {
            dealer_cards.draw_card();
            break;
        } 
        else if dealer_cards.get_sum() > 16 {
            dealer_cards.draw_card();
            break;
        } 
        else if dealer_cards.get_sum() == 12 {
            if number > 31 {
                dealer_cards.draw_card();
                break;
            } else {
                break;
            }
        } else if dealer_cards.get_sum() == 13 {
            if number > 39 {
                dealer_cards.draw_card();
                break;
            } else {
                break;
            }
        } else if dealer_cards.get_sum() == 14 {
            if number > 56 {
                dealer_cards.draw_card();
                break;
            } else {
                break;
            }
        } else if dealer_cards.get_sum() == 15 {
            if number > 58 {
                dealer_cards.draw_card();
                break;
            } else {
                break;
            }
        } else if dealer_cards.get_sum() == 16 {
            if number > 62 {
                dealer_cards.draw_card();
                break;
            } else {
                break;
            }
        }
    }
    //MUDAR A IA COM O CÓGIDO DE DUDU
    // MAQUINA2

    dealer2_cards.print_first_card();
    //CÓDIGO DE IA
    loop{
        let mut rng = rand::thread_rng();
        let number = rng.gen_range(1..=100);

        if dealer2_cards.get_sum() <= 11 {
            dealer2_cards.draw_card();
            break;
        } 
        else if dealer2_cards.get_sum() > 16 {
            dealer2_cards.draw_card();
            break;
        } 
        else if dealer2_cards.get_sum() == 12 {
            if number > 31 {
                dealer2_cards.draw_card();
                break;
            } else {
                break;
            }
        } else if dealer2_cards.get_sum() == 13 {
            if number > 39 {
                dealer2_cards.draw_card();
                break;
            } else {
                break;
            }
        } else if dealer2_cards.get_sum() == 14 {
            if number > 56 {
                dealer2_cards.draw_card();
                break;
            } else {
                break;
            }
        } else if dealer2_cards.get_sum() == 15 {
            if number > 58 {
                dealer2_cards.draw_card();
                break;
            } else {
                break;
            }
        } else if dealer2_cards.get_sum() == 16 {
            if number > 62 {
                dealer2_cards.draw_card();
                break;
            } else {
                break;
            }
        }
    }

    // JOGADOR

    // AQUI É ONDE O JOGADOR REALIZA AS SUAS AÇÕES COM BASE NA SUA MÃO.
    loop {
        user_cards.print_cards(Players::YOU);
        let sum = user_cards.get_sum();
        println!("Soma: {sum}\n");
        if sum >= 21 {
            break;
        }
        let opt = get_select_option_cards();
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        if opt == 1 && sum < 17 {
            println!("Você precisa de um score de 16 ou menos para pegar a carta.\nSeus pontos atuais são:{sum}");
            continue;
        } else if opt == 1 {
            break;
        }
        next_move(opt, &mut user_cards);
    }

    dealer_cards.print_cards(Players::MAQUINA);
    println!("SOMA MAQUINA: {}\n", dealer_cards.get_sum());

    dealer2_cards.print_cards(Players::MAQUINA);
    println!("SOMA MAQUINA2: {}\n", dealer2_cards.get_sum());

    user_cards.print_cards(Players::YOU);
    println!("SOMA JOGADOR: {}\n", user_cards.get_sum());

    let winner = get_winner_2_adicionais(user_cards.get_sum(), dealer_cards.get_sum(),dealer2_cards.get_sum());
    print!("{winner} venceu o jogo!");
}

// modo de jogo DE PLAYER CONTRA MAQUINA1,2 E 3 
fn jogo_normal_3_maquinas(){
    let mut user_cards = PlayerCardList(vec![]);
    let mut dealer_cards = PlayerCardList(vec![]);
    let mut dealer2_cards = PlayerCardList(vec![]);
    let mut dealer3_cards = PlayerCardList(vec![]);
    // user_cards.0.push(Cards::ACE);
    // user_cards.0.push(Cards::ACE);
    user_cards.draw_card();
    user_cards.draw_card();

    dealer_cards.draw_card();
    dealer_cards.draw_card();

    dealer2_cards.draw_card();
    dealer2_cards.draw_card();

    dealer3_cards.draw_card();
    dealer3_cards.draw_card();

    // MAQUINA1
    dealer_cards.print_first_card();
    loop{
        let mut rng = rand::thread_rng();
        let number = rng.gen_range(1..=100);

        if dealer_cards.get_sum() <= 11 {
            dealer_cards.draw_card();
            break;
        } 
        else if dealer_cards.get_sum() > 16 {
            dealer_cards.draw_card();
            break;
        } 
        else if dealer_cards.get_sum() == 12 {
            if number > 31 {
                dealer_cards.draw_card();
                break;
            } else {
                break;
            }
        } else if dealer_cards.get_sum() == 13 {
            if number > 39 {
                dealer_cards.draw_card();
                break;
            } else {
                break;
            }
        } else if dealer_cards.get_sum() == 14 {
            if number > 56 {
                dealer_cards.draw_card();
                break;
            } else {
                break;
            }
        } else if dealer_cards.get_sum() == 15 {
            if number > 58 {
                dealer_cards.draw_card();
                break;
            } else {
                break;
            }
        } else if dealer_cards.get_sum() == 16 {
            if number > 62 {
                dealer_cards.draw_card();
                break;
            } else {
                break;
            }
        }
    }
    //MUDAR A IA COM O CÓGIDO DE DUDU
    // MAQUINA2
    dealer2_cards.print_first_card();
    loop{
        let mut rng = rand::thread_rng();
        let number = rng.gen_range(1..=100);

        if dealer2_cards.get_sum() <= 11 {
            dealer2_cards.draw_card();
            break;
        } 
        else if dealer2_cards.get_sum() > 16 {
            dealer2_cards.draw_card();
            break;
        } 
        else if dealer2_cards.get_sum() == 12 {
            if number > 31 {
                dealer2_cards.draw_card();
                break;
            } else {
                break;
            }
        } else if dealer2_cards.get_sum() == 13 {
            if number > 39 {
                dealer2_cards.draw_card();
                break;
            } else {
                break;
            }
        } else if dealer2_cards.get_sum() == 14 {
            if number > 56 {
                dealer2_cards.draw_card();
                break;
            } else {
                break;
            }
        } else if dealer2_cards.get_sum() == 15 {
            if number > 58 {
                dealer2_cards.draw_card();
                break;
            } else {
                break;
            }
        } else if dealer2_cards.get_sum() == 16 {
            if number > 62 {
                dealer2_cards.draw_card();
                break;
            } else {
                break;
            }
        }
    }
    //IA DA MAQUINA 3
    dealer3_cards.print_first_card();
    loop{
        let mut rng = rand::thread_rng();
        let number = rng.gen_range(1..=100);

        if dealer3_cards.get_sum() <= 11 {
            dealer3_cards.draw_card();
            break;
        } 
        else if dealer3_cards.get_sum() > 16 {
            dealer3_cards.draw_card();
            break;
        } 
        else if dealer3_cards.get_sum() == 12 {
            if number > 31 {
                dealer3_cards.draw_card();
                break;
            } else {
                break;
            }
        } else if dealer3_cards.get_sum() == 13 {
            if number > 39 {
                dealer3_cards.draw_card();
                break;
            } else {
                break;
            }
        } else if dealer3_cards.get_sum() == 14 {
            if number > 56 {
                dealer3_cards.draw_card();
                break;
            } else {
                break;
            }
        } else if dealer3_cards.get_sum() == 15 {
            if number > 58 {
                dealer3_cards.draw_card();
                break;
            } else {
                break;
            }
        } else if dealer3_cards.get_sum() == 16 {
            if number > 62 {
                dealer3_cards.draw_card();
                break;
            } else {
                break;
            }
        }
    }
    // JOGADOR
    loop {
        user_cards.print_cards(Players::YOU);
        let sum = user_cards.get_sum();
        println!("Soma: {sum}\n");
        if sum >= 21 {
            break;
        }
        let opt = get_select_option_cards();
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        if opt == 1 && sum < 17 {
            println!("Você precisa de um score de 16 ou menos para pegar a carta.\nSeus pontos atuais são:{sum}");
            continue;
        } else if opt == 1 {
            break;
        }
        next_move(opt, &mut user_cards);
    }

    dealer_cards.print_cards(Players::MAQUINA);
    println!("SOMA MAQUINA 1: {}\n", dealer_cards.get_sum());

    dealer2_cards.print_cards(Players::MAQUINA);
    println!("SOMA MAQUINA 2: {}\n", dealer2_cards.get_sum());
    
    dealer3_cards.print_cards(Players::MAQUINA);
    println!("SOMA MAQUINA 3: {}\n", dealer3_cards.get_sum());

    user_cards.print_cards(Players::YOU);
    println!("SOMA JOGADOR: {}\n", user_cards.get_sum());

    let winner = get_winner_3_adicionais(user_cards.get_sum(), dealer_cards.get_sum(),dealer2_cards.get_sum(),dealer3_cards.get_sum());
    
    print!("{winner} venceu o jogo!");
}



#[allow(dead_code)]
fn jogo_2v2(){
    print!("acabando")
}


fn main() {

    let option = get_select_option_game_mode(); //pega o modo de jogo na funcao de selecao
    if option == 0{
        let machines = get_select_option_maquinas();

            if machines == 0{
                jogo_normal();// a primeira opção ela é 1v1, as outras seria 1v1v1, e 1v1v1v1.
            }
            else if machines == 1{
                jogo_normal_2_maquinas(); // a segunda opção é 1v1v1
            }
            else if machines == 2{
                jogo_normal_3_maquinas(); //a terceira opção é 1v1v1v1
            }
        } 
    //else if == 1{
    //     jogo_2v2();//mexeram com
    // }

}
