pub(crate) fn fib_loop(n: u8) {
	let mut a = 0;
	let mut b = 1;

	loop {
		fib_cal(&mut a, &mut b);

		println!("{:?}", a);

		if a > n {
            break;
        }
	}
}

pub(crate) fn fib_while(n: u8) {
	let mut a = 0;
    let mut b = 1;

	while a < n {
		fib_cal(&mut a, &mut b);

		println!("{:?}", a);
	}
}

fn fib_cal(a: &mut u8, b: &mut u8) {
    let c = *a;
    *a = *a + *b;
    *b = c;
}

fn fib_for(n: u8) {

	for _i in (0..n).rev() {
		println!("{}", _i);
	}
}
