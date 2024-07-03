pub trait TokenStream: Iterator<Item = char> {
    fn take_next_word(&mut self) -> Option<String>;
}
