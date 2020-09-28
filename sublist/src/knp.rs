pub struct KNP<'a, T: PartialEq> {
    pattern: &'a [T],
    prefixes: Vec<i32>,
}

impl<T: PartialEq> KNP<'_, T> {
    pub fn new(pattern: &[T]) -> KNP<T> {
        let mut prefixes = vec![0; pattern.len() + 1];

        let mut i = 0; // Current position
        let mut j = -1; // Length of already found prefix
        
        prefixes[i] = j;
        while i < pattern.len() {
            while j >= 0 && pattern[j as usize] != pattern[i] {
                j = prefixes[j as usize];
            }

            i = i + 1;
            j = j + 1;
            prefixes[i] = j;
        }

        let k = KNP {
            pattern: pattern,
            prefixes: prefixes,
        };

        k
    }

    pub fn matches(&self, search_list: &[T]) -> bool {
        let mut i = 0;
        let mut j: i32 = 0;

        while i < search_list.len() {
            while j >= 0 && search_list[i] != self.pattern[j as usize] {
                j = self.prefixes[j as usize];
            }

            i = i + 1;
            j = j + 1;
            if j == self.pattern.len() as i32 {
                return true;
            }
        }
        false
    }
}
