use cards_core::models::*;
use cards_core::repository::*;
use diesel::Connection;
use diesel::pg::PgConnection;

pub type TestConnection = PgConnection;

pub fn establish() -> TestConnection {
    let result = connection_without_transaction();
    result.begin_test_transaction().unwrap();
    result
}

pub fn connection_without_transaction() -> TestConnection {
    TestConnection::establish("postgres://keyridan:pandaPass@localhost/postgres").unwrap()
}

#[test]
fn test_connection() {
    establish();
}


pub fn saved_words_to(connection: &TestConnection) -> Vec<Word> {
    let test_words_to = vec![german_word(), english_word()];
    save_all(connection, test_words_to).unwrap()
}

pub fn saved_word_from(connection: &TestConnection) -> Word {
    let test_word_from = test_new_word();
    save(&connection, test_word_from).unwrap()
}

pub fn test_new_words_to<'a>() -> Vec<NewWord<'a>> {
    vec!(german_word(), english_word())
}

pub fn test_new_word<'a>() -> NewWord<'a> {
    NewWord {
        text: "экзамен",
        language: "RU",
        sex: Some("M".to_string()),
    }
}

pub fn german_word<'a>() -> NewWord<'a> {
    NewWord {
        text: "Prüfung",
        language: "DE",
        sex: Some("F".to_string()),
    }
}

pub fn english_word<'a>() -> NewWord<'a> {
    NewWord {
        text: "exam",
        language: "EN",
        sex: None
    }
}