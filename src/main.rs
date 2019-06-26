use std::process;
use std::io::{Write, BufRead, BufReader,BufWriter};
use std::fs::File;

struct Profile {
    id: u32,
    name: String,
    birthday: String,
    home: String,
    comment: String,
}

impl Profile {
    fn print(&self) {
        println!("ID:       {}", self.id);
        println!("Name:     {}", self.name);
        println!("Birthday: {}", self.birthday);
        println!("Addr:     {}", self.home);
        println!("Note:     {}", self.comment);
     }
}
 
fn cmd_quit(){
    process::exit(0);
}

fn cmd_check(profile_data_store: &mut Vec<Profile>){
    println!("{}",profile_data_store.len());
}

fn cmd_print(nitems:i32, profile_data_store: &mut Vec<Profile>){
    if nitems >= 0 {
        for val in profile_data_store.iter().take(nitems as usize) {
            val.print();
        }

    }else{
        for val in profile_data_store.iter().rev().take(nitems.abs() as usize){
            val.print();
        }
    }
}

fn cmd_read(filename: String,profile_data_store: &mut Vec<Profile>){
    let file = File::open(filename).expect("cannot open file");
    let file = BufReader::new(file);
    for line in file.lines().filter_map(|result| result.ok()) {
        parse_line(line,profile_data_store);
    } 
}
    
fn cmd_write(filename: String,profile_data_store: &mut Vec<Profile>){
    let file = File::create(filename).unwrap();
    let mut f = BufWriter::new(file);

    for profile in profile_data_store.iter() {
    write!(f,"{},{},{},{},{}\n",profile.id, profile.name, profile.birthday, profile.home, profile.comment).unwrap(); 
    f.flush().unwrap();
    }
}

fn cmd_find(word:String, profile_data_store: &mut Vec<Profile>){
    for profile in profile_data_store.iter() {
        if profile.id == word.parse().unwrap()
         || profile.name == word
         || profile.birthday == word
         || profile.home == word
         || profile.comment == word {
         profile.print();
         }
    }
}
    
fn cmd_sort(n:i32, profile_data_store: &mut Vec<Profile>){
    match n {
        1 => profile_data_store.sort_by_key(|x| x.id),
        2 => profile_data_store.sort_by(|a, b| a.name.cmp(&b.name)),
        3 => profile_data_store.sort_by(|a, b| a.birthday.cmp(&b.birthday)),
        4 => profile_data_store.sort_by(|a, b| a.home.cmp(&b.home)),
        5 => profile_data_store.sort_by(|a, b| a.comment.cmp(&b.comment)),
        _ => println!("ERROR"),
    }
}
   
// コマンド判定
fn exec_command(line: String, profile_data_store: &mut Vec<Profile>){
    let param: Vec<&str> = line.trim().split(' ').collect();
    match line.chars().nth(1) {
        Some('Q') => cmd_quit(),
        Some('C') => cmd_check(profile_data_store),
        Some('P') => cmd_print(param[1].parse::<i32>().unwrap(),profile_data_store),
        Some('R') => cmd_read(param[1].to_string(),profile_data_store),
        Some('W') => cmd_write(param[1].to_string(),profile_data_store),
        Some('F') => cmd_find(param[1].to_string(),profile_data_store),
        Some('S') => cmd_sort(param[1].parse::<i32>().unwrap(),profile_data_store),
        Some(_) => println!("No such command"),
        None => println!("None"),
   }
}
    
// 登録
fn new_profile(line: String, profile_data_store: &mut Vec<Profile>)  {
    let v: Vec<&str> = line.trim().split(',').collect();

    let profile = Profile{
        id:       v[0].parse().unwrap(),
        name:     v[1].to_string(),
        birthday: v[2].to_string(), 
        home:     v[3].to_string(),
        comment:  v[4].to_string(),
    };
    
    profile_data_store.push(profile);
}

// command or csv data 判定
fn parse_line(line: String, profile_data_store: &mut Vec<Profile>)  {
    if line.starts_with("%") {
        exec_command(line, profile_data_store);
    }else{
        new_profile(line, profile_data_store);
    }
}

fn main() {
    let mut profile_data_store: Vec<Profile> = Vec::new();
    loop {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).ok();
    
        parse_line(line, &mut profile_data_store);
    }
}
