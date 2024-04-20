use sorting_lib::sorting::*;

fn main() {
    let mut vec = vec![5, 3, 2, 4, 1];
    quick_sort(&mut vec);
    println!("Sorted integers using Quick Sort: {:?}", vec);

    let mut words = vec!["banana".to_string(), "apple".to_string(), "orange".to_string()];
    insertion_sort(&mut words);
    println!("Sorted strings using Insertion Sort: {:?}", words);

    let mut floats = vec![3.14, 1.59, 2.65, 5.35, 8.97];
    selection_sort(&mut floats);
    println!("Sorted floats using Selection Sort: {:?}", floats);

    let mut characters = vec!['A', 'b', 'C', 'd'];
    merge_sort(&mut characters);
    println!("Sorted characters using Merge Sort: {:?}", characters);
}
