/*
	sort
	This problem requires you to implement a sorting algorithm
	you can use bubble sorting, insertion sorting, heap sorting, etc.
*/
// I AM DONE

fn sort<T: std::cmp::Ord>(array: &mut [T]){
	//TODO
    if array.len() < 2{
        return ();
    }

    if array.len() == 2{
        if array[0] > array[1] {
            array.swap(0, 1);
        }
        return ;
    }

    let mut l = 1;
    let mut r = array.len() - 1;
    
loop {
        while l <= r && array[l] <= array[0] {
            l += 1;
        }
        while l <= r && array[r] > array[0] {
            r -= 1;
        }
        if l > r {
            break;
        }
        array.swap(l, r);
        l += 1;
        r -= 1;
    }

    // 8. 循环结束后，r 是分界点。交换 pivot 和 array[r]
    //    此时 r 指向了左侧分区的最后一个元素
    array.swap(0, r);
    
    if r > 0 {
        sort(&mut array[0..r]);
    }
    if r + 1 < array.len() {
        sort(&mut array[r+1..]);
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_1() {
        let mut vec = vec![37, 73, 57, 75, 91, 19, 46, 64];
        sort(&mut vec);
        assert_eq!(vec, vec![19, 37, 46, 57, 64, 73, 75, 91]);
    }
	#[test]
    fn test_sort_2() {
        let mut vec = vec![1];
        sort(&mut vec);
        assert_eq!(vec, vec![1]);
    }
	#[test]
    fn test_sort_3() {
        let mut vec = vec![99, 88, 77, 66, 55, 44, 33, 22, 11];
        sort(&mut vec);
        assert_eq!(vec, vec![11, 22, 33, 44, 55, 66, 77, 88, 99]);
    }
}