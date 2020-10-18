// TODO: integrate with crate::io

use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, StdinLock, StdoutLock, Write};

pub struct IO {
    input: Vec<u8>,
    pos: usize,
    in_buf: BufReader<StdinLock<'static>>,
    out_buf: BufWriter<StdoutLock<'static>>,
}

impl IO {
    pub fn new() -> Self {
        let inp = Box::new(stdin());
        let out = Box::new(stdout());
        IO {
            input: Vec::new(),
            pos: 0,
            in_buf: BufReader::new(Box::leak(inp).lock()),
            out_buf: BufWriter::new(Box::leak(out).lock()),
        }
    }
    fn scan_raw(&mut self) -> &[u8] {
        loop {
            if self.pos == self.input.len() {
                self.input.clear();
                self.flush();
                self.in_buf.read_until(b'\n', &mut self.input).unwrap();
                self.pos = 0;
            } else if self.input[self.pos].is_ascii_whitespace() {
                self.pos += 1;
            } else {
                break;
            }
        }
        let i = self.pos;
        while self.pos != self.input.len() && !self.input[self.pos].is_ascii_whitespace() {
            self.pos += 1;
        }
        &self.input[i..self.pos]
    }
    pub fn scan<T: Scan>(&mut self) -> T {
        T::scan(self)
    }
    pub fn vec<T: Scan>(&mut self, n: usize) -> Vec<T> {
        (0..n).map(|_| self.scan()).collect()
    }
    pub fn graph(&mut self) -> (usize, usize, Vec<Vec<usize>>) {
        let n = self.scan();
        let m = self.scan();
        let mut graph = vec![Vec::new(); n];
        for _ in 0..m {
            let u: usize = self.scan();
            let v: usize = self.scan();
            graph[u].push(v);
            graph[v].push(u);
        }
        (n, m, graph)
    }
    pub fn digraph(&mut self) -> (usize, usize, Vec<Vec<usize>>) {
        let n = self.scan();
        let m = self.scan();
        let mut graph = vec![Vec::new(); n];
        for _ in 0..m {
            let u: usize = self.scan();
            let v: usize = self.scan();
            graph[u].push(v);
        }
        (n, m, graph)
    }
    pub fn tree(&mut self) -> (usize, Vec<Vec<usize>>) {
        let n = self.scan();
        let mut graph = vec![Vec::new(); n];
        for _ in 0..n - 1 {
            let u: usize = self.scan();
            let v: usize = self.scan();
            graph[u].push(v);
            graph[v].push(u);
        }
        (n, graph)
    }
}

impl IO {
    pub fn print<T: Print>(&mut self, x: T) {
        T::print(self, x);
    }
    pub fn println<T: Print>(&mut self, x: T) {
        self.print(x);
        self.print("\n");
    }
    pub fn iterln<T: Print, I: Iterator<Item = T>>(&mut self, mut iter: I, delim: &str) {
        if let Some(v) = iter.next() {
            self.print(v);
            for v in iter {
                self.print(delim);
                self.println(v);
            }
        }
        self.print("\n");
    }
    pub fn flush(&mut self) {
        self.out_buf.flush().unwrap();
    }
}

pub trait Scan {
    fn scan(io: &mut IO) -> Self;
}

macro_rules! impl_parse_int {
    ($($t:tt),*) => {
        $(
            impl Scan for $t {
                fn scan(s: &mut IO) -> Self {
                    let mut res = 0;
                    for d in s.scan_raw() {
                        res *= 10;
                        res += (*d - b'0') as $t;
                    }
                    res
                }
            }
        )*
    };
}

impl_parse_int!(i32, i64, isize, u32, u64, usize);

impl Scan for u8 {
    fn scan(s: &mut IO) -> Self {
        let bytes = s.scan_raw();
        debug_assert_eq!(bytes.len(), 1);
        bytes[0]
    }
}

impl<T: Scan, U: Scan> Scan for (T, U) {
    fn scan(s: &mut IO) -> Self {
        (T::scan(s), U::scan(s))
    }
}

impl<T: Scan, U: Scan, V: Scan> Scan for (T, U, V) {
    fn scan(s: &mut IO) -> Self {
        (T::scan(s), U::scan(s), V::scan(s))
    }
}

impl<T: Scan> Scan for [T; 2] {
    fn scan(s: &mut IO) -> Self {
        [s.scan(), s.scan()]
    }
}

impl<T: Scan> Scan for [T; 3] {
    fn scan(s: &mut IO) -> Self {
        [s.scan(), s.scan(), s.scan()]
    }
}

impl<T: Scan> Scan for [T; 4] {
    fn scan(s: &mut IO) -> Self {
        [s.scan(), s.scan(), s.scan(), s.scan()]
    }
}

impl Scan for Vec<u8> {
    fn scan(s: &mut IO) -> Self {
        s.scan_raw().to_owned()
    }
}

pub trait Print {
    fn print(w: &mut IO, x: Self);
}

macro_rules! impl_print_int {
    ($($t:ty),*) => {
        $(
            impl Print for $t {
                fn print(w: &mut IO, x: Self) {
                    w.out_buf.write_all(x.to_string().as_bytes()).unwrap();
                }
            }
        )*
    };
}

impl_print_int!(i32, i64, isize, u32, u64, usize);

impl Print for &str {
    fn print(w: &mut IO, x: Self) {
        w.out_buf.write_all(x.as_bytes()).unwrap();
    }
}

impl Print for u8 {
    fn print(w: &mut IO, x: Self) {
        w.out_buf.write_all(&[x]).unwrap();
    }
}

impl<T: Print, U: Print> Print for (T, U) {
    fn print(w: &mut IO, (x, y): Self) {
        w.print(x);
        w.print(" ");
        w.print(y);
    }
}

impl<T: Print, U: Print, V: Print> Print for (T, U, V) {
    fn print(w: &mut IO, (x, y, z): Self) {
        w.print(x);
        w.print(" ");
        w.print(y);
        w.print(" ");
        w.print(z);
    }
}
