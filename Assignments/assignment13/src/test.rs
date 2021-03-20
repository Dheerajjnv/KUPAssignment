#[cfg(test)]
#[test]
fn add_book_success() {
    use crate::library_menu::Books;
    let mut books = Books {
        title: vec![
            "winner stand alone".to_string(),
            "The alchemist".to_string(),
        ],
        author: vec!["paulo".to_string(), "coelho".to_string()],
        accession_number: vec![1, 2],
        flag: 0,
    };
    let input = books.add_book("Hey God".to_string(), "James carry".to_string(), 6, 1);
    assert_eq!(input, ())
}
#[test]
fn book_display_success() {
    use crate::library_menu::Books;
    let mut books = Books {
        title: vec![
            "winner stand alone".to_string(),
            "The alchemist".to_string(),
        ],
        author: vec!["paulo".to_string(), "coelho".to_string()],
        accession_number: vec![1, 2],
        flag: 0,
    };
    let output = books.book_display();
    assert_eq!(output, Ok("Data displayed".to_string()))
}
#[test]
fn book_display_failure() {
    use crate::library_menu::Books;
    let mut books = Books {
        title: vec![],
        author: vec![],
        accession_number: vec![],
        flag: 0,
    };
    let output = books.book_display();
    assert_eq!(output, Err("Not found".to_string()))
}
#[test]
fn total_book_success() {
    use crate::library_menu::Books;
    let books = Books {
        title: vec![
            "winner stand alone".to_string(),
            "The alchemist".to_string(),
        ],
        author: vec!["paulo".to_string(), "coelho".to_string()],
        accession_number: vec![1, 2],
        flag: 0,
    };
    let output = books.total_book();
    assert_eq!(output, Some(2))
}
#[test]
fn total_book_failure() {
    use crate::library_menu::Books;
    let books = Books {
        title: vec![],
        author: vec![],
        accession_number: vec![],
        flag: 0,
    };
    let output = books.total_book();
    assert_eq!(output, None)
}
#[test]
fn display_by_title_success(){
    use crate::library_menu::Books;
    let books = Books {
        title: vec![
            "winner stand alone".to_string(),
            "The alchemist".to_string(),
        ],
        author: vec!["paulo".to_string(), "coelho".to_string()],
        accession_number: vec![1, 2],
        flag: 0,
    };
    let output = books.display_by_title("winner stand alone".to_string());
    assert_eq!(output,Ok("The book is present withe given title".to_string())
    )

}
#[test]
fn display_by_title_failure(){
    use crate::library_menu::Books;
    let books = Books {
        title: vec![
            "winner stand alone".to_string(),
            "The alchemist".to_string(),
        ],
        author: vec!["paulo".to_string(), "coelho".to_string()],
        accession_number: vec![1, 2],
        flag: 0,
    };
    let output = books.display_by_title("winner stand".to_string());
    assert_eq!(output,Err("Hey book is not present".to_string()));

}
#[test]
fn display_by_author_success(){
    use crate::library_menu::Books;
    let books = Books {
        title: vec![
            "winner stand alone".to_string(),
            "The alchemist".to_string(),
        ],
        author: vec!["paulo".to_string(), "coelho".to_string()],
        accession_number: vec![1, 2],
        flag: 0,
    };
    let output = books.display_by_author("paulo".to_string());
    assert_eq!(output,Ok("The book is present with given author".to_string())
    )

}
#[test]
fn display_by_author_failure(){
    use crate::library_menu::Books;
    let books = Books {
        title: vec![
            "winner stand alone".to_string(),
            "The alchemist".to_string(),
        ],
        author: vec!["paulo".to_string(), "coelho".to_string()],
        accession_number: vec![1, 2],
        flag: 0,
    };
    let output = books.display_by_author("Dheeraj".to_string());
    assert_eq!(output,Err("Hey book is not present".to_string()));

}