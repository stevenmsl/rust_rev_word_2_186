#[derive(Debug, PartialEq)]
pub struct Solution {}

/* key takeaways
   - remove additional spaces
     - leading spaces, trailing spaces, or more than
       one spaces in-between words
     - this is actually harder to get it right than
       you might think
   - reverse the entire string
   - reverse each words
     - as they are all reversed in the previous step
*/

impl Solution {
  pub fn reverse_words(s: &mut Vec<char>) -> String {
    Self::remove_extra_spaces(s);
    println!("remove extra spaces: {:?}", s);

    /* reverse the entire string */
    Self::reverse(s, 0, s.len() - 1);
    println!("reverse the string: {:?}", s);

    let total_length = s.len();
    let mut fast: usize = 0;
    let mut slow: usize = 0;

    while fast < total_length {
      /*
        - find the end char of a word
        - "fast" will stop at a space or
          out-of-bound
      */
      while fast < total_length && s[fast] != ' ' {
        fast += 1;
      }

      /*
        - note that fast will be pointing at
          a space or out-of-bound
        - so the last char of the word you
          are reversing is actually at
          fast - 1
      */
      Self::reverse(s, slow, fast - 1);

      /*
        on to next word
      */
      fast += 1;
      slow = fast;
    }

    println!("reverse each word: {:?}", s);

    s.iter().collect()
  }

  pub fn reverse(s: &mut Vec<char>, start: usize, end: usize) {
    let mut start = start;
    let mut end = end;
    while start < end {
      let temp = s[start];
      s[start] = s[end];
      s[end] = temp;
      start += 1;
      end -= 1;
    }
  }

  /* takeaways
    - don't be deceived by the task
      thinking it's easy
    - you have to deal with additional
      spaces in-between words as well
      as leading and trailing spaces
  */
  pub fn remove_extra_spaces(s: &mut Vec<char>) {
    let len = s.len();
    let mut slow: usize = 0;
    let mut fast: usize = 0;

    while slow < len && fast < len {
      /*
        - "fast" has to move first
        - this is to deal with leading
          spaces (spaces before the first word)
        - stops when you find a word
      */
      while fast < len && s[fast] == ' ' {
        fast += 1;
      }

      /*
        - copying word the "fast" is pointing at
          to the location where "slow" is
          pointing at
      */
      while fast < len && s[fast] != ' ' {
        s[slow] = s[fast];
        slow += 1;
        fast += 1;
      }

      /*
        - why we are doing this again after copying
          a word? It seems redundant.
        - the reason for this is to deal with trailing
          spaces. After you copied the last word
          you are relying on "fast" < len so you
          won't be adding an additional space
          after the last word
      */
      while fast < len && s[fast] == ' ' {
        fast += 1;
      }

      /*
        - add additional space after a word is copied but the last one
      */
      if fast < len {
        s[slow] = ' ';
        slow += 1;
      }
    }
    s.truncate(slow);
  }

  pub fn to_chars(s: &String) -> Vec<char> {
    s.to_ascii_lowercase().chars().collect()
  }

  pub fn test_fixture_extra_spaces_1() -> Vec<char> {
    let s = String::from("  the sky  is  blue");
    Self::to_chars(&s)
  }
  pub fn test_fixture_extra_spaces_2() -> Vec<char> {
    let s = String::from("the  sky is blue ");
    Self::to_chars(&s)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_1() {
    let result = Solution::reverse_words(&mut Solution::test_fixture_extra_spaces_1());
    assert_eq!(result, String::from("blue is sky the"));
  }

  #[test]
  fn test_remove_extra_space_1() {
    let mut chars = Solution::test_fixture_extra_spaces_1();
    Solution::remove_extra_spaces(&mut chars);

    assert_eq!(chars, Solution::to_chars(&String::from("the sky is blue")));
  }

  #[test]
  fn test_remove_extra_space_2() {
    let mut chars = Solution::test_fixture_extra_spaces_2();
    Solution::remove_extra_spaces(&mut chars);

    assert_eq!(chars, Solution::to_chars(&String::from("the sky is blue")));
  }
}
