use std::collections::HashMap;

#[derive(Debug)]
struct Person {
    name: String,
    age: u32
}

impl Person {
    pub fn new(name: String, age: u32) -> Person {
        Person {
            name,
            age
        }
    }
}

#[derive(Debug)]
#[derive(PartialEq)]
struct Book {
    title: String,
    author: String,
    is_available: bool
}

impl Book {
    pub fn new(title: String, author: String, is_available: bool) -> Book {
        Book {
            title,
            author,
            is_available
        }
    }

    pub fn clone(&self) -> Book {
        Book {
            title: self.title.to_string(),
            author: self.author.to_string(),
            is_available: self.is_available
        }
    }
}

#[derive(Debug)]
struct Library {
    books: Vec<Book>,
    book_borrowers: HashMap<String, String>
}

impl Library {
    pub fn new() -> Library {
        Library {
            books: Vec::new(),
            book_borrowers: HashMap::new()
        }
    }

    fn get_books(&self) -> &Vec<Book> {
        &self.books
    }

    fn add_book(& mut self, book: Book) {
        self.books.push(book)
    }

    fn checkout_book(&mut self, book_title: String, borrower_name: String) -> Book {
        let book = self.books.iter_mut().find(|mybook| mybook.title == book_title).expect("Book not in library, cannot checkout");
        if book.is_available == false {
            panic!("Book is already checked out");
        }
        book.is_available = false;
        self.book_borrowers.insert(book.title.to_string(), borrower_name);
        book.clone()
    }

    fn return_book(&mut self, book_title: String) {
        let my_book = self.books.iter_mut().find(|mybook| mybook.title == book_title).expect("Book not in library, cannot return");
        if my_book.is_available {
            panic!("Book is not checked out");
        }
        my_book.is_available = true;
        self.book_borrowers.remove(my_book.title.as_str());
    }

    fn get_available_books(&self) -> Vec<String> {
        let res: Vec<String> = self.books.iter()
            .filter(|book| book.is_available)
            .map(|book| book.title.clone())
            .collect();
        res
    }

    fn get_checkedout_books(&self) -> Vec<String> {
        let res: Vec<String> = self.books.iter()
            .filter(|book| book.is_available == false)
            .map(|book| {
                let mut st = book.title.clone();
                st.push_str(" : ");
                st.push_str(self.book_borrowers.get(book.title.as_str()).expect(""));
                st
            })
            .collect();
        res
    }
}

fn display_person(person: &Person) {
    println!("In display_person method: Person name is: {:?} and age is: {:?}", person.name, person.age);
}


fn main() {
    let person = Person::new("Mark".to_string(), 34);
    display_person(&person);
    // person is only borrowed in above, so can be used still
    println!("Person is: {:?}", person);


    let book = Book::new("Book1".to_string(), "Jules".to_string(), true);
    let book2 = Book::new("Book2".to_string(), "Ove".to_string(), true);
    
    let mut library = Library::new();  
    library.add_book(book);
    library.add_book(book2);

    println!("All books: {:?}", library.get_books());
    println!("Available books: {:?}", library.get_available_books());
    println!("Checked out books: {:?}", library.get_checkedout_books());

    println!("Borrowing book 1 *****");
    let borrowed_book1 = library.checkout_book("Book1".to_string(), "Person1".to_string());
    println!("Borrowed Book: {:?}", borrowed_book1);
    println!("Available books: {:?}", library.get_available_books());
    println!("Checked out books: {:?}", library.get_checkedout_books());

    println!("Borrowing book 2 *****");
    let b = library.checkout_book("Book2".to_string(), "Person1".to_string());
    println!("Borrowed Book: {:?}", b);
    println!("Available books: {:?}", library.get_available_books());
    println!("Checked out books: {:?}", library.get_checkedout_books());

    println!("Returning book 2 *****");
    library.return_book("Book2".to_string());
    println!("Available books: {:?}", library.get_available_books());
    println!("Checked out books: {:?}", library.get_checkedout_books());

    println!("Returning book 1 *****");
    library.return_book("Book1".to_string());
    println!("Available books: {:?}", library.get_available_books());
    println!("Checked out books: {:?}", library.get_checkedout_books());

    println!("Borrowed Book: {:?}", b);
}
