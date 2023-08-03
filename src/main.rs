fn main() {
    println!("Hello, world!");
    calcula_multiplicacao(10,10);
    expressao();
    let x = cinco();
    println!("o valor de x e: {x}");
}

fn calcula_multiplicacao(x: i32, y:i32) {
    let calculo = x * y;
    println!("O resultado da multiplicacao de {x} e {y} = {calculo}");
}

fn expressao() {
    let x = 5;
    let y = {  // inico da expressao, nao termina com ";", se adicionar ";" na expressao transforma ela em declaracao e nao retorna o nd
        let x = 3;
        x + 1
    };

    println!("o valor de y e {y}")
}

fn cinco() -> i32 { // retorna 5 com datatype i32
    5
}
