pub enum Letters<'a> {
  Vowel,
  Consonants(char),
  Err(&'a str)
}

impl Letters<'_> {
  pub fn new(letter :Option<char>) -> Letters<'static> {
    match letter {
      Some(first_char) => {
        if first_char == 'a'
        || first_char == 'e'
        || first_char == 'o'
        || first_char == 'u'
        || first_char == 'i' {
          Letters::Vowel
      } else {
        Letters::Consonants(first_char)
      }
      },
      None => Letters::Err("Error! No letter recieved!"),
    }
    

  }
}