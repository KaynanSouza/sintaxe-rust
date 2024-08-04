// Define uma constante de ponto flutuante com precisão simples (f32).
const PI:f32 = 3.14;

// Define uma variável global mutável do tipo inteiro sem sinal de 8 bits (u8).
// Como é mutável, seu uso deve ser protegido com um bloco 'unsafe'.
static mut GLOBAL:u8 = 1;

// Função que soma dois inteiros e imprime o resultado.
fn soma(a: i32, b: i32) -> i32 {
    println!("{} + {} = {}", a, b, a + b);  // Exibe a soma dos números na tela.
    a + b  // Retorna a soma dos números.
}

// Função que demonstra o conceito de "shadowing" (sombreamento de variáveis).
fn sombra() {
    let a = 123;  // Declara uma variável `a` com valor 123.

    {
        let b = 456;  // Declara uma nova variável `b` dentro do bloco.
        println!("dentro, b = {}", b);  // Imprime `b`, que só existe neste bloco.

        let a = 777;  // "Sombreamento": declara uma nova `a`, que só existe neste bloco.
        println!("dentro, a = {}", a);  // Imprime a `a` interna, que tem valor 777.
    }

    // Fora do bloco interno, a `a` original (valor 123) é utilizada novamente.
    println!("fora, a = {}", a);
}

// Função que demonstra o escopo de variáveis, constantes, e uso de variáveis globais.
fn escopo() {
    println!("PI = {}", PI);  // Exibe a constante PI.

    unsafe {
        // Uso de uma variável global mutável, que requer um bloco 'unsafe'.
        println!("variavel_global = {}", GLOBAL);
    }

    let variavel:i32 = 300;  // Declara uma variável `variavel` com valor 300.
    println!("variavel = {}, tamanho = {} bytes", variavel, std::mem::size_of_val(&variavel));

    let variavel:i32 = 301;  // Sombreamento: nova `variavel` com valor 301.
    println!("variavel = {}, tamanho = {} bytes", variavel, std::mem::size_of_val(&variavel));

    let decimal:f32 = 2.5;  // Declara uma variável `decimal` do tipo f32.
    println!("decimal = {}", decimal);

    let booleana:bool = true;  // Declara uma variável booleana (true ou false).
    println!("Booleana = {}, Tamanho booleana = {}", booleana, std::mem::size_of_val(&booleana));

    let letra:char = 'C';  // Declara uma variável do tipo caractere (char).
    println!("Tamanho do char = {}", std::mem::size_of_val(&letra));
}

fn main() {
    escopo();  // Chama a função `escopo`.
    sombra();  // Chama a função `sombra`.

    println!("Soma = {}", soma(2, 2));  // Chama a função `soma` e exibe o resultado.

    // Chama as funções seguintes para demonstrar condicionais, loops, ownership e pattern matching.
    condicionais();
    repeticoes();
    ownership();
    pattern_matching();
    erros();
}

// Função que demonstra o uso de condicionais.
fn condicionais() {
    let idade: u8 = 18;  // Declara a idade.
    let responsavel_autorizou = true;  // Declara se o responsável autorizou.
    let eh_maior = idade >= 18;  // Verifica se a idade é maior ou igual a 18.

    if eh_maior {
        println!("Pode entrar na balada");  // Permite a entrada se for maior de idade.
    } else if idade > 16 && responsavel_autorizou {
        println!("Pode entrar com assinatura do responsável");  // Permite a entrada com autorização.
    } else {
        println!("Não pode entrar na balada");  // Nega a entrada.
    }

    // Uso de condicional em expressão: atribui "maior" ou "menor" à `condicao` baseado na idade.
    let condicao = if eh_maior { "maior" } else { "menor" };
    println!("É {} de idade", condicao);

    let linguagem = "C#";  // Define a linguagem de programação.

    // Uso de `match` para definir um propósito com base na linguagem.
    let proposito = match linguagem {
        "PHP" => "Web",
        "Kotlin" => "Android",
        "Python" => "Data Science",
        _ => "Desconhecido"  // O '_' é um coringa para todos os outros valores.
    };

    println!("O proposito de {} eh {}", linguagem, proposito);
}

// Função que demonstra diferentes tipos de loops.
fn repeticoes() {
    let multiplicador:u8 = 5;  // Define um multiplicador.

    let mut contador:u8 = 0;
    // Loop `while` que continua até `contador` ser 10.
    while contador < 10 {
        contador += 1;

        if contador == 5 {
            continue;  // Pula o restante do loop se `contador` for 5.
        }

        println!("{} x {} = {}", multiplicador, contador, multiplicador * contador);
    }

    contador = 0;
    // Loop `loop` infinito até encontrar um `break`.
    loop {
        contador += 1;
        println!("{} x {} = {}", multiplicador, contador, multiplicador * contador);

        if contador == 10 {
            break;  // Sai do loop quando `contador` for 10.
        }
    }

    // Loop `for` que vai de 1 a 10.
    for i in 1..11 {
        println!("{} x {} = {}", multiplicador, i, multiplicador * i);
    }
}

// Função que demonstra o conceito de ownership e borrowing.
fn ownership() {
    let mut uma_string = String::from("Vinicius");  // Cria uma string mutável.
    rouba(&mut uma_string);  // Passa a string como referência mutável para a função `rouba`.

    println!("{}", uma_string);  // Exibe a string após a função `rouba` modificá-la.
}

// Função que altera uma string mutável.
fn rouba(string: &mut String) {
    string.push_str(" Dias");  // Adiciona " Dias" ao final da string.
    println!("{}", string);  // Exibe a string modificada.
}

// Função que demonstra o uso de pattern matching.
fn pattern_matching() {
    for x in 1..=20 {  // Loop de 1 a 20.
        println!("{}: {}", x, match x {
            1 => "Pouco",  // Para 1, exibe "Pouco".
            2 | 3 => "Um pouquinho",  // Para 2 ou 3, exibe "Um pouquinho".
            4..=10 => "Um bocado",  // Para 4 até 10, exibe "Um bocado".
            _ if x % 2 == 0 => "Uma boa quantidade",  // Para números pares fora dos casos acima, exibe "Uma boa quantidade".
            _ => "Muito"  // Para todos os outros casos, exibe "Muito".
        });
    }
}

// Função que demonstra tratamento de erros com Result.
fn erros() {
    match resultado() {
        Ok(s) => println!("String de sucesso = {}", s),  // Se `resultado` retornar Ok, exibe a string.
        Err(numero) => println!("Codigo de erro = {}", numero)  // Se `resultado` retornar Err, exibe o número de erro.
    };
}

// Função que retorna um Result, simulando uma operação que pode falhar.
fn resultado() -> Result<String, u8> {
    Err(42)  // Sempre retorna um erro com código 42.
}

