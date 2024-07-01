pub struct TokenStream<I> {
    row: usize,
    column: usize,
    owned_data: I,
}

impl<I> TokenStream<I> {}

impl From<String> for TokenStream<String> {
    fn from(value: String) -> Self {
        Self {
            row: 0,
            column: 0,
            owned_data: value,
        }
    }
}

// TODO: make TokenStream a trait rather than a struct
// Then we can impl TokenStream on Chars and whatever the file one is
