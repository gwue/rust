pub fn build_proverb(list: &[&str]) -> String {
    let mut proverb = String::new();
    if !list.is_empty() {
        for item in 0..list.len() - 1 {
            proverb += &(format!(
                "For want of a {} the {} was lost.\n",
                list[item],
                list[item + 1]
            ));
        }
        proverb += &(format!("And all for the want of a {}.", list[0]));
    }
    proverb
}
