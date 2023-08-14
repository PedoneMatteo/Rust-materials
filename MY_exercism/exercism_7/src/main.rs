use std::fmt::Display;

//Given two lists determine if the first list is contained within the second list, if the second list is contained within the first list, if both lists are contained within each other or if none of these are true.
// Specifically, a list A is a sublist of list B if by dropping 0 or more elements from the front of B and 0 or more elements from the back of B you get a list that's completely equal to A.
// Examples:
//     A = [1, 2, 3], B = [1, 2, 3, 4, 5], A is a sublist of B
//     A = [3, 4, 5], B = [1, 2, 3, 4, 5], A is a sublist of B
//     A = [3, 4], B = [1, 2, 3, 4, 5], A is a sublist of B
//     A = [1, 2, 3], B = [1, 2, 3], A is equal to B
//     A = [1, 2, 3, 4, 5], B = [2, 3, 4], A is a superlist of B
//     A = [1, 2, 4], B = [1, 2, 3, 4, 5], A is not a superlist of, sublist of or equal to B
#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    let superlist = second_list.is_empty()
        || first_list
        .windows(second_list.len())
        .any(|x| x == second_list);
    let sublist = first_list.is_empty()
        || second_list
        .windows(first_list.len())
        .any(|x| x == first_list);
    match (superlist, sublist) {
        (true, true) => Comparison::Equal,
        (true, false) => Comparison::Superlist,
        (false, true) => Comparison::Sublist,
        (false, false) => Comparison::Unequal,
    }
}

fn main() {
    let A=[1,2,3];
    let B=[1,2,3,4,5];
    let result = sublist(&A,&B);
    match(result){
        Comparison::Equal => println!("Equal!"),
        Comparison::Sublist => println!("Sublist!"),
        Comparison::Superlist => println!("Superlist!"),
        Comparison::Unequal => println!("Unequal!"),
    }

}
