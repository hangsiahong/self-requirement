use std::cmp;

pub fn insertion_sort<T>(arr: &mut [T])
where
    T: cmp::PartialOrd+ Copy,
{
    for i in 1..arr.len() {
        let cur = arr[i];
        let mut j = i - 1;
        
        while arr[j] > cur {
            arr[j + 1] = arr[j];
            if j == 0 {
                break;
            }
            j -= 1;
        
        }
        if j == 0 && arr[0] > cur {
            arr[0] = cur;
        } else {
            arr[j + 1] = cur;
        }
    }
}
    
