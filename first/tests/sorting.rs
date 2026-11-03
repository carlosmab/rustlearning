use first::sorting::sort;

#[test]
fn empty_list_stays_empty() {
    assert_eq!(sort(vec![]), vec![]);
}

#[test]
fn unique_elem_list_stays_the_same() {
    assert_eq!(sort(vec![1]), [1]);
}

#[test]
fn elements_are_sorted() {
    let unsorted_array = vec![200, -3, 1210, 100];
    let sorted_array = vec![-3, 100, 200, 1210];
    assert_eq!(sort(unsorted_array), sorted_array);
}