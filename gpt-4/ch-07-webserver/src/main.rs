use actix_cors::Cors;
use actix_web::{http::header, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::sync::Mutex;

// #[derive(Serialize, Deserialize, Debug, Clone)]
// struct GameState {
//     id: u64,
//     word: String,
//     guessed_letters: Vec<char>,
//     incorrect_attempts: u8,
//     last_move: String,
// }

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Task {
    id: u64,
    name: String,
    completed: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct User {
    id: u64,
    username: String,
    password: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Database {
    tasks: HashMap<u64, Task>,
    users: HashMap<u64, User>,
}

impl Database {
    fn new() -> Self {
        Self {
            tasks: HashMap::new(),
            users: HashMap::new(),
        }
    }

    fn insert(&mut self, task: Task) {
        self.tasks.insert(task.id, task);
    }

    fn insert_user(&mut self, user: User) {
        self.users.insert(user.id, user);
    }

    fn get(&self, id: &u64) -> Option<&Task> {
        self.tasks.get(id)
    }

    fn get_user_by_name(&self, username: &str) -> Option<&User> {
        self.users.values().find(|u| u.username == username)
    }

    fn get_all(&self) -> Vec<&Task> {
        self.tasks.values().collect()
    }

    fn get_users_all(&self) -> Vec<&User> {
        self.users.values().collect()
    }

    fn delete(&mut self, id: &u64) {
        self.tasks.remove(id);
    }

    fn delete_user_by_id(&mut self, id: &u64) {
        self.users.remove(id);
    }

    fn update(&mut self, task: Task) {
        self.tasks.insert(task.id, task);
    }

    fn update_user_by_id(&mut self, user: User) {
        self.users.insert(user.id, user);
    }
}

fn main() {
    println!("Hello, world!");
}
