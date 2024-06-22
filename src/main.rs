use std::vec;


// TODO: implement id logic: from/to methods etc
trait Id {}
struct UserId {}
struct ExpenseId {}
// -----------
struct User {
    id: Option<UserId>,
    name: String,
}
struct Expense {
    id: Option<ExpenseId>,
    owner_id: UserId,
    value: u32,
    users: Vec<UserId>
}
struct Group {
    users: Vec<User>,
    expenses: Vec<Expense>
}

impl User {
    fn new(name: String) -> Self {
        User { id: None, name }
    }
}

impl Expense {
    fn new(value: u32, owner_id: UserId, users: Vec<UserId>) -> Self {
        Expense{ id: None, owner_id, value, users }
    }
}

impl Group {
    fn new() -> Self {
        Group { users: vec![], expenses: vec![] }
    }
    fn add_user(&mut self, user: User) {
        self.users.push(user);
    }
    fn add_expense(&mut self, expense: Expense) {
        self.expenses.push(expense);
    }
}

fn main() {

    let lucca = User::new("lucca".to_string());
    let yvens = User::new("yvens".to_string());
    
    let mut yvensiveis = Group::new();
    yvensiveis.add_user(lucca);
    yvensiveis.add_user(yvens);

    let bardana = Expense::new(90, 0,vec![0, 1]);

    yvensiveis.add_expense(bardana);
}
