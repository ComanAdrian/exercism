use saddle_points;

use saddle_points::find_saddle_points;
// Get index of maximum value from vector
//vec
//.into_iter()
//.enumerate()
//.max_by(|(_, a), (_, b)| a.cmp(b))
//.map(|(index, _)| index)
//.unwrap()

// We don't care about order
fn find_sorted_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let mut result = saddle_points::find_saddle_points(input);
    result.sort();
    result
}

#[test]
fn identify_single_saddle_point() {
    let input = vec![vec![9, 8, 7], vec![5, 3, 2], vec![6, 6, 7]];
    assert_eq!(vec![(1, 0)], find_saddle_points(&input));
}

#[test]
#[ignore]
fn identify_empty_matrix() {
    let input = vec![vec![], vec![], vec![]];
    let expected: Vec<(usize, usize)> = Vec::new();
    assert_eq!(expected, find_saddle_points(&input));
}

#[test]
#[ignore]
fn identify_lack_of_saddle_point() {
    let input = vec![vec![1, 2, 3], vec![3, 1, 2], vec![2, 3, 1]];
    let expected: Vec<(usize, usize)> = Vec::new();
    assert_eq!(expected, find_saddle_points(&input));
}

#[test]
#[ignore]
fn multiple_saddle_points_in_col() {
    let input = vec![vec![4, 5, 4], vec![3, 5, 5], vec![1, 5, 4]];
    assert_eq!(
        vec![(0, 1), (1, 1), (2, 1)],
        find_sorted_saddle_points(&input)
    );
}

#[test]
#[ignore]
fn multiple_saddle_points_in_row() {
    let input = vec![vec![6, 7, 8], vec![5, 5, 5], vec![7, 5, 6]];
    assert_eq!(
        vec![(1, 0), (1, 1), (1, 2)],
        find_sorted_saddle_points(&input)
    );
}

//Why do I deserve the science
//To feel better about you?
//At a loss I lost my cool
//I denied that I found you

//I tried to be a basket case
//I did not surprise you
//I'm trying to find a signal fire
//Let me know when I should move

//But you, amplified in the silence
//Justified in the way you make me bruise
//Magnified in the science
//Anatomically proved that you don't need me

//Why do I desire the space?
//I was mourning after you
//I was lost and lost my shape
//There was nothing I could do

//I don't want to waste away
//It was all I gave to you
//Take me back and take my place
//I will rise right up for you

//But you, amplified in the silence
//Justified in the way you make me bruise
//Magnified in the science
//Anatomically proved that you don't need me

//All the while you waste away, you're asking
//"Did I really need another one to take me down?"
//Everybody knows it's something that you had to live with darling
//Nobody's gonna tear you down now
//There is nothing you keep, there is only your reflection

//There was nothing but quiet retractions
//And families pleading, "Don't look in that cabinet
//There's far more bad than there's good, I don't know how it got there"
//That was something your father had burned in me
//Twenty hours out of Homestake eternity
//You can go anywhere but you are where you came from

//Little girl you are cursed by my ancestry
//There is nothing but darkness and agony
//I can not only see, but you stopped me from blinking
//Let me watch you as close as a memory
//Let me hold you above all the misery
//Let me open my eyes and be glad that I got here

#[test]
#[ignore]
fn identify_bottom_right_saddle_point() {
    let input = vec![vec![8, 7, 9], vec![6, 7, 6], vec![3, 2, 5]];
    assert_eq!(vec![(2, 2)], find_saddle_points(&input));
}

// track specific as of v1.3
#[test]
#[ignore]
fn non_square_matrix_high() {
    let input = vec![vec![1, 5], vec![3, 6], vec![2, 7], vec![3, 8]];
    assert_eq!(vec![(0, 1)], find_saddle_points(&input));
}

#[test]
#[ignore]
fn non_square_matrix_wide() {
    let input = vec![vec![3, 1, 3], vec![3, 2, 4]];
    assert_eq!(vec![(0, 0), (0, 2)], find_sorted_saddle_points(&input));
}

#[test]
#[ignore]
fn single_column_matrix() {
    let input = vec![vec![2], vec![1], vec![4], vec![1]];
    assert_eq!(vec![(1, 0), (3, 0)], find_sorted_saddle_points(&input));
}

#[test]
#[ignore]
fn single_row_matrix() {
    let input = vec![vec![2, 5, 3, 5]];
    assert_eq!(vec![(0, 1), (0, 3)], find_sorted_saddle_points(&input));
}

#[test]
#[ignore]
fn identify_all_saddle_points() {
    let input = vec![vec![5, 5, 5], vec![5, 5, 5], vec![5, 5, 5]];
    assert_eq!(
        vec![
            (0, 0),
            (0, 1),
            (0, 2),
            (1, 0),
            (1, 1),
            (1, 2),
            (2, 0),
            (2, 1),
            (2, 2)
        ],
        find_sorted_saddle_points(&input)
    );
}
