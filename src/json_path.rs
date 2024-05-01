const JSON_TEST: &str = r#"
{ "store": {
    "book": [ 
      { "category": "reference",
        "author": "Nigel Rees",
        "title": "Sayings of the Century",
        "price": 8.95
      },
      { "category": "fiction",
        "author": "Evelyn Waugh",
        "title": "Sword of Honour",
        "price": 12.99
      },
      { "category": "fiction",
        "author": "Herman Melville",
        "title": "Moby Dick",
        "isbn": "0-553-21311-3",
        "price": 8.99
      },
      { "category": "fiction",
        "author": "J. R. R. Tolkien",
        "title": "The Lord of the Rings",
        "isbn": "0-395-19395-8",
        "price": 22.99
      }
    ],
    "bicycle": {
      "color": "red",
      "price": 19.95
    }
  }
}"#;

#[derive(Debug)]
pub struct JsonPath {
    pub body: String,
}

impl JsonPath {
    pub fn new(payload: String) -> JsonPath {
        JsonPath { body: payload }
    }

    pub fn find(&self, jsonPath: &str) -> Vec<String> {
        vec![String::new()]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_authors_of_all_books() {
        let input = JsonPath::new(JSON_TEST.to_string());

        assert_eq!(
            input.find("$.store.book[*].author"),
            vec![
                String::from("Nigel Rees"),
                String::from("Evelyn Waugh"),
                String::from("Herman Melville"),
                String::from("J. R. R. Tolkien")
            ]
        );
    }

    #[test]
    fn all_authors() {
        let input = JsonPath::new(JSON_TEST.to_string());

        assert_eq!(
            input.find("$.store.book[*].author"),
            vec![
                String::from("Nigel Rees"),
                String::from("Evelyn Waugh"),
                String::from("Herman Melville"),
                String::from("J. R. R. Tolkien")
            ]
        );
    }

    #[test]
    fn all_store_items() {
        let input = JsonPath::new(JSON_TEST.to_string());

        assert_eq!(
            input.find("$.store.book[*].author"),
            vec![
                String::from("Nigel Rees"),
                String::from("Evelyn Waugh"),
                String::from("Herman Melville"),
                String::from("J. R. R. Tolkien")
            ]
        );
    }

    #[test]
    fn all_prices() {
        let input = JsonPath::new(JSON_TEST.to_string());

        assert_eq!(
            input.find("$.store.book[*].author"),
            vec![
                String::from("Nigel Rees"),
                String::from("Evelyn Waugh"),
                String::from("Herman Melville"),
                String::from("J. R. R. Tolkien")
            ]
        );
    }

    #[test]
    fn third_book() {
        let input = JsonPath::new(JSON_TEST.to_string());

        assert_eq!(
            input.find("$.store.book[*].author"),
            vec![
                String::from("Nigel Rees"),
                String::from("Evelyn Waugh"),
                String::from("Herman Melville"),
                String::from("J. R. R. Tolkien")
            ]
        );
    }

    #[test]
    fn last_book_in_order_1() {
        let input = JsonPath::new(JSON_TEST.to_string());

        assert_eq!(
            input.find("$.store.book[*].author"),
            vec![
                String::from("Nigel Rees"),
                String::from("Evelyn Waugh"),
                String::from("Herman Melville"),
                String::from("J. R. R. Tolkien")
            ]
        );
    }

    #[test]
    fn last_book_in_order_2() {
        let input = JsonPath::new(JSON_TEST.to_string());

        assert_eq!(
            input.find("$.store.book[*].author"),
            vec![
                String::from("Nigel Rees"),
                String::from("Evelyn Waugh"),
                String::from("Herman Melville"),
                String::from("J. R. R. Tolkien")
            ]
        );
    }

    #[test]
    fn first_two_books_1() {
        let input = JsonPath::new(JSON_TEST.to_string());

        assert_eq!(
            input.find("$.store.book[*].author"),
            vec![
                String::from("Nigel Rees"),
                String::from("Evelyn Waugh"),
                String::from("Herman Melville"),
                String::from("J. R. R. Tolkien")
            ]
        );
    }

    #[test]
    fn first_two_books_2() {
        let input = JsonPath::new(JSON_TEST.to_string());

        assert_eq!(
            input.find("$.store.book[*].author"),
            vec![
                String::from("Nigel Rees"),
                String::from("Evelyn Waugh"),
                String::from("Herman Melville"),
                String::from("J. R. R. Tolkien")
            ]
        );
    }

    #[test]
    fn all_books_with_isbn() {
        let input = JsonPath::new(JSON_TEST.to_string());

        assert_eq!(
            input.find("$.store.book[*].author"),
            vec![
                String::from("Nigel Rees"),
                String::from("Evelyn Waugh"),
                String::from("Herman Melville"),
                String::from("J. R. R. Tolkien")
            ]
        );
    }

    #[test]
    fn all_books_cheaper_than() {
        let input = JsonPath::new(JSON_TEST.to_string());

        assert_eq!(
            input.find("$.store.book[*].author"),
            vec![
                String::from("Nigel Rees"),
                String::from("Evelyn Waugh"),
                String::from("Herman Melville"),
                String::from("J. R. R. Tolkien")
            ]
        );
    }

    #[test]
    fn all_elements() {
        let input = JsonPath::new(JSON_TEST.to_string());

        assert_eq!(
            input.find("$.store.book[*].author"),
            vec![
                String::from("Nigel Rees"),
                String::from("Evelyn Waugh"),
                String::from("Herman Melville"),
                String::from("J. R. R. Tolkien")
            ]
        );
    }
}
