pub fn container_with_most_water_fn(x: &Vec<usize>) -> usize{
    let mut l = 0;
    let mut r = x.len() - 1;

    let mut max_area = 0;

    while l < r {
        let area = (std::cmp::min(x[l], x[r])) * (r-1);
        max_area = std::cmp::max(max_area, area);

        if x[l] < x[r] {
            l += 1
        }
        else {
            r -= 1
        }
    }

    max_area
}