pub fn main() {
    lazy_eval_const::main();
}

mod lazy_eval_const {

    use lazy_static::lazy_static;
    use std::collections::HashMap;

    lazy_static! {
        static ref PRIVILEGES: HashMap<&'static str, Vec<&'static str>> = {
            let mut map = HashMap::new();
            map.insert("James", vec!["user", "admin"]);
            map.insert("Jim", vec!["user"]);
            map
        };
    }

    fn show_access(name: &str) {
        let access = PRIVILEGES.get(name);
        println!("{}: {:?}", name, access);
    }

    pub fn main() {
        let access = PRIVILEGES.get("James");
        println!("James: {:?}", access);

        show_access("Jim");
    }
}
