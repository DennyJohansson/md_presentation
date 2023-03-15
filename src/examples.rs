#[allow (unused)]
// moved
fn example () {
    let hello = "hello".to_string();

    let hello_world = append_world(hello);
//     let hello_world_two = append_world(hello);
// 
//     println!("{} {}", hello_world, hello_world_two);
}

fn append_world (s: String) -> String {
    let mut s = s.to_string();
    s.push_str(", world!");
    s
}


#[allow (unused)]
fn print_out_item(item: Vec<i32>) {
    for i in item {
        println!("{}", i);
    }

    // item.push(4);
}

#[allow (unused)]
fn example_two() {
    let item = vec![1, 2, 3];

    print_out_item(item);
    // print_out_item(item);
}

#[allow (unused)]
// mutable ref
fn example_three() {
    let mut vector = vec![1, 2, 3];

    let last_item = vector.last();

    vector.pop();

    // println!("{:?}", last_item);
}

