pub mod parsing {
    use crate::metadata;

    pub fn parse(meta: &metadata::Metainfo) {
        for tokens in meta {
            match tokens.get_tokens() {
                Some(tks) => {
                    let mut it = metadata::Tokenable::new(tks.iter());
                    println!("next: {:?} -> {:?}", it.next(), it.look());
                },
                None => continue,
            }
        }
    }
}
