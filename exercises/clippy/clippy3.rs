// clippy3.rs

fn main() {
    let my_option: Option<()> = None;
    // 可以直接不写 if，或者用 if let 处理 Some 的情况
    // 这里既然是 None，什么都不做就行
    // 移除 .is_none() + unwrap() 的危险组合

    let my_arr = &[
        -1, -2, -3,
        -4, -5, -6,
    ];
    println!("My array! Here it is: {:?}", my_arr);

    let mut my_empty_vec = vec![1, 2, 3, 4, 5];
    my_empty_vec.clear();  // 最清晰的清空方式
    println!("This Vec is empty, see? {:?}", my_empty_vec);

    let mut value_a = 45;
    let mut value_b = 66;
    std::mem::swap(&mut value_a, &mut value_b);
    println!("value a: {}; value b: {}", value_a, value_b);
}