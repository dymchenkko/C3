use std::borrow::Cow;

//As there wasn't a particular condition when it should process data and in what way, let's add '.' to the end of string it there is no one.
pub fn process_data(sentence: &mut Cow<str>){
    if !sentence.ends_with(".") {
        sentence.to_mut().push('.');
    }
}
