fn array_loop(){
    // Array and Loop
    let mut arr: [i32; 5] = [10, 20, 30, 40, 50]; 
    println!("Array: {:?}", arr); // the :? is a debug trait that prints the array in a debug format
    println!("Array: {}", arr[0]);
    println!("Array length: {}", arr.len());

    // Loop through array 
    let arr_2 = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let mut loop_index = 0;
    loop {
        // Check if the current element is even or odd and print accordingly
        if (arr_2[loop_index] % 2 == 0) {
            println!("Even number: {}", arr_2[loop_index]);
        } else {
            println!("Odd number: {}", arr_2[loop_index]);
        } 
        // Check if reached the end of the array and break the loop if so
        if (arr_2.len() - 1 == loop_index ) {
            break;
        }
        loop_index += 1;
    };

    // while loop
    let arr_3 = [12, 4, 65, 3, 6, 8];
    let mut loop_index3 = 0;
    print!("Array 3 while loop: ");
    while loop_index3 < arr_3.len() {
        print!("{} ", arr_3[loop_index3]);
        loop_index3 += 1;
    };
    print!(" \n");

    let arr_4 = [9, 4, 1, 11, 17, 19];
    // for loop 
    print!("Array 4 for loop: ");
    for value in  arr_4.iter(){
        print!("{} ", value);
    };
    print!(" \n");
}
