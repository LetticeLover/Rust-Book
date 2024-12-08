fn main() {
    // Can use unconditional loops and "break" out of them with the
    // break keyword.
    // Can also name loops and specify the loop you're breaking out of.
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    // Can use a while loop to loop over a collection
    // but this is error prone and generally not a good idea.
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }

    // Instead, use a for loop to loop over each element.
    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("the value is: {element}");
    }

    // Can use a Range to loop code a certain number of times.
    // This loop uses .rev() to reverse the range and thus countdown
    // from 4 to 1.
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}