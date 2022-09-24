#[allow(unused_macros)]
macro_rules! parse_line {
    ( $t:ty ) => (
        {
            let mut line = String::new();
            ::std::io::stdin().read_line(&mut line).unwrap();
            let mut iter = line.split_whitespace();
            iter.next().unwrap().parse::<$t>().unwrap()
        }
    );

    ( $( $t:ty), +) => (
        {
            let mut line = String::new();
            ::std::io::stdin().read_line(&mut line).unwrap();
            let mut iter = line.split_whitespace();
            ( $(iter.next().unwrap().parse::<$t>().unwrap()),* )
        }
    );
}

#[allow(unused_macros)]
macro_rules! read_line {
    () => {{
        let mut line = String::new();
        ::std::io::stdin().read_line(&mut line).unwrap();
        line.pop();
        line
    }};
}

#[allow(unused_macros)]
macro_rules! parse_vec {
    ( $t:ty ) => {{
        let mut line = String::new();
        ::std::io::stdin().read_line(&mut line).unwrap();
        let iter = line.split_whitespace();
        iter.map(|v| v.parse::<$t>().unwrap()).collect::<Vec<_>>()
    }};
}

fn main() {
    let (x, y, z) = parse_line!(i32, i32, i32);
    let mut ans = 0;
    if x.signum() != y.signum() {
        ans = x.abs();
    } else if x > 0 && x < y {
        ans = x.abs();
    } else if x < 0 && x > y {
        ans = x.abs();
    } else if y > 0 && y < z {
        ans = -1;
    } else if y < 0 && y > z {
        ans = -1
    } else {
        let diff = x - z;
        ans = z.abs() + diff.abs();
    }
    println!("{}", ans);
}
