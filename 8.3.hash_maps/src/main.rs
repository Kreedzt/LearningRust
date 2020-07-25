use std::collections::HashMap;

fn main() {
    // let mut scores = HashMap::new();

    // scores.insert(String::from("Blue"), 10);
    // scores.insert(String::from("Yello"), 50);

    // let teams = vec![String::from("Blue"), String::from("Yellow")];
    // let initial_scores = vec![10, 50];

    // 使用下划线占位类型, 根据 Vector 中数据的类型推断出类型
    // zip 创建一个元组的 Vector
    // collect 转成 HashMap
    // let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    // let field_name = String::from("Favorite color");
    // let field_value = String::from("Blue");

    // let mut map = HashMap::new();
    // map.insert(field_name, field_value);

    // println!("map: {:?}", map);

    // Error: 所有权已转移
    // println!("field_name: {}, field_value: {}", field_name, field_value);
    //    |                                                 ^^^^^^^^^^ value borrowed here after move

    // let team_name = String::from("Blue");
    // let score = scores.get(&team_name);

    // match score {
    //     Some(a) => println!("score: {}", a),
    //     None => println!("No values!")
    // }

    // if let Some(a) = score {
    //     println!("if let score: {}", a);
    // }

    // 遍历
    // for (key, value) in &scores {
    //     println!("{}: {}", key, value);
    // }

    // 更新
    // scores.insert(String::from("Blue"), 10);

    // println!("{:?}", scores);

    // 仅在不存在时添加
    // let mut scores = HashMap::new();
    // scores.insert(String::from("Blue"), 10);

    // entry 用于检测是否存在关联值
    // scores.entry(String::from("Cyan")).or_insert(30);
    // scores.entry(String::from("Blue")).or_insert(70);

    // println!("{:?}", scores);

    let text = "hello world wonderful world";

    let mut map = HashMap::new();
    
    // 以单词分割
    for word in text.split_whitespace() {
        // 统计每一个单词出现多少次, 出现了 + 1
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);

    statistics();
}


fn statistics() {
    println!("statistics fn:");
    let numbers = vec![1, 5, 3, 20, 40, 20, 18, 20, 20, 2];

    let mut total = 0;

    for num in numbers.iter() {
        println!("current num: {}", num);
        total += num;
    }
    
    let average = total as f64 / numbers.len() as f64;

    println!("average: {}", average);

    let mut sorted_numbers: Vec<i32> = numbers.clone();
    let mut inner_index = 0;
    let mut temp_number;
    // 排序
    for (pos) in numbers.iter().enumerate() {
        println!("current pos: {}, num: {}", pos, num);

        inner_index = 0;
        loop {
            if inner_index <= sorted_numbers.len() - 2 {
                break;
            }
            if &sorted_numbers[inner_index] > &sorted_numbers[inner_index + 1] {
                temp_number = sorted_numbers[inner_index + 1];
                sorted_numbers[inner_index + 1] = sorted_numbers[inner_index];
                sorted_numbers[inner_index] = temp_number;
            }
            inner_index += 1;
        }
    }

    println!("sorted_numbers: {:?}", sorted_numbers);
}
