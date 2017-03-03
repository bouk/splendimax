pub struct CopyingIter<I> where I: Iterator + Clone {
    iter: I,
}

impl<I> Iterator for CopyingIter<I> where I: Iterator + Clone {
    type Item = (I::Item, I);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|item| (item, self.iter.clone()))
    }
}

pub trait CopyingIterator: Iterator {
    fn copying(self) -> CopyingIter<Self> where Self: Clone {
        CopyingIter {
            iter: self,
        }
    }
}

impl<I> CopyingIterator for I where I: Iterator {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn copying_iter() {
        let mut total = 0;
        for (_, inner_iter) in (0..4).copying() {
            for _ in inner_iter {
                total += 1;
            }
        }
        assert_eq!(total, 6);
    }
}