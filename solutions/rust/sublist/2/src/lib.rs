#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist(first_list: &[i32], second_list: &[i32]) -> Comparison {
    if first_list == second_list {
        return Comparison::Equal;
    }

    if first_list.len() < second_list.len() {
        let pot_sublist = first_list;
        let superlist = second_list;

        if (0..=(superlist.len() - pot_sublist.len()))
            .map(|i| &superlist[i..i + pot_sublist.len()])
            .any(|slice| slice == pot_sublist)
        {
            return Comparison::Sublist;
        }
    } else {
        let pot_sublist = second_list;
        let superlist = first_list;

        if (0..=(superlist.len() - pot_sublist.len()))
            .map(|i| &superlist[i..i + pot_sublist.len()])
            .any(|slice| slice == pot_sublist)
        {
            return Comparison::Superlist;
        }
    }

    Comparison::Unequal
}
