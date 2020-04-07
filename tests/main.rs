use magicsquare::peano::magic_print;

#[test]
fn find_one() {
    magic_print(4);
    // The sum of the rank-4 square is 34.
    // 1 15 14  4
    // 12  6  7  9
    // 8 10 11  5
    // 13  3  2 16
    magic_print(5);
    // The sum of the rank-5 square is 65.
    // 17 23  4 10 11
    // 24  5  6 12 18
    // 1  7 13 19 25
    // 8 14 20 21  2
    // 15 16 22  3  9
    magic_print(6);
    // The sum of the rank-6 square is 111.
    // 35  3  4 26 21 22
    // 1 32  9 19 23 27
    // 33  7  2 24 25 20
    // 8 30 31 17 12 13
    // 28  5 36 10 14 18
    // 6 34 29 15 16 11
}
