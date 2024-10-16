pub fn print_elements<T: std::fmt::Debug>(lst: &[T]) {
    lst.iter().for_each(|x| println!("{:?}", x));
}

pub fn shorten_string(lst: &mut [String], max_length: usize) {
    lst.iter_mut().for_each(|line| line.truncate(max_length));
}

pub fn uppercase(lst: &[String]) -> Vec<String> {
    lst.iter().map(|line| line.to_uppercase()).collect()
}

pub fn move_elements<T>(list: Vec<T>, destination: &mut Vec<T>) {
    list.into_iter()
        .for_each(|element| destination.push(element));
}

pub fn explode(list: &[String]) -> Vec<Vec<String>> {
    list.iter()
        .map(|elem| elem.chars().map(|e| e.to_string()).collect())
        .collect()
}

pub fn find_element_or(list: &[String], pattern: &str, fallback: String) -> String {
    list.iter()
        .find(|elem| elem.contains(pattern))
        .map_or(fallback, |elem| elem.to_string())
}
