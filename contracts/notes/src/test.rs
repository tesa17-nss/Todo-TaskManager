#[cfg(test)]
mod tests {
    use crate::{TodoContract, TodoContractClient, Priority, Status};
    use soroban_sdk::{testutils::Address as _, Address, Env, String};

    fn setup() -> (Env, TodoContractClient<'static>, Address) {
        let env = Env::default();
        env.mock_all_auths();

        let contract_id = env.register_contract(None, TodoContract);
        let client = TodoContractClient::new(&env, &contract_id);

        let admin = Address::generate(&env);
        client.init(&admin);

        (env, client, admin)
    }

    #[test]
    fn test_init_twice() {
        let (env, client, _admin) = setup();
        let admin2 = Address::generate(&env);
        let result = client.init(&admin2);
        assert_eq!(result, String::from_str(&env, "Kontrak sudah diinisialisasi"));
    }


    #[test]
    fn test_create_todo() {
        let (env, client, _admin) = setup();

        let result = client.create_todo(
            &String::from_str(&env, "Belajar Soroban"),
            &String::from_str(&env, "Pelajari cara deploy smart contract"),
            &Priority::High,
        );

        assert_eq!(result, String::from_str(&env, "Todo berhasil dibuat"));

        let todos = client.get_todos();
        assert_eq!(todos.len(), 1);
        assert_eq!(todos.get(0).unwrap().id, 1);
    }

    #[test]
    fn test_get_todo_by_id() {
        let (env, client, _admin) = setup();

        client.create_todo(
            &String::from_str(&env, "Task A"),
            &String::from_str(&env, "Deskripsi A"),
            &Priority::Low,
        );

        let todo = client.get_todo(&1u64);
        assert!(todo.is_some());
        assert_eq!(todo.unwrap().title, String::from_str(&env, "Task A"));
    }

    #[test]
    fn test_get_todo_not_found() {
        let (_env, client, _admin) = setup();
        let todo = client.get_todo(&999u64);
        assert!(todo.is_none());
    }

    #[test]
    fn test_update_status() {
        let (env, client, _admin) = setup();

        client.create_todo(
            &String::from_str(&env, "Task B"),
            &String::from_str(&env, "Deskripsi B"),
            &Priority::Medium,
        );

        let result = client.update_status(&1u64, &Status::InProgress);
        assert_eq!(result, String::from_str(&env, "Status berhasil diperbarui"));

        let todo = client.get_todo(&1u64).unwrap();
        assert_eq!(todo.status, Status::InProgress);
    }

    #[test]
    fn test_update_priority() {
        let (env, client, _admin) = setup();

        client.create_todo(
            &String::from_str(&env, "Task C"),
            &String::from_str(&env, "Deskripsi C"),
            &Priority::Low,
        );

        let result = client.update_priority(&1u64, &Priority::High);
        assert_eq!(result, String::from_str(&env, "Priority berhasil diperbarui"));

        let todo = client.get_todo(&1u64).unwrap();
        assert_eq!(todo.priority, Priority::High);
    }

    #[test]
    fn test_delete_todo() {
        let (env, client, _admin) = setup();

        client.create_todo(
            &String::from_str(&env, "Task D"),
            &String::from_str(&env, "Deskripsi D"),
            &Priority::High,
        );

        assert_eq!(client.get_todos().len(), 1);

        let result = client.delete_todo(&1u64);
        assert_eq!(result, String::from_str(&env, "Todo berhasil dihapus"));
        assert_eq!(client.get_todos().len(), 0);
    }

    #[test]
    fn test_delete_not_found() {
        let (env, client, _admin) = setup();
        let result = client.delete_todo(&999u64);
        assert_eq!(result, String::from_str(&env, "Todo tidak ditemukan"));
    }

    #[test]
    fn test_id_sequential() {
        let (env, client, _admin) = setup();

        for i in 1u64..=3 {
            client.create_todo(
                &String::from_str(&env, "Task"),
                &String::from_str(&env, "Desc"),
                &Priority::Low,
            );
            let todos = client.get_todos();
            assert_eq!(todos.get((i - 1) as u32).unwrap().id, i);
        }
    }
}