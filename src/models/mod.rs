pub mod deck_components {
    use crate::utils::CardState;
    use serde::{Deserialize, Serialize};
    use uuid::Uuid;

    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
    pub struct Card {
        pub id: String,
        pub front: String,
        pub back: String,
        pub state: CardState,
        pub interval: u32,
        pub intensity: u32,
    }

    impl Card {
        pub fn new(front: String, back: String) -> Self {
            Card {
                id: Uuid::new_v4().to_string(),
                front,
                back,
                state: CardState::NEW,
                interval: 1,
                intensity: 1,
            }
        }
    }

    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
    pub struct Deck {
        pub id: String,
        pub owner_id: String,
        pub name: String,
        pub cards: Vec<Card>,
    }

    impl Deck {
        pub fn new(owner_id: String, name: String, cards: Vec<Card>) -> Self {
            Deck {
                id: Uuid::new_v4().to_string(),
                owner_id,
                name,
                cards,
            }
        }
    }
}

pub mod user_components {
    use serde::{Deserialize, Serialize};
    use uuid::Uuid;

    #[derive(Serialize, Deserialize)]
    pub struct User {
        pub id: String,
        pub name: String,
        pub email: String,
        pub decks: Vec<String>,
    }

    impl User {
        pub fn new(name: String, email: String) -> Self {
            User {
                id: Uuid::new_v4().to_string(),
                name,
                email,
                decks: Vec::new(),
            }
        }
    }
}
