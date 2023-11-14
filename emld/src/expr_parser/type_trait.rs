pub struct CharType;

pub trait TypeTrait {
    type Item;
}

impl TypeTrait for CharType {
    type Item = char;
}
