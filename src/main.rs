mod warmup;

use std::io;

fn main() {
    println!("=== ПРОГРАММИРОВАНИЕ: ВЫПОЛНЕНИЕ ЗАДАНИЙ ===");
    println!("Выберите категорию заданий:");
    println!("1. Линейные алгоритмы");
    println!("2. Разветвляющиеся алгоритмы");
    println!("3. Циклические алгоритмы");
    println!("0. Выход");

    loop {
        println!("\nВведите номер категории (0-3):");

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Ошибка чтения ввода");

        match choice.trim() {
            "0" => {
                println!("До свидания!");
                break;
            }
            "1" => run_linear_tasks(),
            "2" => run_branching_tasks(),
            "3" => run_cyclic_tasks(),
            _ => println!("Неверный выбор. Попробуйте снова."),
        }
    }
}

fn run_linear_tasks() {
    println!("\n=== ЛИНЕЙНЫЕ АЛГОРИТМЫ ===");
    println!("Доступные задачи: 1-10");
    println!("Введите номер задачи или 0 для возврата:");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Ошибка чтения ввода");

    match input.trim() {
        "0" => return,
        "1" => warmup::linear::task1(),
        "2" => warmup::linear::task2(),
        "3" => warmup::linear::task3(),
        "4" => warmup::linear::task4(),
        "5" => warmup::linear::task5(),
        "6" => warmup::linear::task6(),
        "7" => warmup::linear::task7(),
        "8" => warmup::linear::task8(),
        "9" => warmup::linear::task9(),
        "10" => warmup::linear::task10(),
        _ => println!("Задача с таким номером не реализована"),
    }
}

fn run_branching_tasks() {
    println!("\n=== РАЗВЕТВЛЯЮЩИЕСЯ АЛГОРИТМЫ ===");
    println!("Доступные задачи: 1-10");
    println!("Введите номер задачи или 0 для возврата:");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Ошибка чтения ввода");

    match input.trim() {
        "0" => return,
        "1" => warmup::branching::task1(),
        "2" => warmup::branching::task2(),
        "3" => warmup::branching::task3(),
        "4" => warmup::branching::task4(),
        "5" => warmup::branching::task5(),
        "6" => warmup::branching::task6(),
        "7" => warmup::branching::task7(),
        "8" => warmup::branching::task8(),
        "9" => warmup::branching::task9(),
        "10" => warmup::branching::task10(),
        _ => println!("Задача с таким номером не реализована"),
    }
}

fn run_cyclic_tasks() {
    println!("\n=== ЦИКЛИЧЕСКИЕ АЛГОРИТМЫ ===");
    println!("Доступные задачи: 1-9");
    println!("Введите номер задачи (1-9) или 0 для возврата:");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Ошибка чтения ввода");

    match input.trim() {
        "0" => return,
        "1" => warmup::cyclic::task1(),
        "2" => warmup::cyclic::task2(),
        "3" => warmup::cyclic::task3(),
        "4" => warmup::cyclic::task4(),
        "5" => warmup::cyclic::task5(),
        "6" => warmup::cyclic::task6(),
        "7" => warmup::cyclic::task7(),
        "8" => warmup::cyclic::task8(),
        "9" => warmup::cyclic::task9(),
        _ => println!("Задача с таким номером не реализована"),
    }
}
