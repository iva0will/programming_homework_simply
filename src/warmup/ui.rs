//! Модуль для отображения пользовательского интерфейса в ASCII

/// Рисует горизонтальную линию заданной длины
pub fn horizontal_line(length: usize, char: &str) -> String {
    char.repeat(length)
}

/// Рисует рамку вокруг текста
pub fn framed_box(title: &str, content: &str) {
    let width = 60;
    let top_border = format!("┌{}┐", "─".repeat(width - 2));
    let bottom_border = format!("└{}┘", "─".repeat(width - 2));
    
    println!("\n{}", top_border);
    println!("│{: ^width$}│", title);
    println!("│{}│", "─".repeat(width - 2));
    
    // Разбиваем контент на строки
    for line in wrap_text(content, width - 4) {
        println!("│ {: <width$} │", line);
    }
    
    println!("{}", bottom_border);
}

/// Рисует меню с пунктами
pub fn menu(title: &str, items: &[(&str, &str)]) {
    let width = 50;
    println!("\n╔{}╗", "═".repeat(width - 2));
    println!("║{: ^width$}║", title);
    println!("╠{}╣", "═".repeat(width - 2));
    
    for (i, (num, desc)) in items.iter().enumerate() {
        println!("║ {:2}. {:<width$} ║", num, desc, width = width - 7);
    }
    
    println!("╚{}╝", "═".repeat(width - 2));
}

/// Рисует разделитель
pub fn separator() {
    println!("{}", "─".repeat(60));
}

/// Рисует заголовок задачи
pub fn task_header(number: &str, title: &str) {
    println!("\n┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓");
    println!("┃ {:^55} ┃", format!("ЗАДАЧА {}: {}", number, title));
    println!("┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛");
}

/// Рисует результат вычислений в красивом формате
pub fn result_box(title: &str, result: &str) {
    let width = 50;
    println!("\n╔{}╗", "═".repeat(width - 2));
    println!("║{: ^width$}║", "РЕЗУЛЬТАТ");
    println!("╠{}╣", "─".repeat(width - 2));
    println!("║ {:<width$} ║", title);
    println!("╠{}╣", "─".repeat(width - 2));
    println!("║{: ^width$}║", result);
    println!("╚{}╝", "═".repeat(width - 2));
}

/// Обертка текста для переноса строк
fn wrap_text(text: &str, width: usize) -> Vec<String> {
    let mut lines = Vec::new();
    let mut current_line = String::new();
    
    for word in text.split_whitespace() {
        if current_line.len() + word.len() + 1 > width {
            lines.push(current_line.trim().to_string());
            current_line = String::new();
        }
        current_line.push_str(word);
        current_line.push(' ');
    }
    
    if !current_line.is_empty() {
        lines.push(current_line.trim().to_string());
    }
    
    lines
}

/// Анимированная загрузка (для эффекта)
pub fn loading_spinner(seconds: u64) {
    let frames = ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];
    for i in 0..seconds*10 {
        print!("\rВычисление {} ", frames[i as usize % frames.len()]);
        std::io::Write::flush(&mut std::io::stdout()).unwrap();
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
    print!("\r{}", " ".repeat(20));
    print!("\r");
}