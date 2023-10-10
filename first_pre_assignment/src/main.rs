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
}

#[derive(Debug)]
struct Library {
    books: Vec<Book>
}

impl Library {
    pub fn new() -> Library {
        Library {
            books: Vec::new()
        }
    }

    pub fn get_books(&self) -> &Vec<Book> {
        &self.books
    }

    pub fn add_book(&mut self, book: Book) {
        self.books.push(book)
    }
}

fn display_person(person: &Person) {
    println!("In display_person method: Person name is: {:?} and age is: {:?}", person.name, person.age);
}


fn main() {
    let person = Person::new("Mark".to_string(), 34);
    println!("Person is: {:?}", person);
    println!("Person name is: {:?}", person.name);
    println!("Person age is: {:?}", person.age);

    let book = Book::new("Around the world".to_string(), "Mark".to_string(), true);
    println!("Book is: {:?}", book);
    println!("Book title is: {:?}", book.title);
    println!("Book author is: {:?}", book.author);
    println!("Book is_available is: {:?}", book.is_available);

    let mut library = Library::new();
    println!("Library is: {:?}", library);
    println!("Library books are: {:?}", library.get_books());
    
    library.add_book(book);
    println!("Library is: {:?}", library);
    println!("Library books are: {:?}", library.get_books());

    display_person(&person);

    println!("Person is: {:?}", person);
}
