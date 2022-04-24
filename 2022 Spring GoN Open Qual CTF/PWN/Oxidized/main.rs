use std::io::{self, Write, BufRead};
use std::vec;

const BUF_SIZE: usize = 0x1000;
const HASH_MASK: u64 = 0xff;

#[derive(Clone)]
struct Node {
    key : u64,
    is_string : bool, 
    ptr : usize,
}

struct KVStore {
    inner: Vec<Vec<*mut Node>>,
}

impl KVStore {
    fn new() -> Self {
        Self {
            inner: vec![vec!(); 0xff],
        }
    }

    fn insert(&mut self, key : u64, is_string: bool, val : usize) -> bool {
        match self.search(key) {
            None => {
                let new = Box::new(Node::new(key, is_string, val as usize));
                self.inner[(key & HASH_MASK) as usize].push(Box::into_raw(new));
                true
            },
            Some(_) => false,
        }
    }

    fn search(&self, key : u64) -> Option<&mut Node> {
        let v = &self.inner[(key & HASH_MASK) as usize];
        for val in v.iter() {
            if unsafe{val.as_ref()}.unwrap().key == key {
                return unsafe { val.as_mut() } ;
            }
        }
        None
    }

    fn delete(&mut self, key : u64) -> bool{
        match self.search(key) {
            None => false,
            Some(n) => {
                drop(unsafe { Box::from_raw(n as &mut Node as *mut Node) } );
                true
            },
        }
    }

    fn print_all(&self) -> (){
        println("key -> value");
        println("---------------------");
        for i in 0..HASH_MASK {
            let v = &self.inner[(i & HASH_MASK) as usize];
            for val in v.iter() {
                print_u64(&unsafe{val.as_ref()}.unwrap().key);
                print(" -> ");
                unsafe{val.as_ref()}.unwrap().print();
            }
        }
    }
}

impl Node {
    fn new(key: u64, is_string: bool, ptr: usize) -> Self {
        Self{
            key: key,
            is_string: is_string,
            ptr : ptr
        }
    }

    fn print(&self) -> () {
        match self.get_as_str() {
            Some(s) => println(s),
            None => match self.get_as_u64() {
                Some(i) => println_u64(i),
                None => panic!("Fatal error while print Node"),
            },
        }
    }

    fn get_as_str(&self) -> Option<&String> {
        match self.is_string {
            true => unsafe { (self.ptr as *mut String).as_ref() },
            false => None
        }
    }

    fn get_as_u64(&self) -> Option<&u64> {
        match self.is_string {
            false => unsafe { (self.ptr as *mut u64).as_ref() } ,
            true => None
        }
    }

    fn clear(&mut self) -> () {
        match self.is_string {
            true => drop(unsafe{Box::from_raw(self.ptr as *mut String)} ),
            false => drop(unsafe{Box::from_raw(self.ptr as *mut u64)} ),
        }
    }

    fn update(&mut self, val: usize) -> () {
        match self.is_string {
            true => {
                self.clear();
                self.ptr = val; 
            },
            false => {
                unsafe { (self.ptr as *mut u64).write( val as u64) };
            },
        }        
    }
}

impl Drop for Node {
    fn drop(&mut self) {
        self.clear()
    }
}

impl Drop for KVStore {
    fn drop(&mut self) {
        for i in 0..HASH_MASK {
            let v = &self.inner[(i & HASH_MASK) as usize];
            for val in v.iter() {
                drop(unsafe { Box::from_raw(*val) } );
            }
        }
    }
}

fn print(s: &str){
    print!("{}", s);
    io::stdout().flush().ok().expect("Fail to flush stdout");
}

fn println(s: &str){
    println!("{}",s);
    io::stdout().flush().ok().expect("Fail to flush stdout");
}

fn print_u64(i: &u64){
    print!("{}",i);
    io::stdout().flush().ok().expect("Fail to flush stdout");
}

fn println_u64(i: &u64){
    println!("{}",i);
    io::stdout().flush().ok().expect("Fail to flush stdout");
}

fn print_menu() {
    println("1. Insert item");
    println("2. Search item");
    println("3. Update item");
    println("4. Delete item");
    println("5. View all item");
    println("6. exit");
    print(">> ");
}

fn read_u64(buf: &mut String) -> u64 {
    read_str(buf);

    let res = buf.trim().
    parse::<u64>().
    expect("Fail to read integer");

    buf.clear();

    res 
}

fn read_yn(buf: &mut String) -> bool {
    read_str(buf);

    let res = buf.trim().starts_with("Y");
    buf.clear();

    res 
}

fn read_str(buf: &mut String) -> () {
    io::stdin()
    .read_line(buf)
    .expect("Failed to read string");
}

fn read_line(buf : &mut Vec<u8>) -> () {
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    handle.read_until(b'\n', buf)
    .expect("Failed to read bytes");
}

fn main() {
    let mut input_buf = String::with_capacity(BUF_SIZE);
    let mut store = KVStore::new();

    println!("Welcome to Key-Value storage");
    loop {
        print_menu();
        
        match read_u64(&mut input_buf) {
            1 => {
                print("key >> ");
                let key = read_u64(&mut input_buf);
                print("is it String? (Y/N) >> ");
                let is_string =  read_yn(&mut input_buf);
                let val;
                match is_string {
                    true => { // String insert
                        print("size >> ");
                        let size = read_u64(&mut input_buf) as usize;
                        print(">> ");
                        let mut bytes = Vec::with_capacity(size);
                        read_line(&mut bytes);
                        let str = Box::leak(Box::new(unsafe{ String::from_utf8_unchecked(bytes) } ));
                        val = str as *mut String as usize;
                    }
                    false => { // Integer insert
                        print(">> ");
                        let num = Box::leak(Box::new(read_u64(&mut input_buf)));
                        val = num as *mut u64 as usize;
                    }
                }
                match store.insert(key, is_string, val){
                    true => println!("Successfully inserted!"),
                    false => {
                        println!("Insert fail...");
                        if is_string {
                            drop(unsafe { Box::from_raw(val as *mut String) } )
                        }
                        else {
                            drop(unsafe { Box::from_raw(val as *mut u64) } )
                        }
                    }
                }
            },
            2 => {
                print("key >> ");
                let key = read_u64(&mut input_buf);
                match store.search(key) {
                    Some(n) => n.print(),
                    None => println("Search fail...")
                }
            },
            3 => {
                print("key >> ");
                let key = read_u64(&mut input_buf);
                match store.search(key) {
                    Some(n) => {
                        if n.is_string {
                            print("size >> ");
                            let size = read_u64(&mut input_buf) as usize;
                            let mut str = Box::leak(Box::new(String::with_capacity(size)));
                            print(">> ");
                            read_str(&mut str);
                            n.update(str as *mut String as usize);
                        }
                        else {
                            print(">> ");
                            n.update(read_u64(&mut input_buf) as usize);
                        }
                    },
                    None => println("Update fail...")
                }
            },
            4 => {
                print("key >> ");
                let key = read_u64(&mut input_buf);
                match store.delete(key){
                    true => println!("Successfully deleted!"),
                    false => {
                        println!("Delete fail...");
                    }
                }
            }, 
            5 => {
                store.print_all();
            }
            6 => {
                println("Thank you for usage!");
                break;
            }
            _ => {
                println("Invalid menu");
            }
        }
    }
}
