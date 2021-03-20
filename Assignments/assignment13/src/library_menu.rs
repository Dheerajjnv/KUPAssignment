/// Structure containing data about books.
///
/// #Varient
/// title - Title of the book stored in the type Vec<String>.
/// author - Author name of the book stored in the type Vec<String>.
/// accession_number - Accession number of the book stored in the type Vec<i32>.
/// flag - Flag value representing the issue status.
pub struct Books {
    pub title: Vec<String>,
    pub author: Vec<String>,
    pub accession_number: Vec<i32>,
    pub flag: i32,
}
/// Implementation of method on Books
impl Books {
    /// This function add books details in the existing data.
    ///
    /// #Argument
    /// title - Title of the book of type String.
    /// author - Author name of the type String.
    /// accession_number - Accession number of the book  type String.
    /// flag - Flag value representing the issue status of type i32
    ///
    /// #Return
    /// Panic if the book already exist else add the book
    pub fn add_book(&mut self, title: String, author: String, number: i32, flag: i32) {
        if self.accession_number.contains(&number) {
            panic!("Hey already exist")
        } else {
            self.author.push(author);
            self.title.push(title);
            self.accession_number.push(number);
            self.flag = flag
        }
    }
    /// This function display the data of the books are present.
    ///
    /// #Argument
    /// &mut self - Self type parameter
    ///
    /// #Return
    /// Result of Ok value if the data is present else error message.
    pub fn book_display(&mut self) -> Result<String, String> {
        if self.accession_number.is_empty() {
            return Err("Not found".to_string());
        }
        for index in 0..self.accession_number.len() {
            log::info!(
                "{},{},{}",
                self.accession_number[index],
                self.author[index],
                self.title[index]
            );
        }
        Ok("Data displayed".to_string())
    }
    /// This function gives the total no. of  books are present.
    ///
    /// #Argument
    /// &mut self - Self type parameter
    ///
    /// #Return
    /// Option  of Some value if the data is present else None.

    pub fn total_book(&self) -> Option<usize> {
        match !self.accession_number.is_empty() {
            true => Some(self.accession_number.len()),
            false => None,
        }
    }
    /// This function gives the information of book based on title.
    ///
    /// #Argument
    /// &self - A referenced Self type parameter
    /// reference - The title given of type String
    ///
    /// #Return
    /// The Ok type value if the book with title is present in data else error message.
    pub fn display_by_title(&self, reference: String) -> Result<String, String> {
        if !self.title.contains(&reference) {
            return Err("Hey book is not present".to_string());
        }
        for index in 0..self.title.len() {
            if reference == self.title[index] {
                log::info!(
                    "{},{},{},{}",
                    self.title[index],
                    self.flag,
                    self.accession_number[index],
                    self.author[index]
                );
            }
        }
        Ok("The book is present withe given title".to_string())
    }
    /// This function gives the information of book based on author.
    ///
    /// #Argument
    /// &self - A referenced Self type parameter
    /// reference - The author given of type String
    ///
    /// #Return
    /// The Ok type value if the book with author present in data else error message.
    pub fn display_by_author(&self, reference: String) -> Result<String, String> {
        if !self.author.contains(&reference) {
            return Err("Hey book is not present".to_string());
        }
        for index in 0..self.author.len() {
            if reference == self.author[index] {
                log::info!(
                    "{},{},{},{}",
                    self.title[index],
                    self.flag,
                    self.accession_number[index],
                    self.author[index]
                );
            }
        }
        Ok("The book is present with given author".to_string())
    }
    /// This function issue the book and update the data.
    ///
    /// #Argument
    /// &self - A referenced Self type parameter
    /// title - The title  given of type String
    ///
    /// #Return
    /// The Ok type value if the book with title issued in data else error message.
    pub fn issue_books(&mut self, title: String) -> Result<String, String> {
        if !self.title.contains(&title) {
            Err("Hey this is not present".to_string())
        } else {
            for i in 0..self.title.len() - 1 {
                if title == self.title[i] {
                    self.title.remove(i);
                    self.flag = 1;
                    self.author.remove(i);
                    self.accession_number.remove(i);
                }
            }
            Ok("Book issued".to_string())
        }
    }
}
