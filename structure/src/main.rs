struct User {
    username: String,
    email: String,
    active: bool,
    sign_in_count: u64
}

impl User {
    fn active_user(&mut self)
    {
        self.active = true;
    }

    fn sign_in_count_inc(&mut self)
    {
        self.sign_in_count += 1;
    }
}

impl User {
    fn build_user(username: &str, email: &str) -> User
    {
        // username 和 email 使用了 'field init shorthand syntax'
        User {
            username: String::from(username),
            email: String::from(email),
            active: false, // account not active
            sign_in_count: 0 // account not signed in
        }
    }

    fn is_same_user(user1: &User, user2: &User) -> bool
    {
        user1.username == user2.username && user1.email == user2.email
    }
}

fn main() {
    let mut user1 = User::build_user("kevin123", "kevin123@gmail.com");
    let mut user2 = User::build_user("mike123", "mike123@gmail.com");

    println!("is user1 active: {}", user1.active);
    user1.active_user();
    println!("is user1 active: {}", user1.active);

    println!("user2 sign in count: {}", user2.sign_in_count);
    user2.sign_in_count_inc();
    println!("user2 sign in count: {}", user2.sign_in_count);

    println!("is user1 and user2 same user: {}", User::is_same_user(&user1, &user2));

    let user3 = User {
        username: String::from("mike123"),
        email: String::from("mike123@gmail.com"),
        ..user2
    };

    println!("is user2 and user3 same user: {}", User::is_same_user(&user2, &user3));
}
