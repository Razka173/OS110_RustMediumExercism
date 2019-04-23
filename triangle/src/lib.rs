pub struct Triangle {
    sisi_a: u64, 
    sisi_b: u64,
    sisi_c: u64, 
}

impl Triangle {
    pub fn build(sides: [u64; 3]) -> Option<Triangle> {
        //unimplemented!("Construct new Triangle from following sides: {:?}. Return None if the sides are invalid.", sides);
        if sides[0] <= 0 || sides[1] <= 0 || sides[2] <= 0 {
            return None
        } else if (sides[0] + sides[1] <= sides[2]) || (sides[1] + sides[2] <= sides[0]) || (sides[0] + sides[2] <= sides[1]) {
            return None
        } else {
            let triangle: Triangle = Triangle { sisi_a: sides[0], sisi_b: sides[1], sisi_c: sides[2] };
            Some(triangle)
        }
    }

    pub fn is_equilateral(&self) -> bool {
        //unimplemented!("Determine if the Triangle is equilateral.");
        if self.sisi_a == self.sisi_b && self.sisi_a == self.sisi_c {
            true
        } else {
            false
        }
    }

    pub fn is_scalene(&self) -> bool {
        //unimplemented!("Determine if the Triangle is scalene.");
        if self.sisi_a != self.sisi_b && self.sisi_a != self.sisi_c && self.sisi_b != self.sisi_c {
            true
        } else {
            false
        }
    }

    pub fn is_isosceles(&self) -> bool {
        //unimplemented!("Determine if the Triangle is isosceles.");
        if (self.sisi_a == self.sisi_b && self.sisi_a != self.sisi_c) || (self.sisi_a != self.sisi_b && self.sisi_b == self.sisi_c) || (self.sisi_a == self.sisi_c && self.sisi_a != self.sisi_b) {
            true
        } else {
            false
        }
    }
}
