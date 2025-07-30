fn main() {
    // TYPE CASTING

    let val: i32 = 1_3_3_7;
    let val_i8 = val as i8;
    println!("Value in i8 is {val_i8}");

    //FORMATING OF FLOAT with FORMAT SPECIFIERS

    let score = 67.879756373;
    //let new_score = score as f16; //error
    let new_score_prec = format!("{:.3}", score);
    println!("New score precise upto 3 is {new_score_prec}");

    let floating = 65.35;
    let new_floating = format!("{floating:.4}");
    println!("New floating value precisie upto 4  is {new_floating}");

    println!("The new no. with reqd. precision of 4 is {:.4}", floating);
    println!("The new no. with reqd. precision of 3 is {floating:.3}");

    // LOGICAL OPERATIONS

    let with_milk = true;
    let with_sugar = false;
    let tea = false;
    let is_my_type_of_coffee = with_milk && with_sugar || tea; //result: false
    let is_acceptable_coffee = with_milk || with_sugar && tea; //result: true

    // ARRAYS

    let mut numbers: [i32; 3] = [0, 0, 0];
    for number in numbers {
        println!("{number}");
    }

    println!("Length of the array is {}", numbers.len());

    let mut digits: [i8; 4] = [2, 6, 3, 8];
    println!("{digits:#?}");
    dbg!(digits); // dbg!(digits) is equivalent to println!("{digits:#?}")
    println!("Printing the 2nd element: {}", digits[1]); // ACCESSING ARRAY ELEMENTS

    digits[2] = 1; //Manipulating the array elements
    println!("{digits:#?}");

    //TUPLES

    let tup = ("Anol", 56, 'x', true);
    // let (name, age, grade, active) = ("Anol", 56,'x', true);  // CORRECT
    let (name, age, grade, active) = tup;

    dbg!(tup);
    println!("{tup:?}");

    //RANGE

    let range_var = 1..31;
    let range_inc = 1..=31;

    for range in range_var {
        println!("{range} ");
    }
    for range in range_inc {
        println!("{range} ");
    }

    // simple for loop
    for i in 0..10 {
        println!("{i}");
        dbg!(i);
    }
}
