// fn main() {
//     let data = vec![1, 2, 3];
//     let mut foo = data.
//         // Convert list to iterator
//         iter()
//         // Map each value
//         .map(|x| x + 1)
//         // Reconverted it back into a list
//         // .collect();

//         let mut new_vector = vec![];

//         while let Some(x) = foo.next() {
//             new_vector.push(x);
//         }
//     println!("{:?}", new_vector);
// }

// fn main() {
//     let file = std::fs::read_to_string("lines").unwrap();

//     // Similar to split and it creates an iterator
//     // file.lines()
//     //     .enumerate()
//     //     .filter(|(idx, _)| idx % 2 == 0)
//     //     .for_each(|(_, line)| println!("{}", line));

//     file.lines()
//         .enumerate()
//         .filter(|(idx, _)| idx % 2 == 0)
//         .skip(2)
//         .take(2)
//         .for_each(|(_, line)| println!("{}", line));
// }

// enum Color {
//     Red,
//     Blue,
//     Green,
//     Yellow,
// }

// impl Color {
//     fn is_green(&self) -> bool {
//         if let Color::Green = self {
//             return true;
//         }
//         return false;
//     }

//     fn is_green_parts(&self) -> bool {
//         match self {
//             Color::Red => return false,
//             Color::Green => return false,
//             Color::Blue => return true,
//             Color::Yellow => return true,
//         }
//     }
// }

// fn print_color(color: Color) {
//     match color {
//         Color::Red => print!("Red"),
//         Color::Blue => print!("Blue"),
//         Color::Green => print!("Green"),
//         Color::Yellow => print!("Yellow"),
//     }
// }

// fn main() {
//     // print_color(Color::Yellow);
//     let foo = Color::Green;

//     foo.is_green();
// }

// struct Custom {
//     age: usize,
//     name: String,
// }

// enum Item {
//     Number(usize),
//     String(String),
//     MyCustom(Custom),
// }

// fn append(items: &mut Vec<Item>) {
//     items.push(Item::String("Hello fem".into()));
// }

// fn main() {
//     let mut items: Vec<Item> = vec![];

//     append(&mut items);
// }

// fn multiply(num: Option<usize>) -> Option<usize> {
//     // Converts from Option to usize
//     // let num = num?;
//     // return num.map(|x| x * 5);
//     return Some(num? * 5);
// }

// fn multiply(num: Vec<usize>, index: usize) -> usize {
//     return num.get(index).unwrap_or(&index) * 5;
// }
// fn main() {
//     let file_name = std::env::args().nth(1).expect("File name was not provided");

//     // File read
//     let file = std::fs::read_to_string(file_name).expect("Unable to read the file to string");

//     file.lines().for_each(|line| {
//         if let Ok(value) = line.parse::<usize>() {
//             println!("{}", value);
//         } else {
//             println!("Line not a number")
//         }
//     });
// }

#[derive(Debug)]
struct Item {
    count: usize,
}

fn add_one(item: &mut Item) {
    item.count += 1;
}

fn print_all(items: &Vec<Item>) {
    for item in items {
        println!("{:?}", item);
    }
}

fn main() {
    let mut items = vec![Item { count: 1 }];

    let first = items.first_mut();
    println!("{:?}", first);
    print_all(&items);
}
