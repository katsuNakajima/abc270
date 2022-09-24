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

type Graph = Vec<Vec<usize>>;

fn dfs(g: &Graph, st: usize, end: usize, ans: &mut Vec<usize>, n: usize) -> bool {
    if st == end {
        ans.push(st);
        return true;
    }
    for &i in g[st].iter() {
        if i == n {
            continue;
        }
        if dfs(&g, i, end, ans, st) {
            ans.push(st);
            return true;
        }
    }
    false
}

fn main() {
    let mut ans = Vec::new();
    let (n, y, z) = parse_line!(usize, usize, usize);
    let mut g: Graph = vec![Vec::new(); n + 1];
    for _i in 0..n - 1 {
        let (u, v) = parse_line!(usize, usize);
        g[u].push(v);
        g[v].push(u);
    }
    dfs(&g, y, z, &mut ans, n);
    ans.reverse();
    for x in ans {
        print!("{} ", x);
    }
}
