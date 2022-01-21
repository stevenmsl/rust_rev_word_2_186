use rust_rev_word_2_186::Solution;
fn main() {
    let result = Solution::reverse_words(&mut Solution::test_fixture_extra_spaces_1());
    println!("{:?}", result);
}
