#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracttype,
    symbol_short, Env, String, Symbol, Vec, Address,
};

#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub enum Priority {
    Low,
    Medium,
    High,
}

#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub enum Status {
    Pending,
    InProgress,
    Done,
}


#[contracttype]
#[derive(Clone, Debug)]
pub struct Todo {
    pub id: u64,
    pub title: String,
    pub description: String,
    pub priority: Priority,
    pub status: Status,
    pub created_at: u64, 
}


const TODO_DATA: Symbol = symbol_short!("TODO_DATA");
const ADMIN_KEY: Symbol = symbol_short!("ADMIN");
const COUNTER:   Symbol = symbol_short!("COUNTER");


#[contract]
pub struct TodoContract;

#[contractimpl]
impl TodoContract {

    pub fn init(env: Env, admin: Address) -> String {

        if env.storage().instance().has(&ADMIN_KEY) {
            return String::from_str(&env, "Kontrak sudah diinisialisasi");
        }

        env.storage().instance().set(&ADMIN_KEY, &admin);
        env.storage().instance().set(&COUNTER, &0u64);

        String::from_str(&env, "Kontrak berhasil diinisialisasi")
    }

    fn get_admin(env: &Env) -> Address {
        env.storage().instance().get(&ADMIN_KEY).unwrap()
    }

    pub fn get_todos(env: Env) -> Vec<Todo> {
        env.storage()
            .instance()
            .get(&TODO_DATA)
            .unwrap_or(Vec::new(&env))
    }

    pub fn get_todo(env: Env, id: u64) -> Option<Todo> {
        let todos: Vec<Todo> = env
            .storage()
            .instance()
            .get(&TODO_DATA)
            .unwrap_or(Vec::new(&env));

        for i in 0..todos.len() {
            let todo = todos.get(i).unwrap();
            if todo.id == id {
                return Some(todo);
            }
        }
        None
    }

    pub fn create_todo(
        env: Env,
        title: String,
        description: String,
        priority: Priority,
    ) -> String {
        let admin = Self::get_admin(&env);
        admin.require_auth();

        let mut todos: Vec<Todo> = env
            .storage()
            .instance()
            .get(&TODO_DATA)
            .unwrap_or(Vec::new(&env));

        let mut counter: u64 = env
            .storage()
            .instance()
            .get(&COUNTER)
            .unwrap_or(0u64);
        counter += 1;

        let todo = Todo {
            id: counter,
            title,
            description,
            priority,
            status: Status::Pending, // default status saat dibuat
            created_at: env.ledger().timestamp(),
        };

        todos.push_back(todo);

        env.storage().instance().set(&TODO_DATA, &todos);
        env.storage().instance().set(&COUNTER, &counter);

        String::from_str(&env, "Todo berhasil dibuat")
    }

    pub fn update_status(env: Env, id: u64, new_status: Status) -> String {
        let admin = Self::get_admin(&env);
        admin.require_auth();

        let mut todos: Vec<Todo> = env
            .storage()
            .instance()
            .get(&TODO_DATA)
            .unwrap_or(Vec::new(&env));

        for i in 0..todos.len() {
            let mut todo = todos.get(i).unwrap();
            if todo.id == id {
                todo.status = new_status;
                todos.set(i, todo);
                env.storage().instance().set(&TODO_DATA, &todos);
                return String::from_str(&env, "Status berhasil diperbarui");
            }
        }

        String::from_str(&env, "Todo tidak ditemukan")
    }

    pub fn update_priority(env: Env, id: u64, new_priority: Priority) -> String {
        let admin = Self::get_admin(&env);
        admin.require_auth();

        let mut todos: Vec<Todo> = env
            .storage()
            .instance()
            .get(&TODO_DATA)
            .unwrap_or(Vec::new(&env));

        for i in 0..todos.len() {
            let mut todo = todos.get(i).unwrap();
            if todo.id == id {
                todo.priority = new_priority;
                todos.set(i, todo);
                env.storage().instance().set(&TODO_DATA, &todos);
                return String::from_str(&env, "Priority berhasil diperbarui");
            }
        }

        String::from_str(&env, "Todo tidak ditemukan")
    }

    pub fn delete_todo(env: Env, id: u64) -> String {
        let admin = Self::get_admin(&env);
        admin.require_auth();

        let mut todos: Vec<Todo> = env
            .storage()
            .instance()
            .get(&TODO_DATA)
            .unwrap_or(Vec::new(&env));

        for i in 0..todos.len() {
            if todos.get(i).unwrap().id == id {
                todos.remove(i);
                env.storage().instance().set(&TODO_DATA, &todos);
                return String::from_str(&env, "Todo berhasil dihapus");
            }
        }

        String::from_str(&env, "Todo tidak ditemukan")
    }
}

mod test;