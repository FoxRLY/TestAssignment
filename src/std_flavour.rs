use std::marker::Send;
use std::vec;
use std::{thread, time::Duration};

/// Тестовая функция для проверки работы многопоточности
pub fn test_func(t: i32) -> String {
    println!("Processing value: {t}");
    thread::sleep(Duration::from_secs(2));
    format!("Processed value: {t}")
}

/// Целочисленное деление с округлением в большую сторону
///
/// # Паникует:
/// Если b = 0
fn div_ceil(a: usize, b: usize) -> usize {
    if b == 0 {
        panic!("DivisionByZero: b == 0")
    };
    (a + b - 1) / b
}

/// Разбивает вектор на равные части, поглощая его
///
/// # Поведение:
/// Чем меньше элементов, тем на меньшее количество частей делится вектор.
/// Если количество элементов превышает предел в два раза, то вектор делится на равные части
/// с количеством элементов меньше двойного предела.
/// При превышении максимума частей вектор делится на максимум равных частей, игнорируя пределы.
///
/// # Аргументы:
/// - data: Vec<T> - разбиваемый вектор
/// - chunk_threshold: usize - минимальное кол-во элементов в части (предел)
/// - max_chunk_count: usize - максимальное количество частей (максимум)
///
/// # Паникует:
/// Если предел или максимум равны нулю
pub fn slice_into_chunks<T>(
    data: Vec<T>,
    chunk_threshold: usize,
    max_chunk_count: usize,
) -> Vec<Vec<T>> {
    if chunk_threshold == 0 || max_chunk_count == 0 {
        panic!("InvalidArgument: chunk_threshold or max_chunk_count equals zero");
    }

    // Расчет оптимального количества частей
    let chunk_count = match data.len() / chunk_threshold {
        0 => 1,
        count if count > max_chunk_count => max_chunk_count,
        count => count,
    };

    // Расчет оптимального размера части
    let chunk_size = div_ceil(data.len(), chunk_count);

    // Вектор для хранения частей
    let mut result: Vec<Vec<T>> = Vec::with_capacity(chunk_count);

    // Процесс разбиения на части
    let mut data_iter = data.into_iter().peekable();    // Превращаем данные в итератор 
    'chunk_end: for chunk_index in 0..chunk_count {     // Верхний цикл по частям
        if data_iter.peek().is_none() {                 // Если данные кончились - выходим
            break;
        }
        result.push(Vec::with_capacity(chunk_size));    // Иначе создаем новую часть
        for _ in 0..chunk_size {                        // Цикл заполнения части
            if let Some(item) = data_iter.next() {
                result[chunk_index].push(item);
            } else {
                break 'chunk_end;                       // Если данные кончились, то выходим из
                                                        // обоих циклов
            }
        }
    }
    result
}

/// Многопоточный исполнитель
pub fn parallel_func<T, R, F>(data: Vec<T>, func: F) -> Vec<R>
where
    F: (FnMut(T) -> R) + Send + 'static + Clone,
    R: Send + 'static,
    T: Send + 'static,
{
    let threshold_chunk_size: usize = 10;   // Размеры делений данных до достижения
                                            // максимального количества потоков
    let max_thread_count = 10;              // Максимальное количество потоков

    // Деление данных на более менее равные части
    let chunks = slice_into_chunks(data, threshold_chunk_size, max_thread_count);

    // Запуск потоков 
    let mut thread_handles = vec![];
    for chunk in chunks.into_iter() {
        let mut new_func = func.clone();
        thread_handles.push(thread::spawn(move || {
            let mut thread_result = vec![];
            for item in chunk.into_iter() {
                thread_result.push(new_func(item));
            }
            thread_result
        }));
    }

    // Получение результатов через join
    let mut result = vec![];
    for handle in thread_handles {
        result.extend(handle.join().unwrap());
    }
    result
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn parallel_test(){
        let data = vec![1,2,3,4,5];
        let result = parallel_func(data.clone(), |x|{
            println!("Processed {x}");
            x
        });
        assert_eq!(data, result);
    }
}
