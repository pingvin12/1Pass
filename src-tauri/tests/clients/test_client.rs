use app::db::commands;

#[derive(Clone, Debug)]
struct TestClientAuthCredentials {
    pub email: String,
    pub username: String,
    pub password: String,
}

pub struct TestClient {
    credentials: TestClientAuthCredentials,
    token: String,
}

impl TestClient {
    pub fn new() -> Self {

        let credentials = &TestClientAuthCredentials {
            email: "test@test.com".to_owned(),
            username: "test".to_owned(),
            password: "test".to_owned(),
        };

        let token: String = match commands::auth(credentials.email.clone(), credentials.password.clone()) {
            Ok(token) => token,
            Err(_err) => {
                println!("Error while authenticating");
                println!("Creating user...");
                match commands::register(credentials.username.clone(), credentials.password.clone(), credentials.email.clone()) {
                    Ok(_res) => {
                        println!("User created");
                        println!("Authenticating...");
                        match commands::auth(credentials.email.clone(), credentials.password.clone()) {
                            Ok(token) => token,
                            Err(_err) => {
                                println!("Error while authenticating");
                                panic!("Something went wrong while creating user, and authenticating...");
                            }
                        }
                    },
                    Err(_err) => {
                        println!("Error while creating user");
                        panic!();
                    }
                }
            }
        };
            

        Self {
            credentials: credentials.clone(),
            token: token,
        }
    }

    pub fn get_token(&self) -> String {
        self.token.clone()
    }

    pub fn clean_client(&self) {
        commands::clean_server();
    }


}