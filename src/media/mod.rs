pub mod media_container {

    #[derive(Debug)]
    enum Media {
        Book { title: String, author: String },
        Movie { title: String, director: String },
        Audiobook { title: String, author: String },
        Podcast(u32),
        No_Media,
    }

    struct Catalog {
        media: Vec<Media>,
    }

    impl Catalog {
        fn new() -> Self {
            Self { media: Vec::new() }
        }

        fn add(&mut self, media: Media) {
            self.media.push(media);
        }
    }

    impl Media {
        pub fn new_book(title: String, author: String) -> Media {
            Media::Book { title, author }
        }

        pub fn new_movie(title: String, director: String) -> Media {
            Media::Movie { title, director }
        }

        pub fn new_audiobook(title: String, author: String) -> Media {
            Media::Audiobook { title, author }
        }

        pub fn description(&self) -> String {
            match self {
                Media::Book { title, author } => format!("{} by {}", title, author),
                Media::Movie { title, director } => format!("{} by {}", title, director),
                Media::Audiobook { title, author } => format!("{} by {}", title, author),
                Media::Podcast(episodes) => format!("{} episodes", episodes),
                Media::No_Media => "No_Media".to_string(),
            }
        }
    }

    pub fn add_media() {
        let book = Media::new_book("The Hobbit".to_string(), "J.R.R. Tolkien".to_string());
        let movie = Media::new_movie(
            "The Lord of the Rings".to_string(),
            "Peter Jackson".to_string(),
        );
        let audiobook =
            Media::new_audiobook("The Silmarillion".to_string(), "J.R.R. Tolkien".to_string());

        println!(
            "{}, {}, {}",
            book.description(),
            movie.description(),
            audiobook.description()
        );

        let mut media_catalog = Catalog::new();
        media_catalog.add(book);
        media_catalog.add(movie);
        media_catalog.add(audiobook);

        let ins = media_catalog.media.get(30).unwrap_or(&Media::No_Media);

        println!("{:#?}", ins);
    }
}
