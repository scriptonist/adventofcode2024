fn main() {
    let input = include_str!("../input.txt");
    println!("{}", p1(input));
    println!("{}", p2(input));
}

fn p1(input: &str) -> u64 {
    #[derive(Clone, Debug, PartialEq, Copy)]
    enum Item {
        File(u32),
        Free,
    }
    fn print_memory(memory: &Vec<Item>) {
        for item in memory {
            match item {
                Item::File(id) => {
                    print!("{}", id)
                }
                Item::Free => {
                    print!(".")
                }
            }
        }
        println!();
    }
    let mut checksum: u64 = 0;
    let memory: Vec<char> = input.lines().next().unwrap().chars().collect();
    let mut memory_layout: Vec<Item> = Vec::new();
    let mut file_id = 0;
    for (idx, c) in memory.iter().enumerate() {
        let size = c.to_digit(10).unwrap();
        let item: Item;
        if idx % 2 == 0 {
            item = Item::File(file_id);
            file_id += 1;
        } else {
            item = Item::Free;
        }
        for _ in 0..size {
            memory_layout.push(item.clone());
        }
    }
    // print_memory(&memory_layout);
    let (mut p1, mut p2) = (0, memory_layout.len() - 1);
    while p1 < memory_layout.len() || p2 > 0 {
        while p1 < memory_layout.len() - 1 && p2 > 0 {
            if memory_layout[p1] != Item::Free {
                p1 += 1
            } else {
                break;
            }
        }
        while p2 > 0 && p1 < memory_layout.len() - 1 {
            if memory_layout[p2] == Item::Free {
                p2 -= 1
            } else {
                break;
            }
        }
        if p1 >= p2 || p2 <= p1 {
            break;
        }
        // println!("{} {}", p1, p2);
        (memory_layout[p1], memory_layout[p2]) = (memory_layout[p2], memory_layout[p1]);
        // print_memory(&memory_layout);
    }
    for (idx, item) in memory_layout.iter().enumerate() {
        match item {
            Item::File(id) => {
                checksum += (idx as u32 * id) as u64;
            }
            _ => {}
        }
    }
    checksum
}

fn p2(input: &str) -> u64 {
    #[derive(Clone, Debug, PartialEq, Copy)]
    enum Item {
        File(u32, u32),
        Free(u32),
    }
    fn print_memory(memory: &Vec<Item>) {
        for item in memory {
            match item {
                Item::File(id, size) => {
                    print!("{}", id.to_string().repeat(*size as usize))
                }
                Item::Free(size) => {
                    print!("{}", ".".to_string().repeat(*size as usize))
                }
            }
        }
        println!();
    }
    let mut checksum: u64 = 0;
    let memory: Vec<char> = input.lines().next().unwrap().chars().collect();
    let mut memory_layout: Vec<Item> = Vec::new();
    let mut file_id = 0;
    for (idx, c) in memory.iter().enumerate() {
        let size = c.to_digit(10).unwrap();
        let item: Item;
        if idx % 2 == 0 {
            item = Item::File(file_id, size);
            file_id += 1;
        } else {
            item = Item::Free(size);
        }
        memory_layout.push(item.clone());
    }
    let mut s: Vec<Item> = vec![];
    for item in &memory_layout {
        match item {
            Item::File(_id, _size) => {
                s.push(item.clone());
            }
            _ => {}
        }
    }
    // print_memory(&memory_layout);
    while !s.is_empty() {
        let file = s.pop().unwrap();
        let memory_layout_copy = memory_layout.clone();
        let mut is_moved = false;
        // dbg!(file);
        for (idx, item) in memory_layout_copy.iter().enumerate() {
            if file == *item {
                break
            }
            match file {
                Item::File(id, file_size) => {
                    match item {
                        Item::Free(free_size) => {
                            if *free_size >= file_size {
                                let mut to_append: Vec<Item> = vec![file.clone()];
                                let space_left = *free_size - file_size;
                                if space_left > 0 {
                                   to_append.push(Item::Free(space_left)); 
                                }
                                memory_layout.remove(idx);
                                memory_layout.splice(idx..idx, to_append.clone());
                                is_moved = true;
                                break
                            }
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }
        if is_moved {
            let mut to_remove_idx = 0;
            for (idx, item) in memory_layout.iter().enumerate() {
                if *item == file {
                    to_remove_idx = idx
                }
            }
            match file {
                Item::File(id, file_size) => {
                    memory_layout[to_remove_idx] = Item::Free(file_size);
                }
                _ => {}
            }
        }
        // print_memory(&memory_layout);
    }
    let mut i = 0;
    for item in &memory_layout {
       match item {
           Item::File(id, size) => {
               for _ in 0..*size {
                   // println!("{} * {}", i, id);
                   checksum += (i * id) as u64;
                   i += 1;
               }
           }
           Item::Free(size) => {
               i += size;
           }
       } 
    }
    checksum
}
