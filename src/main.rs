fn main() {
    multiples_of_three_and_five();
    sum_of_even_fibonacchin_numbers();
}

fn multiples_of_three_and_five()
{
    let mut x = 0;
    let mut rsult = 0;
    while x < 1000{
        if x % 3 == 0 || x % 5 == 0 {
            rsult = rsult + x;
        }
        x = x+1;
    }
let s: String = rsult.to_string();
println!("{}",s);
}

fn sum_of_even_fibonacchin_numbers()
{
    let mut first = 0;
    let mut second = 1;

    let mut sum = 0;
    let mut rsult = 0;
    while sum < 4000000{
        sum = first + second;
        first = second;
        second = sum;
        if sum % 2 == 0{
            rsult = rsult + sum;
        }
    }
let s: String = rsult.to_string();
println!("{}",s);
}
