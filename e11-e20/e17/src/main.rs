/*
If the numbers 1 to 5 are written out in words: one, two, three, four, five, then there are 3 + 3 + 5 + 4 + 4 = 19 letters used in total.

If all the numbers from 1 to 1000 (one thousand) inclusive were written out in words, how many letters would be used?

NOTE: Do not count spaces or hyphens. For example, 342 (three hundred and forty-two) contains 23 letters and 115 (one hundred and fifteen)
contains 20 letters. The use of "and" when writing out numbers is in compliance with British usage.
*/

fn main() {
    let mut total: i32 = 0;
    
    for i in 1..1001 {
        total+=get_letter_count(i);
//        println!("{} : {}", i, get_letter_count(i));
    }
    println!("Total number of letters: {}", total);
}

fn get_letter_count(n: usize) -> i32 {
    // Let's create an array of the total number of letters in each word
    // i.e. one = 3, and = 3, etc.
    //
    // "one", "two", ... "teens" ... "tens" ... "hundred", "thousand", "and"
    let mut n2 = n;
    let nums = vec![3i32,3,5,4,4,3,5,5,4,3,
                    6,6,8,8,7,7,9,8,8,
                    6,6,5,5,5,7,6,6,
                    7,8,3]; // Size 30
    // Let's go ahead and take care of one thousand right off the top...
    if n2 == 1000 {
        return nums[28] + nums[0];
    }
    let mut count: i32 = 0;
    // Now we will get all the hundreds ranges...
    if n2 / 100 > 0 {
        count+=nums[n2 / 100 - 1] + nums[27];
        n2 = n2 - ((n2 / 100) * 100); // remove the hundreds range
        if n2>0 {
            count+=nums[29]; // There is some remainder after removing hundreds, so there must be an "and"
        } else {
            return count; // Even hundreds
        }
    }
    if n2 < 21 && n2>0  {
        count+=nums[n2-1];
    } else {
        count+=nums[n2/10 + 17];
        if n2%10 > 0 {
            count+=nums[n2%10 - 1];
        }
    }
    return count;
}

    
