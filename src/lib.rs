use state::Storage;
use std::fs;
use std::sync::RwLock;

pub struct StateList {
    list_file_path: Storage<&'static str>,
    state_list: Storage<RwLock<Vec<String>>>,
}

impl StateList {
    pub const fn new() -> StateList {
        StateList {
            list_file_path: Storage::new(),
            state_list: Storage::new(),
        }
    }

    pub fn init_empty(&self) {
        self.state_list.set(RwLock::new(Vec::<_>::new()));
    }

    pub fn load(&self, file_path: &'static str) {
        let rules = fs::read_to_string(file_path).unwrap_or_else(|e| {
            println!("Couldn't load file for list: {} {}", file_path, e);
            return String::new();
        });
        self.list_file_path.set(file_path);

        let mut write_list = Vec::<_>::new();
        for line in rules.split("\n") {
            if line.trim() != "" {
                write_list.push(line.trim().to_string());
            }
        }
        self.state_list.set(RwLock::new(write_list));
    }

    pub fn retain_matching(&self, match_fn: fn(&String) -> bool) {
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

    pub fn add_item(&self, item: String) {
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

    pub fn contains(&self, item: &String) -> bool {
        let read_list = self.state_list.get();
        return match read_list.try_read() {
            Ok(list) => list.contains(item),
            Err(_) => false,
        };
    }

    pub fn get_entries(&self) -> Vec<String> {
        let mut vecy = Vec::<String>::new();
        let read_list = self.state_list.get();
        match read_list.try_read() {
            Ok(list) => {
                for item in &*list {
                    vecy.push((*item).as_str().to_string());
                }
            }
            Err(_) => {}
        }
        return vecy;
    }

    pub fn dot_reverse(str: &String) -> String {
        let mut split = str.split('.').collect::<Vec<&str>>();
        split.reverse();
        return split.join(".");
    }

    pub fn sort_dedup_list(&self){
        let write_list = self.state_list.get();
        match write_list.try_write() {
            Ok(mut list) => {
                list.sort_by({
                    |a, b| 
                    StateList::dot_reverse(a).cmp(&StateList::dot_reverse(b))
                });
                list.dedup();

            },
            Err(_) => println!("No access to state list."),
        }

    }

    pub fn save_matching(&self, match_fn: fn(&String) -> bool) {
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
