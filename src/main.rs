fn main() {
    let mut a: int = 99;

    while a >= 0 {
    	if a > 1 {
    		println!("{} bottles of beer on the wall, {} bottles of beer.", a, a);
    		println!("Take one down and pass it around, {} bottles of beer on the wall.", a - 1);
    	}
    	else if a == 1 {
    		println!("{} bottle of beer on the wall, {} bottle of beer.", a, a);
    		println!("Take one down and pass it around, no more bottles of beer on the wall.");
    	}
    	else {
    		println!("No more bottles of beer on the wall, no more bottles of beer.");
    		println!("Go to the store and buy some more, 99 bottles of beer on the wall.");
    	}
    	a -= 1;
    }
}
