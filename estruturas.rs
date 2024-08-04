fn main() {
    // Define um array de 4 elementos, todos inicializados com 6.5.
    let notas: [f32; 4] = [6.5; 4];
    let inteiro: usize = 0;  // Índice do tipo usize.

    // Imprime o primeiro elemento do array (notas[0]).
    println!("{}", notas[inteiro]);

    // Loop que percorre o array e imprime cada nota com seu índice.
    for indice in 0..notas.len() {
        println!("A nota {} é = {}", indice + 1, notas[indice]);
    }

    // Chama várias funções para demonstrar diferentes conceitos.
    matriz();  // Demonstra a manipulação de uma matriz (array 2D).
    println!("É fim de semana? {}", eh_fim_de_semana(DiaDaSemana::Quarta));  // Verifica se um dia é fim de semana.

    cores();  // Demonstra o uso de enums com valores associados.
    conteudo_opcional();  // Demonstra o uso de Option.
    vectors();  // Demonstra a manipulação de vetores (arrays dinâmicos).

    conta_corrente();  // Demonstra o uso de structs e métodos associados.
}

// Enum que representa os dias da semana.
#[allow(dead_code)]
enum DiaDaSemana {
    Domingo,
    Segunda,
    Terca,
    Quarta,
    Quinta,
    Sexta,
    Sabado
}

// Função que verifica se o dia passado como parâmetro é fim de semana.
fn eh_fim_de_semana(dia_da_semana: DiaDaSemana) -> bool {
    match dia_da_semana {
        DiaDaSemana::Domingo | DiaDaSemana::Sabado => true,  // Retorna true se for sábado ou domingo.
        _ => false  // Para qualquer outro dia, retorna false.
    }
}

// Função que demonstra o uso de uma matriz (array 2D).
fn matriz() {
    // Define uma matriz 2x3 de números de ponto flutuante.
    let matriz: [[f32; 3]; 2] = [
        [0.0, 1.2, 0.1],
        [1.3, 0.3, 1.4]
    ];

    // Loop aninhado que percorre e imprime cada elemento da matriz.
    for linha in matriz {
        for item in linha {
            println!("Item = {}", item);
        }
    }
}

// Enum que representa cores, com opções para RGB e CMYK.
#[allow(dead_code)]
enum Color {
    Red,
    Green,
    Blue,
    RgbColor(u8, u8, u8),  // Cor RGB com 3 valores (vermelho, verde, azul).
    CymkColor{cyan: u8, magenta: u8, yellow: u8, black: u8}  // Cor CMYK com valores nomeados.
}

// Função que demonstra o uso de enums com valores associados.
fn cores() {
    let cor = Color::RgbColor(12, 5, 32);  // Define uma cor RGB.

    // Match que avalia a cor e imprime uma string correspondente.
    println!("Cor = {}", match cor {
        Color::Red => "vermelho",
        Color::Green => "verde",
        Color::Blue => "blue",
        Color::RgbColor(0, 0, 0)
            | Color::CymkColor{cyan: _, magenta: _, yellow: _, black: 255} => "preta",  // Cor preta em RGB ou CMYK.
        Color::RgbColor(_, green, _) => {
            println!("{}", green);  // Imprime o valor do canal verde.
            "RGB desconhecido"
        }
        Color::CymkColor{cyan: _, magenta: _, yellow: _, black: _} => "CYMK desconhecido"  // Cor CMYK desconhecida.
    });
}

// Função que demonstra o uso de Option (conteúdo opcional).
fn conteudo_opcional() {
    let conteudo_arquivo = ler_arquivo(String::from(""));  // Simula a leitura de um arquivo.

    // Match que verifica se o arquivo foi lido com sucesso.
    match &conteudo_arquivo {
        Some(valor) => println!("{}", valor),  // Se o arquivo existe, imprime seu conteúdo.
        None => println!("Arquivo nao existe")  // Se não existe, informa que o arquivo não foi encontrado.
    };

    println!("{:?}", conteudo_arquivo);  // Imprime o valor de `conteudo_arquivo`.

    // Uso de `if let` para trabalhar com o valor dentro de Some, se existir.
    if let Some(valor) = conteudo_arquivo {
        println!("Agora, tenho certeza de ha o valor {}", valor);
    }
}

// Função que simula a leitura de um arquivo e retorna um Option<String>.
#[allow(unused_variables)]
fn ler_arquivo(caminho_arquivo: String) -> Option<String> {
    Some(String::from("Algum conteudo"))  // Sempre retorna Some com uma string.
}

// Função que demonstra o uso de vetores (arrays dinâmicos).
fn vectors() {
    let mut notas: Vec<f32> = Vec::with_capacity(4);  // Cria um vetor com capacidade inicial de 4 elementos.
    notas.push(10.0);  // Adiciona elementos ao vetor.
    notas.push(8.0);
    notas.push(6.5);
    println!("Capacidade = {}", notas.capacity());  // Imprime a capacidade atual do vetor.

    println!("{:?}", notas);  // Imprime o vetor.

    notas.push(6.8);  // Adiciona mais um elemento, possivelmente expandindo a capacidade.
    println!("Capacidade = {}", notas.capacity());  // Imprime a nova capacidade do vetor.
    println!("{:?}", notas);

    println!("Nota 1 = {}", notas[0]);  // Acessa o primeiro elemento do vetor.

    // Uso de `get` para acessar um elemento que pode não existir.
    println!("Nota 6 = {}", match notas.get(7) {
        Some(n) => *n,  // Se o índice for válido, retorna o valor.
        None => 0.0  // Se o índice for inválido, retorna 0.0.
    });

    /*
    // Loop que remove e imprime cada elemento do vetor até que ele esteja vazio.
    while let Some(nota) = notas.pop() {
        println!("Valor removido = {}", nota);
    }
    */

    // Itera sobre o vetor, imprimindo cada nota.
    for nota in &notas {
        println!("Nota = {}", nota);
    }
    println!("{:?}", notas);  // Imprime o vetor após a iteração.
}

// Struct que representa uma conta bancária.
struct Conta {
    titular: Titular,  // A conta tem um titular.
    saldo: f64  // E um saldo.
}

// Implementação de métodos para a struct `Conta`.
impl Conta {
    // Método para sacar dinheiro da conta.
    fn sacar(&mut self, valor: f64) {
        self.saldo -= valor;  // Subtrai o valor do saldo.
    }
}

// Struct que representa um titular de conta bancária.
struct Titular {
    nome: String,
    sobrenome: String
}

// Função que demonstra a criação e manipulação de uma conta bancária.
fn conta_corrente() {
    // Cria um titular.
    let titular = Titular{nome: String::from("Vinicius"), sobrenome: String::from("Dias")};
    let mut conta: Conta = Conta{
        titular,  // Associa o titular à conta.
        saldo: 100.0  // Define o saldo inicial.
    };

    conta.sacar(50.0);  // Realiza um saque de 50.0.

    // Imprime os dados da conta após o saque.
    println!(
        "Dados da conta: Titular = {} {}, Saldo = {}",
        conta.titular.nome,
        conta.titular.sobrenome,
        conta.saldo
    );
}

