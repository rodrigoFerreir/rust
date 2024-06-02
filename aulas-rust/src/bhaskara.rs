

fn calculate_delta(a: i32, b: i32, c: i32) -> i32 {
    //! Calcula o valor de delta e retona o resultado.

    //! ::param a, b, c :i32.
    return (b * b) - 4 * a * c;
}
fn calculate_raiz(number: i32) -> i32 {
    let r = number / number;
    if r >= 0 {
        return r;
    } else {
        return 0;
    }
}

fn calculate_bhaskara(delta: i32, b: i32, a: i32) -> i32 {
    /*
    Calcula o valor de delta e retona o resultado
    ::param delta, b, c :i32
    */
    let raiz_delta: i32 = calculate_raiz(delta);
    let x1 = (-b + raiz_delta) / 2 * a;
    let x2 = (-b - raiz_delta) / 2 * a;

    if x1 >= 0 {
        println!("Retornou x1");
        return x1;
    } else if x2 >= 0 {
        println!("Retornou x2");
        return x2;
    } else {
        return 0;
    }
}

pub fn run() {
    println!("Hello, world!");
    let total = 100;
    println!("{}", total);
    //testes
    let a = 6;
    let b = -4;
    let c = 48;
    let delta = calculate_delta(a, b, c);
    println!(" O valor de delta é: {}", delta);

    let result = calculate_bhaskara(delta, b, a);
    println!(" o resultado da equação é: {}", result);
}
