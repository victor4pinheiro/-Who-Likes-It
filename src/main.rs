fn likes(names: &[&str]) -> String {
    let len_names: usize = names.len();

    if len_names == 0 {
        return "no one likes this".to_string();
    }
    if len_names == 1 {
        return format!("{} likes this", names[len_names - 1]);
    } else if len_names >= 4 {
        return format!(
            "{}, {} and {} others like this",
            names[0], names[1], len_names - 2
        );
    } else {
        let mut status_like = String::new();

        for (index, name) in names.into_iter().enumerate() {
            if index == len_names - 1 {
                status_like = format!("{} and {}", status_like.clone(), name);
            } else if index as i8 - 1 != -1 && index != len_names - 1 {
                status_like = format!("{}, {}", status_like.clone(), name);
            } else {
                status_like = format!("{}{}", status_like, name);
            }
        }

        return format!("{} like this", status_like);
    }
}

fn main() {
    assert_eq!(likes(&[]), "no one likes this");
    assert_eq!(likes(&["Peter"]), "Peter likes this");
    assert_eq!(likes(&["Jacob", "Alex"]), "Jacob and Alex like this");
    assert_eq!(
        likes(&["Max", "John", "Mark"]),
        "Max, John and Mark like this"
    );
    assert_eq!(
        likes(&["Alex", "Jacob", "Mark", "Max"]),
        "Alex, Jacob and 2 others like this"
    );
}
