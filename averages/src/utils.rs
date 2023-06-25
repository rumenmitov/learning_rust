pub fn merge_sort(mut array :Vec<i32>) -> Vec<i32> {
  if array.len() == 1 {
      array
  } else if array.len() == 2 {
      if array[0] > array[1] {
          (array[0], array[1]) = (array[1], array[0]);
      }
      array
  } else {
      let midpoint = &array.len() / 2;
      let left_arr :Vec<i32> = merge_sort(array[..midpoint].to_vec());
      let mut right_arr :Vec<i32> = merge_sort(array[midpoint..].to_vec());

      'outerloop: for num_left in &left_arr {
          for (index, num_right) in &mut right_arr.iter().enumerate() {
              if *num_left <= *num_right {
                  right_arr.insert(index, *num_left);
                  continue 'outerloop;
              }
          }
          right_arr.push(*num_left);
      }
      right_arr
  }
}

pub fn get_median(array :&Vec<i32>) -> f32 {
  let midpoint = array.len() / 2;
  
  if array.len() % 2 == 0 {
    (array[midpoint-1] as f32 + array[midpoint] as f32) / 2.0
  } else {
    array[midpoint] as f32
  }
}

pub fn get_mode(array :&Vec<i32>) -> Vec<i32> {
  let mut current_mode_count = (array[0], 0);
  let mut max_mode_count = (vec![array[0]], 0);

  for num in array {
    if *num != current_mode_count.0 {
      current_mode_count.0 = *num;
      current_mode_count.1 = 0;
    }

    current_mode_count.1 += 1;

    if current_mode_count.1 > max_mode_count.1 {
      max_mode_count = (vec![current_mode_count.0], current_mode_count.1);
    } else if current_mode_count.1 == max_mode_count.1 {
      max_mode_count.0.push(current_mode_count.0);
    }
  }

  max_mode_count.0
}

pub fn get_mean(array :&Vec<i32>) -> f32 {
  let mut sum :f32 = 0.0;

  for num in array {
    sum += (*num) as f32;
  }

  (sum / (array.len()) as f32) as f32
}