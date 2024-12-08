fn main() {
    for day in 1..=12 {
        // Print the day it is.
        println!();
        println!("On the {} day of Christmas, my true love gave to me", 
                match day {
                    1 => "first",
                    2 => "second",
                    3 => "third",
                    4 => "fourth",
                    5 => "fifth",
                    6 => "sixth",
                    7 => "seventh",
                    8 => "eighth",
                    9 => "ninth",
                    10 => "tenth",
                    11 => "eleventh",
                    12 => "twelfth",
                    _ => panic!("This value ({}) should not have been reached.", day)
                }
        );

        if day >= 12 { println!("Twelve drummers drumming,"); }
        if day >= 11 { println!("Eleven pipers piping,"); }
        if day >= 10 { println!("Ten lords a-leaping,"); }
        if day >= 9 { println!("Nine ladies dancing,"); }
        if day >= 8 { println!("Eight maids a-milking,"); }
        if day >= 7 { println!("Seven swans a-swimming,"); }
        if day >= 6 { println!("Six geese a-laying,"); }
        if day >= 5 { println!("Five golden rings,"); }
        if day >= 4 { println!("Four calling birds,"); }
        if day >= 3 { println!("Three French hens,"); }
        if day >= 2 { println!("Two turtle doves,"); }

        if day >= 2 { println!("And a partridge in a pear tree."); }
        else { println!("A partridge in a pear tree."); }
    }
}
