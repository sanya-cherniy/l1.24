use bloomfilter::Bloom; // используем крейт реализующий фильтр Блума
use std::io;

fn main() {
    let num_items = 100000; // указываем оценочное количество элементов
    let fp_rate = 0.001; // указываем необходимую частоту ложных срабатываний
    let mut bloom = Bloom::new_for_fp_rate(num_items, fp_rate);

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap(); // считываем строку

    // разбиваем строку по символам
    for ch in input.chars() {
        if bloom.check(&ch) {
            // выводим "false" и завершаем работу если находим не уникальный символ
            println!("false");
            return;
        } else {
            // если символ уникальный добавляем этот символ и его версию в другом регистре в фильтр Блума
            bloom.set(&ch.to_lowercase().next().unwrap());
            bloom.set(&ch.to_uppercase().next().unwrap());
            // если символ не имеет регистра запись будет осуществлена дважды, но это не повлияет на вероятность ложного срабатывания
        }
    }
    println!("true");
}
