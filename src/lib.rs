#![feature(const_fn)]
use state::Storage;
use std::fs;
use std::sync::RwLock;

pub struct StateList<T: Send + Sync + 'static + std::cmp::PartialEq + std::clone::Clone> {
    list_file_path: Storage<&'static str>,
    state_list: Storage<RwLock<Vec<T>>>,
}

impl<T: Send + Sync + 'static + std::cmp::PartialEq + std::clone::Clone> StateList<T> {
    pub const fn new() -> StateList<T> {
        StateList {
            list_file_path: Storage::new(),
            state_list: Storage::new(),
        }
    }

    pub fn init_empty(&self) {
        self.state_list.set(RwLock::new(Vec::<_>::new()));
    }

    pub fn retain_matching(&self, match_fn: fn(&T) -> bool) {
        let write_list = self.state_list.get();
        match write_list.try_write() {
            Ok(mut list) => {
                list.retain(match_fn);
            }
            Err(e) => {
                println!("error adding to list: {}", e);
            }
        };
    }

    pub fn add_item(&self, item: T) {
        let write_list = self.state_list.get();
        match write_list.try_write() {
            Ok(mut list) => {
                &list.push(item);
            }
            Err(e) => {
                println!("error adding to list: {}", e);
            }
        };
    }

    pub fn contains(&self, item: &T) -> bool {
        let read_list = self.state_list.get();
        return match read_list.try_read() {
            Ok(list) => list.contains(item),
            Err(_) => false,
        };
    }

    pub fn length(&self) -> usize {
        let read_list = self.state_list.get();
        match read_list.try_read() {
            Ok(list) => {
                return (*list).len();
            }
            Err(_) => {
                return 0;
            }
        }
    }

    pub fn pop(&self) -> Option<T> {
        let write_list = self.state_list.get();
        match write_list.try_write() {
            Ok(mut list) => {
                return (*list).pop();
            }
            Err(_) => {
                return None;
            }
        }
    }

    pub fn get_entries(&self) -> Vec<T> {
        let mut vecy = Vec::<T>::new();
        let read_list = self.state_list.get();
        match read_list.try_read() {
            Ok(list) => {
                vecy = (*list).to_vec();
            }
            Err(_) => {}
        }
        return vecy;
    }

    pub fn sort_dedup_list(&self, sorting: fn(&T, &T) -> std::cmp::Ordering) {
        let write_list = self.state_list.get();
        match write_list.try_write() {
            Ok(mut list) => {
                list.sort_by(sorting);
                list.dedup();
            }
            Err(_) => println!("No access to state list."),
        }
    }
}

impl<T: Send + Sync + 'static
        + std::cmp::PartialEq
        + std::clone::Clone
        + std::fmt::Display
        + std::str::FromStr,
    > StateList<T>
{
    pub fn load(&self, file_path: &'static str) {
        let rules = fs::read_to_string(file_path).unwrap_or_else(|e| {
            println!("Couldn't load file for list: {} {}", file_path, e);
            return String::new();
        });
        self.list_file_path.set(file_path);

        let mut write_list = Vec::<T>::new();
        for line in rules.split("\n") {
            if line.trim() != "" {
                match line.trim().parse() {
                    Ok(item) => write_list.push(item),
                    Err(_) => println!("can't parse item from: {}", line),
                }
            }
        }
        self.state_list.set(RwLock::new(write_list));
    }

    pub fn save_matching(&self, match_fn: fn(&T) -> bool) {
        let write_list = self.state_list.get();
        let mut rules = String::new();
        match write_list.try_write() {
            Ok(list) => {
                for item in &*list {
                    if match_fn(item) {
                        rules.push_str(format!("{}\n", item).as_str());
                    }
                }
            }
            Err(_) => println!("No access to state list."),
        };
        fs::write(self.list_file_path.get(), rules.as_bytes()).unwrap_or_else(|e| {
            println!(
                "Couldn't save IGNORE_FILE: {} {}",
                self.list_file_path.get(),
                e
            );
        });
    }

    pub fn save_state(&self) {
        self.save_matching(|_| true);
    }
}
