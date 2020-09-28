mod knp;

#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    if _first_list.len() == 0 && _second_list.len() == 0 {
        return Comparison::Equal;
    } else if _first_list.len() == 0 {
        return Comparison::Sublist;
    } else if _second_list.len() == 0 {
        return Comparison::Superlist;
    } else {
        let long;
        let short;
        if _first_list.len() > _second_list.len() {
            long = _first_list;
            short = _second_list;
            let k = knp::KNP::new(short);
            return match k.matches(long) {
                true => Comparison::Superlist,
                false => Comparison::Unequal,
            };
        } else {
            long = _second_list;
            short = _first_list;
            let k = knp::KNP::new(short);
            return match k.matches(long) {
                true => {
                    if short.len() == long.len() {
                        Comparison::Equal
                    } else {
                        Comparison::Sublist
                    }
                }
                false => Comparison::Unequal,
            };
        }
    }
}
