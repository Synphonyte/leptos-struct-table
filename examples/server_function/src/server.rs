use crate::app::*;
use leptos::*;
use leptos_struct_table::*;
use std::collections::VecDeque;

#[server(GetBooks, "/api")]
pub async fn get_books(
    sorting: VecDeque<(BookColumnName, ColumnSort)>,
) -> Result<Vec<Book>, ServerFnError> {
    log!("{sorting:?}");
    if sorting.is_empty() {
        return Ok(vec![]);
    }

    let mut books = get_table_data_json();

    for (name, sort_type) in sorting.iter().rev() {
        books.sort_by(|b1, b2| b1.get(*name).partial_cmp(&b2.get(*name)).unwrap());
        if *sort_type == ColumnSort::Descending {
            books.reverse();
        }
    }

    Ok(books)
}

pub fn get_table_data_json() -> Vec<Book> {
    serde_json::from_str("[
  {
    \"id\": 1,
    \"title\": \"To Kill a Mockingbird\",
    \"author\": \"Harper Lee\",
    \"publication_year\": 1960,
    \"publisher\": \"J. B. Lippincott & Co.\",
    \"genre\": \"Fiction\",
    \"description\": \"A classic novel depicting racial injustice in the Deep South.\",
    \"isbn\": \"9780061120084\"
  },
  {
    \"id\": 2,
    \"title\": \"1984\",
    \"author\": \"George Orwell\",
    \"publication_year\": 1949,
    \"publisher\": \"Secker & Warburg\",
    \"genre\": \"Dystopian\",
    \"description\": \"A dystopian novel set in a totalitarian society.\",
    \"isbn\": \"9780451524935\"
  },
  {
    \"id\": 3,
    \"title\": \"The Great Gatsby\",
    \"author\": \"F. Scott Fitzgerald\",
    \"publication_year\": 1925,
    \"publisher\": \"Charles Scribner\'s Sons\",
    \"genre\": \"Fiction\",
    \"description\": \"A story of wealth, love, and the American Dream in the 1920s.\",
    \"isbn\": \"9780743273565\"
  },
  {
    \"id\": 4,
    \"title\": \"Pride and Prejudice\",
    \"author\": \"Jane Austen\",
    \"publication_year\": 1813,
    \"publisher\": \"T. Egerton, Whitehall\",
    \"genre\": \"Romance\",
    \"description\": \"A classic romance novel exploring social class and marriage.\",
    \"isbn\": \"9780141439518\"
  },
  {
    \"id\": 5,
    \"title\": \"The Catcher in the Rye\",
    \"author\": \"J. D. Salinger\",
    \"publication_year\": 1951,
    \"publisher\": \"Little, Brown and Company\",
    \"genre\": \"Fiction\",
    \"description\": \"A coming-of-age story of a teenager in New York City.\",
    \"isbn\": \"9780316769488\"
  },
  {
    \"id\": 6,
    \"title\": \"To the Lighthouse\",
    \"author\": \"Virginia Woolf\",
    \"publication_year\": 1927,
    \"publisher\": \"Hogarth Press\",
    \"genre\": \"Modernist\",
    \"description\": \"An experimental novel exploring the complexities of human experience.\",
    \"isbn\": \"9780156907392\"
  },
  {
    \"id\": 7,
    \"title\": \"Moby-Dick\",
    \"author\": \"Herman Melville\",
    \"publication_year\": 1851,
    \"publisher\": \"Harper & Brothers\",
    \"genre\": \"Adventure\",
    \"description\": \"A tale of obsession and revenge set against the backdrop of whaling.\",
    \"isbn\": \"9780142437247\"
  },
  {
    \"id\": 8,
    \"title\": \"Jane Eyre\",
    \"author\": \"Charlotte Brontë\",
    \"publication_year\": 1847,
    \"publisher\": \"Smith, Elder & Co.\",
    \"genre\": \"Gothic\",
    \"description\": \"A bildungsroman following the life of an orphaned governess.\",
    \"isbn\": \"9780141441146\"
  },
  {
    \"id\": 9,
    \"title\": \"The Hobbit\",
    \"author\": \"J. R. R. Tolkien\",
    \"publication_year\": 1937,
    \"publisher\": \"George Allen & Unwin\",
    \"genre\": \"Fantasy\",
    \"description\": \"A fantasy adventure novel set in Middle-earth.\",
    \"isbn\": \"9780345339683\"
  },
  {
    \"id\": 10,
    \"title\": \"The Lord of the Rings\",
    \"author\": \"J. R. R. Tolkien\",
    \"publication_year\": 1954,
    \"publisher\": \"George Allen & Unwin\",
    \"genre\": \"Fantasy\",
    \"description\": \"An epic fantasy trilogy set in the world of Middle-earth.\",
    \"isbn\": \"9780618640157\"
  },
  {
    \"id\": 11,
    \"title\": \"Brave New World\",
    \"author\": \"Aldous Huxley\",
    \"publication_year\": 1932,
    \"publisher\": \"Chatto & Windus\",
    \"genre\": \"Dystopian\",
    \"description\": \"A dystopian novel exploring a futuristic societ\'s control over its citizens.\",
    \"isbn\": \"9780060850524\"
  },
  {
    \"id\": 12,
    \"title\": \"The Chronicles of Narnia\",
    \"author\": \"C. S. Lewis\",
    \"publication_year\": 1950,
    \"publisher\": \"Geoffrey Bles\",
    \"genre\": \"Fantasy\",
    \"description\": \"A series of fantasy novels set in the magical world of Narnia.\",
    \"isbn\": \"9780066238500\"
  },
  {
    \"id\": 13,
    \"title\": \"The Book Thief\",
    \"author\": \"Markus Zusak\",
    \"publication_year\": 2005,
    \"publisher\": \"Picador\",
    \"genre\": \"Historical\",
    \"description\": \"A story of a young girl\'s love for books during World War II.\",
    \"isbn\": \"9780375842207\"
  },
  {
    \"id\": 14,
    \"title\": \"Gone with the Wind\",
    \"author\": \"Margaret Mitchell\",
    \"publication_year\": 1936,
    \"publisher\": \"Macmillan Publishers\",
    \"genre\": \"Historical\",
    \"description\": \"An epic novel set in the American South during the Civil War.\",
    \"isbn\": \"9781416548942\"
  },
  {
    \"id\": 15,
    \"title\": \"The Odyssey\",
    \"author\": \"Homer\",
    \"publication_year\": -800,
    \"publisher\": \"Various\",
    \"genre\": \"Epic\",
    \"description\": \"An ancient Greek epic poem depicting Odysseus\' journey back home.\",
    \"isbn\": \"9780140268867\"
  },
  {
    \"id\": 16,
    \"title\": \"Alice\'s Adventures in Wonderland\",
    \"author\": \"Lewis Carroll\",
    \"publication_year\": 1865,
    \"publisher\": \"Macmillan\",
    \"genre\": \"Fantasy\",
    \"description\": \"A whimsical tale following Alice\'s adventures in a fantastical world.\",
    \"isbn\": \"9780141439761\"
  },
  {
    \"id\": 17,
    \"title\": \"Wuthering Heights\",
    \"author\": \"Emily Brontë\",
    \"publication_year\": 1847,
    \"publisher\": \"Thomas Cautley Newby\",
    \"genre\": \"Gothic\",
    \"description\": \"A dark and passionate tale of love and revenge on the Yorkshire moors.\",
    \"isbn\": \"9781853260018\"
  },
  {
    \"id\": 18,
    \"title\": \"The Little Prince\",
    \"author\": \"Antoine de Saint-Exupéry\",
    \"publication_year\": 1943,
    \"publisher\": \"Reynal & Hitchcock\",
    \"genre\": \"Children\'s\",
    \"description\": \"A philosophical children\'s novella about a young prince.\",
    \"isbn\": \"9780156012195\"
  },
  {
    \"id\": 19,
    \"title\": \"Anna Karenina\",
    \"author\": \"Leo Tolstoy\",
    \"publication_year\": 1877,
    \"publisher\": \"The Russian Messenger\",
    \"genre\": \"Realistic\",
    \"description\": \"A Russian novel exploring themes of love, infidelity, and society.\",
    \"isbn\": \"9780143035008\"
  },
  {
    \"id\": 20,
    \"title\": \"The Adventures of Sherlock Holmes\",
    \"author\": \"Arthur Conan Doyle\",
    \"publication_year\": 1892,
    \"publisher\": \"George Newnes Ltd\",
    \"genre\": \"Mystery\",
    \"description\": \"A collection of detective stories featuring the famous detective Sherlock Holmes.\",
    \"isbn\": \"9781503275938\"
  },
  {
    \"id\": 21,
    \"title\": \"Don Quixote\",
    \"author\": \"Miguel de Cervantes\",
    \"publication_year\": 1605,
    \"publisher\": \"Francisco de Robles\",
    \"genre\": \"Adventure\",
    \"description\": \"A Spanish novel following the adventures of a nobleman who imagines himself to be a knight.\",
    \"isbn\": \"9780060934347\"
  },
  {
    \"id\": 22,
    \"title\": \"The Grapes of Wrath\",
    \"author\": \"John Steinbeck\",
    \"publication_year\": 1939,
    \"publisher\": \"The Viking Press\",
    \"genre\": \"Fiction\",
    \"description\": \"A novel depicting the struggles of a family during the Great Depression.\",
    \"isbn\": \"9780143039433\"
  },
  {
    \"id\": 23,
    \"title\": \"Lord of the Flies\",
    \"author\": \"William Golding\",
    \"publication_year\": 1954,
    \"publisher\": \"Faber and Faber\",
    \"genre\": \"Adventure\",
    \"description\": \"A story of a group of boys stranded on an uninhabited island, exploring the dark side of human nature.\",
    \"isbn\": \"9780140283334\"
  },
  {
    \"id\": 24,
    \"title\": \"The Old Man and the Sea\",
    \"author\": \"Ernest Hemingway\",
    \"publication_year\": 1952,
    \"publisher\": \"Charles Scribner\'s Sons\",
    \"genre\": \"Adventure\",
    \"description\": \"A novella depicting an aging fisherman\'s struggle with a giant marlin.\",
    \"isbn\": \"9780684801223\"
  },
  {
    \"id\": 25,
    \"title\": \"The Stranger\",
    \"author\": \"Albert Camus\",
    \"publication_year\": 1942,
    \"publisher\": \"Gallimard\",
    \"genre\": \"Absurdist\",
    \"description\": \"A philosophical novel exploring existentialism and the meaning of life.\",
    \"isbn\": \"9780679720201\"
  },
  {
    \"id\": 26,
    \"title\": \"Great Expectations\",
    \"author\": \"Charles Dickens\",
    \"publication_year\": 1861,
    \"publisher\": \"Chapman & Hall\",
    \"genre\": \"Coming-of-age\",
    \"description\": \"A bildungsroman following the life of an orphan named Pip.\",
    \"isbn\": \"9780141439563\"
  },
  {
    \"id\": 27,
    \"title\": \"The Kite Runner\",
    \"author\": \"Khaled Hosseini\",
    \"publication_year\": 2003,
    \"publisher\": \"Riverhead Books\",
    \"genre\": \"Historical\",
    \"description\": \"A story of friendship, betrayal, and redemption set in Afghanistan.\",
    \"isbn\": \"9781594631931\"
  },
  {
    \"id\": 28,
    \"title\": \"The Alchemist\",
    \"author\": \"Paulo Coelho\",
    \"publication_year\": 1988,
    \"publisher\": \"HarperOne\",
    \"genre\": \"Fantasy\",
    \"description\": \"A philosophical novel about a young shepherd\'s journey to find his personal legend.\",
    \"isbn\": \"9780062315007\"
  },
  {
    \"id\": 29,
    \"title\": \"One Hundred Years of Solitude\",
    \"author\": \"Gabriel García Márquez\",
    \"publication_year\": 1967,
    \"publisher\": \"Harper & Row\",
    \"genre\": \"Magic Realism\",
    \"description\": \"A multigenerational family saga blending reality and fantasy.\",
    \"isbn\": \"9780060883287\"
  },
  {
    \"id\": 30,
    \"title\": \"Crime and Punishment\",
    \"author\": \"Fyodor Dostoevsky\",
    \"publication_year\": 1866,
    \"publisher\": \"The Russian Messenger\",
    \"genre\": \"Psychological\",
    \"description\": \"A psychological novel exploring the moral dilemmas of a young man.\",
    \"isbn\": \"9780140449136\"
  },
  {
    \"id\": 31,
    \"title\": \"The Picture of Dorian Gray\",
    \"author\": \"Oscar Wilde\",
    \"publication_year\": 1890,
    \"publisher\": \"Ward, Lock and Company\",
    \"genre\": \"Gothic\",
    \"description\": \"A novel about a man whose portrait ages while he remains youthful.\",
    \"isbn\": \"9780141439570\"
  },
  {
    \"id\": 32,
    \"title\": \"The Scarlet Letter\",
    \"author\": \"Nathaniel Hawthorne\",
    \"publication_year\": 1850,
    \"publisher\": \"Ticknor, Reed & Fields\",
    \"genre\": \"Romance\",
    \"description\": \"A novel set in 17th-century Puritan Boston, exploring sin and redemption.\",
    \"isbn\": \"9780142437261\"
  },
  {
    \"id\": 33,
    \"title\": \"Fahrenheit 451\",
    \"author\": \"Ray Bradbury\",
    \"publication_year\": 1953,
    \"publisher\": \"Ballantine Books\",
    \"genre\": \"Dystopian\",
    \"description\": \"A dystopian novel depicting a future society where books are banned.\",
    \"isbn\": \"9781451673319\"
  },
  {
    \"id\": 34,
    \"title\": \"Frankenstein\",
    \"author\": \"Mary Shelley\",
    \"publication_year\": 1818,
    \"publisher\": \"Lackington, Hughes, Harding, Mavor & Jones\",
    \"genre\": \"Gothic\",
    \"description\": \"A science fiction novel exploring the consequences of creating life.\",
    \"isbn\": \"9780141439471\"
  },
  {
    \"id\": 35,
    \"title\": \"The Count of Monte Cristo\",
    \"author\": \"Alexandre Dumas\",
    \"publication_year\": 1844,
    \"publisher\": \"Pétion\",
    \"genre\": \"Adventure\",
    \"description\": \"An adventure novel of revenge and redemption in 19th-century France.\",
    \"isbn\": \"9780140449266\"
  },
  {
    \"id\": 36,
    \"title\": \"The Divine Comedy\",
    \"author\": \"Dante Alighieri\",
    \"publication_year\": 1320,
    \"publisher\": \"Various\",
    \"genre\": \"Epic Poetry\",
    \"description\": \"An epic poem depicting the journey through Hell, Purgatory, and Heaven.\",
    \"isbn\": \"9780141195152\"
  },
  {
    \"id\": 37,
    \"title\": \"The Secret Garden\",
    \"author\": \"Frances Hodgson Burnett\",
    \"publication_year\": 1911,
    \"publisher\": \"Frederick A. Stokes\",
    \"genre\": \"Children\'s\",
    \"description\": \"A children\'s novel about a girl who discovers a hidden garden.\",
    \"isbn\": \"9780141336534\"
  },
  {
    \"id\": 38,
    \"title\": \"A Tale of Two Cities\",
    \"author\": \"Charles Dickens\",
    \"publication_year\": 1859,
    \"publisher\": \"Chapman & Hall\",
    \"genre\": \"Historical\",
    \"description\": \"A historical novel set in London and Paris during the French Revolution.\",
    \"isbn\": \"9780141439600\"
  },
  {
    \"id\": 39,
    \"title\": \"Les Misérables\",
    \"author\": \"Victor Hugo\",
    \"publication_year\": 1862,
    \"publisher\": \"A. Lacroix, Verboeckhoven & Cie\",
    \"genre\": \"Historical\",
    \"description\": \"A historical novel following the lives of several characters in 19th-century France.\",
    \"isbn\": \"9780451525260\"
  },
  {
    \"id\": 40,
    \"title\": \"The Adventures of Huckleberry Finn\",
    \"author\": \"Mark Twain\",
    \"publication_year\": 1884,
    \"publisher\": \"Chatto & Windus\",
    \"genre\": \"Adventure\",
    \"description\": \"A coming-of-age adventure story set along the Mississippi River.\",
    \"isbn\": \"9780486280615\"
  },
  {
    \"id\": 41,
    \"title\": \"The Canterbury Tales\",
    \"author\": \"Geoffrey Chaucer\",
    \"publication_year\": 1475,
    \"publisher\": \"Various\",
    \"genre\": \"Poetry\",
    \"description\": \"A collection of stories told by pilgrims traveling to Canterbury.\",
    \"isbn\": \"9780140424386\"
  },
  {
    \"id\": 42,
    \"title\": \"Dracula\",
    \"author\": \"Bram Stoker\",
    \"publication_year\": 1897,
    \"publisher\": \"Archibald Constable and Company\",
    \"genre\": \"Gothic\",
    \"description\": \"A Gothic horror novel featuring Count Dracula and vampire mythology.\",
    \"isbn\": \"9780141439846\"
  },
  {
    \"id\": 43,
    \"title\": \"The Iliad\",
    \"author\": \"Homer\",
    \"publication_year\": -800,
    \"publisher\": \"Various\",
    \"genre\": \"Epic Poetry\",
    \"description\": \"An ancient Greek epic poem recounting the Trojan War.\",
    \"isbn\": \"9780140275364\"
  },
  {
    \"id\": 44,
    \"title\": \"The Jungle Book\",
    \"author\": \"Rudyard Kipling\",
    \"publication_year\": 1894,
    \"publisher\": \"Macmillan Publishers\",
    \"genre\": \"Children\'s\",
    \"description\": \"A collection of stories set in the Indian jungle featuring Mowgli and animal characters.\",
    \"isbn\": \"9780141325293\"
  },
  {
    \"id\": 45,
    \"title\": \"Moby-Dick\",
    \"author\": \"Herman Melville\",
    \"publication_year\": 1851,
    \"publisher\": \"Harper & Brothers\",
    \"genre\": \"Adventure\",
    \"description\": \"A novel about Captain Ahab\'s obsessive pursuit of the white whale.\",
    \"isbn\": \"9780142437247\"
  },
  {
    \"id\": 46,
    \"title\": \"To Kill a Mockingbird\",
    \"author\": \"Harper Lee\",
    \"publication_year\": 1960,
    \"publisher\": \"J. B. Lippincott & Co.\",
    \"genre\": \"Fiction\",
    \"description\": \"A coming-of-age story set in the racially charged atmosphere of the 1930s Deep South.\",
    \"isbn\": \"9780060935467\"
  },
  {
    \"id\": 47,
    \"title\": \"The Catcher in the Rye\",
    \"author\": \"J. D. Salinger\",
    \"publication_year\": 1951,
    \"publisher\": \"Little, Brown and Company\",
    \"genre\": \"Fiction\",
    \"description\": \"A novel narrated by Holden Caulfield, a disenchanted teenager.\",
    \"isbn\": \"9780316769488\"
  },
  {
    \"id\": 48,
    \"title\": \"The Great Gatsby\",
    \"author\": \"F. Scott Fitzgerald\",
    \"publication_year\": 1925,
    \"publisher\": \"Charles Scribner\'s Sons\",
    \"genre\": \"Fiction\",
    \"description\": \"A novel depicting the Jazz Age and the elusive American Dream.\",
    \"isbn\": \"9780141182633\"
  },
  {
    \"id\": 49,
    \"title\": \"Pride and Prejudice\",
    \"author\": \"Jane Austen\",
    \"publication_year\": 1813,
    \"publisher\": \"T. Egerton, Whitehall\",
    \"genre\": \"Romance\",
    \"description\": \"A classic romance novel exploring themes of love, social class, and prejudice.\",
    \"isbn\": \"9780141439518\"
  },
  {
    \"id\": 50,
    \"title\": \"The Hobbit\",
    \"author\": \"J. R. R. Tolkien\",
    \"publication_year\": 1937,
    \"publisher\": \"George Allen & Unwin\",
    \"genre\": \"Fantasy\",
    \"description\": \"A fantasy adventure novel set in Middle-earth.\",
    \"isbn\": \"9780345339683\"
  },
  {
    \"id\": 51,
    \"title\": \"The Lord of the Rings\",
    \"author\": \"J. R. R. Tolkien\",
    \"publication_year\": 1954,
    \"publisher\": \"George Allen & Unwin\",
    \"genre\": \"Fantasy\",
    \"description\": \"An epic fantasy trilogy set in the world of Middle-earth.\",
    \"isbn\": \"9780618640157\"
  },
  {
    \"id\": 52,
    \"title\": \"Brave New World\",
    \"author\": \"Aldous Huxley\",
    \"publication_year\": 1932,
    \"publisher\": \"Chatto & Windus\",
    \"genre\": \"Dystopian\",
    \"description\": \"A dystopian novel exploring a futuristic society\'s control over its citizens.\",
    \"isbn\": \"9780060850524\"
  },
  {
    \"id\": 53,
    \"title\": \"Jane Eyre\",
    \"author\": \"Charlotte Brontë\",
    \"publication_year\": 1847,
    \"publisher\": \"Smith, Elder & Co.\",
    \"genre\": \"Gothic\",
    \"description\": \"A Gothic romance novel following the life of Jane Eyre.\",
    \"isbn\": \"9780141441146\"
  },
  {
    \"id\": 54,
    \"title\": \"1984\",
    \"author\": \"George Orwell\",
    \"publication_year\": 1949,
    \"publisher\": \"Secker & Warburg\",
    \"genre\": \"Dystopian\",
    \"description\": \"A dystopian novel depicting a totalitarian society ruled by Big Brother.\",
    \"isbn\": \"9780451524935\"
  },
  {
    \"id\": 55,
    \"title\": \"The Odyssey\",
    \"author\": \"Homer\",
    \"publication_year\": -725,
    \"publisher\": \"Various\",
    \"genre\": \"Epic Poetry\",
    \"description\": \"An ancient Greek epic poem recounting Odysseus\' journey back from the Trojan War.\",
    \"isbn\": \"9780143039952\"
  }
]").expect("Serializing books should not fail.")
}
