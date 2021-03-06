use super::models::*;
use diesel::*;
use diesel::pg::upsert::*;
use connection::*;

pub fn save(connection: &DieselPgConnection, new_word: &NewWord) -> Result<Word, DBError> {
    insert(&new_word.on_conflict((words::text, words::language),
                                 do_update().set(new_word)))
        .into(words::table)
        .get_result::<Word>(connection)
}

//TODO insert_into instead of insert
//pub fn save(connection: &DieselPgConnection, new_word: NewWord) -> Result<Word, DBError> {
//    insert_into(words::table)
//        .values(&new_word)
//        .on_conflict((words::text, words::language))
//        .do_update()
//        .get_result::<Word>(connection)
//}

pub fn save_all(connection: &DieselPgConnection, new_words: Vec<NewWord>) -> Result<Vec<Word>, DBError> {
    insert(&new_words)
        .into(words::table)
        .get_results::<Word>(connection)
}

pub fn translate(connection: &DieselPgConnection, word_from: &Word, word_to: &Word) -> Result<Vec<Translation>, DBError> {
    let translations = word_from.new_translation(word_to);
    save_translations(connection, translations)
}

pub fn translate_all(connection: &DieselPgConnection, word_from: &Word, words_to: &Vec<Word>) -> Result<Vec<Translation>, DBError> {
    let translations = word_from.new_translations(words_to);
    save_translations(connection, translations)
}

pub fn find_by_text(connection: &DieselPgConnection, word_text: &str) -> Option<Word> {
    words::table
        .filter(words::text.eq(word_text))
        .first::<Word>(connection)
        .optional()
        .expect("Database error")
}

pub fn find_all_by_text(connection: &DieselPgConnection, word_text: &str) -> Vec<Word> {
    words::table
        .filter(words::text.eq(word_text))
        .load::<Word>(connection)
        .expect("Database error")
}

pub fn find_by_word(connection: &DieselPgConnection, word_from: &Word) -> Vec<TranslatedWord> {
    TranslatedWord::belonging_to(word_from)
        .load::<TranslatedWord>(connection)
        .expect("Error loading translated_words")
}

fn save_translations(connection: &DieselPgConnection, translations: Vec<NewTranslation>) -> Result<Vec<Translation>, DBError> {
    let result_translations: Result<Vec<Translation>, DBError> = translations.iter()
        .map(|translation| save_translation(connection, translation))
        .collect();

    result_translations
}

fn save_translation(connection: &DieselPgConnection, translation: &NewTranslation) -> Result<Translation, DBError> {
    insert(&translation
        .on_conflict((translations::word_from, translations::word_to),
                     do_update().set(translation)))
        .into(translations::table)
        .get_result::<Translation>(connection)
}