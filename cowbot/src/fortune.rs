type DB = [u8];
type Fortunes = Vec<String>;

macro_rules! fortune {
    ($file:expr) => {
        include_bytes!(concat!("../assets/fortune/datfiles/", $file))
    };
}

const DATABASES: [&DB; 43] = [
    fortune!("art"),
    fortune!("ascii-art"),
    fortune!("computers"),
    fortune!("cookie"),
    fortune!("definitions"),
    fortune!("debian"),
    fortune!("disclaimer"),
    fortune!("drugs"),
    fortune!("education"),
    fortune!("ethnic"),
    fortune!("humorists"),
    fortune!("food"),
    fortune!("fortunes"),
    fortune!("goedel"),
    fortune!("kids"),
    fortune!("knghtbrd"),
    fortune!("law"),
    fortune!("literature"),
    fortune!("linux"),
    fortune!("love"),
    fortune!("magic"),
    fortune!("medicine"),
    fortune!("men-women"),
    fortune!("miscellaneous"),
    fortune!("news"),
    fortune!("paradoxum"),
    fortune!("people"),
    fortune!("perl"),
    fortune!("pets"),
    fortune!("platitudes"),
    fortune!("politics"),
    fortune!("pratchett"),
    fortune!("riddles"),
    fortune!("science"),
    fortune!("shlomif-fav"),
    fortune!("songs-poems"),
    fortune!("sports"),
    fortune!("startrek"),
    fortune!("tao"),
    fortune!("translate-me"),
    fortune!("wisdom"),
    fortune!("work"),
    fortune!("zippy"),
];

// ...fortune text...\n%
// \n% = end of fortune
fn parse_db(db: &DB) -> Fortunes {
    let mut fortunes = Vec::new();
    let mut fortune = String::new();

    for line in db.split(|&x| x == b'\n') {
        if line == b"%" {
            fortunes.push(fortune.clone());
            fortune.clear();
        } else {
            fortune.push_str(&String::from_utf8_lossy(line));
            fortune.push('\n');
        }
    }

    fortunes
}

fn get_db() -> Fortunes {
    let random_int = rand::random::<usize>() % DATABASES.len();
    return parse_db(DATABASES[random_int]);
}

pub fn get_fortune() -> String {
    let db = get_db();
    let random_int = rand::random::<usize>() % db.len();
    db.get(random_int).unwrap().clone()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_db() {
        let db = parse_db(fortune!("art"));
        assert!(db.len() > 0);
    }

    #[test]
    fn test_get_db() {
        let db = get_db();
        assert!(db.len() > 0);
    }

    #[test]
    fn test_get_fortune() {
        let fortune = get_fortune();
        assert!(fortune.len() > 0);
    }
}
