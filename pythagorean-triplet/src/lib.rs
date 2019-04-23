use std::collections::HashSet;

pub fn find(sum: u32) -> HashSet<[u32; 3]> {
    //unimplemented!("Given the sum {}, return all possible Pythagorean triplets, which produce the said sum, or an empty HashSet if there are no such triplets. Note that you are expected to return triplets in [a, b, c] order, where a < b < c", sum);
    let mut hashset_result: HashSet<[u32; 3]> = HashSet::with_capacity(3);
    
    for x in 1..(sum/3) {

        for y in 1..(sum/2) {
            
            let z = sum - x - y;
            let x_cube = x*x;
            let y_cube = y*y;
            let z_cube = z*z;
            if x_cube + y_cube == z_cube {
                if x > y {
                    hashset_result.insert([y, x, z]);
                } else {
                    hashset_result.insert([x, y, z]);
                }
            }
        }
    }

    return hashset_result;

}
