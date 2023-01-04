
// fn transpose<'a, 'b>(matrix: &[&[i32]]) -> &'a[&'b[i32]] {
//     let len = matrix.len();
//     let len2 = matrix[0].len();
//     let mut new_matrix: &[&[i32]] = matrix.clone();
//     for x in 0..len2 {
//         for y in 0..len {
//             new_matrix[x][y] = matrix[y][x];
//         }
//     }
//     return new_matrix;
// }

// fn pretty_print(matrix: &[&[i32]]) {
//     for row in matrix {
//         println!("{row:?}");
//     }
// }

// fn main() {
//     let matrix = [
//         [101, 102, 103], // <-- the comment makes rustfmt add a newline
//         [201, 202, 203],
//         [301, 302, 303],
//     ];

//     println!("matrix:");
//     pretty_print(&matrix);

//     let transposed = transpose(&matrix);
//     println!("transposed:");
//     pretty_print(&transposed);
// }

struct Library {
    books: Vec<Book>,
}

struct Book {
    title: String,
    year: u16,
}

impl Book {
    // This is a constructor, used below.
    fn new(title: &str, year: u16) -> Book {
        Book {
            title: String::from(title),
            year,
        }
    }
}

// This makes it possible to print Book values with {}.
impl std::fmt::Display for Book {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.title, self.year)
    }
}

impl Library {
    fn new() -> Library {
        Library {
            books: Vec::new()
        }
    }

    fn len(&self) -> usize {
        self.books.len()
    }

    fn is_empty(&self) -> bool {
       self.books.is_empty()
    }

    fn add_book(&mut self, book: Book) {
       self.books.push(book);
    }

    fn print_books(&self) {
        for book in &self.books {
            println!("{book}")
        }
    }

    fn oldest_book(&self) -> Option<&Book> {
        self.books.iter().min_by_key(|book| book.year)
    }
    //fn oldest_book(self) -> Option<&Book> {
    //    unimplemented!()
    //}
}

fn main() {
    // This shows the desired behavior. Uncomment the code below and
    // implement the missing methods. You will need to update the
    // method signatures, including the "self" parameter!
    let mut library = Library::new();

    println!("Our library is empty: {}", library.is_empty());

    library.add_book(Book::new("Lord of the Rings", 1954));
    library.add_book(Book::new("Alice's Adventures in Wonderland", 1865));
    
    library.print_books();
    
    match library.oldest_book() {
       Some(book) => println!("My oldest book is {book}"),
       None => println!("My library is empty!"),
    }
    
    println!("Our library has {} books", library.len());
}