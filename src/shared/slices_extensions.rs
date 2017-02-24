trait Rotatable {
    fn rotate_forward(&mut self, steps : usize);
    fn rotate_backward(&mut self, steps : usize);
}

impl<T: Clone> Rotatable for [T] {
    fn rotate_forward(&mut self, steps : usize){
        let clone = self.to_vec();
        let mid : usize = steps % self.len();
        let antimid : usize = self.len() - mid;
        let (left, right) = clone.split_at(antimid);
        self[..mid].clone_from_slice(right);
        self[mid..].clone_from_slice(left);
    }
    fn rotate_backward(&mut self, steps : usize){
        let clone = self.to_vec();
        let antimid : usize = steps % self.len();
        let mid : usize = self.len() - antimid;
        let (left, right) = clone.split_at(antimid);
        self[..mid].clone_from_slice(right);
        self[mid..].clone_from_slice(left);
    }
}

fn bisect_right<T>(x: &[T], value: T) -> usize
    where T: PartialOrd {
    if x.len() == 0 || value < x[0] {
        return 0;
    }
    let (mut l, mut r) = (0, x.len());
    while r - l > 1 {
        let middle = (r + l) / 2;
        if value < x[middle] {
            r = middle;
        }
        else {
            l = middle;
        }
    }
    r
}
fn bisect_left<T>(x: &[T], value: T) -> usize
    where T: PartialOrd {
    if x.len() == 0 || !(x[0] < value) {
        return 0;
    }
    let (mut l, mut r) = (0, x.len());
    while r - l > 1 {
        let middle = (r + l) / 2;
        if x[middle] < value {
            l = middle;
        }
        else {
            r = middle;
        }
    }
    r
}