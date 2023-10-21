

/*
 * @brief: Helper function to get the minimum value and associated index of a 
 *         Array1<u32> 
 */
fn find_minimum_value_and_index(arr: &Array1<u32>) -> (u32, usize) {
  let (index, min_value) = arr
      .iter()
      .enumerate()
      .min_by(|(_, x), (_, y)| x.partial_cmp(y).unwrap())
      .unwrap();
  (*min_value, index)
}

/*
 * @brief: Helper function to get the maximum value and associated index of a 
 *         Array1<u32> 
 */
fn find_maximum_value_and_index(arr: &Array1<u32>) -> (u32, usize) {
  let (index, max_value) = arr
      .iter()
      .enumerate()
      .max_by(|(_, x), (_, y)| x.partial_cmp(y).unwrap())
      .unwrap();
  (*max_value, index)
}