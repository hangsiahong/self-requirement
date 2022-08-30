use std::collections::LinkedList;

fn main() {
    let mut list = LinkedList::new();
    list.push_back('a');

    // list.append(["Test"].to_string());
    println!("{:?}", list);

    let mut list2 = LinkedList::new();
    list2.push_back('b');
    list2.push_back('c');

    list.append(&mut list2);

    println!("{:?}", list);
    // let mut list_len = list.len();
    println!("{:?}", list.get(2));

}

// pub fn bubble_sort<T: Ord>(arr: &mut [T]) {
//     if arr.is_empty() {
//         return;
//     }
//     let mut sorted = false;
//     let mut n = arr.len();
//     while !sorted {
//         sorted = true;
//         for i in 0..n - 1 {
//             if arr[i] > arr[i + 1] {
//                 arr.swap(i, i + 1);
//                 sorted = false;
//             }
//         }
//         n -= 1;
//     }
// }
