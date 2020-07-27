use std::collections::HashMap;
use std::io;

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

    // println!("{:?}", map);

    // statistics();

    // convert_str("first");
    // convert_str("apple");
    init_inser_practice();
}

fn init_inser_practice() {
    let mut user_vec: Vec<String> = Vec::new();
    let mut dept_map: HashMap<String, Vec<String>> = HashMap::new();

    loop {
        println!("Input u to add user, d to add dept, s to display, q to quit:");
        let mut user_select = String::new();
        io::stdin().read_line(&mut user_select).expect("Error format");

        match user_select.chars().next().unwrap() {
            'u' => {
                let mut user_name = String::new();
                let mut dept_name = String::new();
                if dept_map.len() == 0 {
                    println!("Empty dept!");
                    continue;
                }
                println!("Input user name");
                io::stdin().read_line(&mut user_name).expect("Error username format");
                
                println!("Input dept name");
                io::stdin().read_line(&mut dept_name).expect("Error deptname format");

                match dept_map.get_mut(&dept_name) {
                    Some(c) => {
                        println!("current user vec: {:?}", c);
                        c.push(user_name);
                    },
                    None => println!("Empty")
                }
            },
            'd' => {
                println!("Input dept name");
                let mut dept_name = String::new();
                io::stdin().read_line(&mut dept_name).expect("Error deptname format");

                dept_map.insert(dept_name, Vec::new());
            },
            's' => {
                for (dept, c_user_vec) in dept_map.iter() {
                    println!("Dept name: {}", dept);

                    for user in c_user_vec.iter() {
                        println!("User name: {}", user);
                    }
                }  
            },
            'q' => {
                println!("Exit");
                break
            },
            _ => println!("Not a valid input")
        }   
    }

    
    // add_dept(&mut deptMap);
    // add_user_to_dept(&mut userVec, &mut deptMap);
}

fn add_dept(deptMap: &mut HashMap<&str, &mut Vec<&str>>) {
    // println!("Add dept ctrl:");
    // let mut input_str = String::new();
    // io::stdin().read_line(&mut input_str).expect("Error format");
    // let mut newVec: Vec<&str> = Vec::new();
    // deptMap.insert(&input_str, &mut Vec::new()).expect("insert Error");
    // println!("Exit add dept.");
}

fn add_user_to_dept(userVec: &mut Vec<&str>, deptMap: &mut HashMap<&str, &mut Vec<&str>>) {
    // println!("Add user to dept ctrl:");
    // let mut input_str = String::new();
    // io::stdin().read_line(&mut input_str).expect("Error format");

    
}

fn convert_str(source: &str) {
    let mapper: HashMap<&str, bool> = [
        ("a", true),
        ("e", true),
        ("i", true),
        ("o", true),
        ("u", true),
    ]
    .iter()
    .cloned()
    .collect();

    let mut res = String::new();
    for word in source.split_whitespace() {
        let mut target = String::from(source);

        let mut needed_suffix: bool = true;
        for c in mapper.keys() {
            // 判断是否元音起始
            let res = target.starts_with(c);

            // 有元音, 不需要偏移
            if res {
                needed_suffix = false;
                break;
            }
        }

        println!("needed_suffix: {}", needed_suffix);

        if needed_suffix {
            for w in word.chars() {
                let suffix_str = format!("{}-{}{}", &word[1..], w, "ay");
              res.push_str(&suffix_str); 
              break;
            }
            
        } else {
            let suffix_str = String::from(format!("{}-{}", word, "hay"));
            res.push_str(&suffix_str);
        }
    }

    println!("res str: {}", res);
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

    let mut temp_number;
    let length = sorted_numbers.len();
    println!("sorted_numbers length: {}", length);
    // 排序
    for (pos, num) in numbers.iter().enumerate() {
        println!("current pos: {}, num: {}", pos, num);

        let mut inner_index = 0;
        loop {
            if inner_index >= (length - pos - 1) {
                break;
            }
            if sorted_numbers[inner_index] > sorted_numbers[inner_index + 1] {
                temp_number = sorted_numbers[inner_index + 1];
                sorted_numbers[inner_index + 1] = sorted_numbers[inner_index];
                sorted_numbers[inner_index] = temp_number;
            }
            inner_index += 1;
        }
    }

    if length % 2 == 0 {
        println!(
            "中位数: {}",
            (sorted_numbers[length / 2] + sorted_numbers[(length / 2) + 1]) / 2
        );
    } else {
        println!("中位数: {}", sorted_numbers[length / 2]);
    }

    let mut times_map: HashMap<i32, i32> = HashMap::new();

    for num in numbers.iter() {
        let count = times_map.entry(*num).or_insert(0);
        *count += 1;
    }

    let mut max_times_num: Option<i32> = None;
    let mut max_times_count = 0;

    for (key, times) in times_map.iter() {
        println!("num: {}, times: {}", key, times);
        if *times > max_times_count {
            println!("entered if");
            max_times_num = Some(*key);
            max_times_count = *times;
        }
    }

    if let Some(c) = max_times_num {
        println!("众数: {}", c);
    }

    println!("sorted_numbers: {:?}", sorted_numbers);
}
