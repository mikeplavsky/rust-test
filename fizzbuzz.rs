extern mod std;

fn is_n( num: int, n: int) -> bool {
  num % n == 0
}

fn is_five( num: int ) -> bool {
  is_n( num, 5 )
}

fn is_fifteen( num: int ) -> bool {
  is_n( num, 15 )
}

fn is_three( num: int ) -> bool {
  is_n( num, 3 )
}

#[test]
fn test_FizzBuzz() {
  assert!( fizz_buzz(15) == ~"FizzBuzz"  ) 
}

#[test]
fn test_Buzz() {
  assert!( fizz_buzz(5) == ~"Buzz"  ) 
}

#[test]
fn test_Fizz() {
  assert!( fizz_buzz(3) == ~"Fizz"  ) 
}
                              
#[test]
fn test_is_three_with_not_three() {
  assert!( !is_three(1) )
}

#[test]
fn test_is_three_with_three() {
  assert!( is_three(3) )
}

fn fizz_buzz(num: int) -> ~str {

  if is_fifteen( num ) { ~"FizzBuzz" }
  else if is_three( num ) { ~"Fizz" }
  else if is_five( num ) { ~"Buzz" }
  else { int::to_str( num ) }

}

fn main() {
  for int::range(0,100) |num| {
    let answer = fizz_buzz( num ); 
    io::println( answer )  
  }
}
