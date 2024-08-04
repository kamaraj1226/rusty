mod sort;

#[cfg(test)]
mod tests {
    use crate::sort::merge_sort::merge_sort;
    #[test]
    fn check_merge_sort() {
        let result = merge_sort(vec![2, 1, 3]);
        assert_eq!(result, vec![1, 2, 3]);

        let result = merge_sort(vec![0, 0, 0]);
        assert_eq!(result, vec![0, 0, 0]);

        let result = merge_sort(vec![100, 0, -1]);
        assert_eq!(result, vec![-1, 0, 100]);

        let result = merge_sort(vec![200]);
        assert_eq!(result, vec![200]);
    }
}
