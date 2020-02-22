use state::Storage;
use std::fs;
use std::sync::RwLock;

static LIST_FILE_PATH: Storage<&str> = Storage::new();
static STATE_LIST: Storage<RwLock<Vec<String>>> = Storage::new();

pub fn load(file_path: &'static str) {
    let rules = fs::read_to_string(file_path).unwrap_or_else(|e| {
        println!("Couldn't load file for list: {} {}", file_path, e);
        return String::new();
    });
    LIST_FILE_PATH.set(file_path);

    let mut write_list = Vec::<_>::new();
    for line in rules.split("\n") {
        if line.trim() != "" {
            write_list.push(line.trim().to_string());
        }
    }
    STATE_LIST.set(RwLock::new(write_list.clone()));
}

pub fn add_item(item: String) {
    let write_list = STATE_LIST.get();
    match write_list.try_write() {
        Ok(mut list) => {
            &list.push(item);
        }
        Err(e) => {
            println!("error adding to list: {}", e);
        }
    };
}

pub fn contains(item: &String) -> bool {
    let read_list = STATE_LIST.get();
    return match read_list.try_read() {
        Ok(list) => list.contains(item),
        Err(_) => false,
    };
}

pub fn get_entries() -> Vec<String>{
    let mut vecy = Vec::<String>::new();

    let read_list = STATE_LIST.get();
    match read_list.try_read() {
        Ok(list) => {
            for item in &*list {
                vecy.push((*item).as_str().to_string());
            }
        },
        Err(_) => {},
    }
    return vecy;
}

pub fn get_list() -> &'static Storage<RwLock<Vec<String>>> {
    return &STATE_LIST;
}

pub fn save_state() {
    let write_list = STATE_LIST.get();
    let mut rules = String::new();
    match write_list.try_write() {
        Ok(list) => {
            for ignore in &*list {
                rules.push_str(format!("{}\n", ignore).as_str());
            }
        }
        Err(_) => println!("No access to ignore list."),
    };

    fs::write(LIST_FILE_PATH.get(), rules.as_bytes()).unwrap_or_else(|e| {
        println!("Couldn't save IGNORE_FILE: {} {}", LIST_FILE_PATH.get(), e);
    });
}
