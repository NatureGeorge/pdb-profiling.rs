use rle_vec::RleVec;
use std::ops::{Add, Sub, Mul};


#[allow(dead_code)]
pub struct RleNumSeq<T> {
    rle: RleVec<T>,
    denominator: T,
}


#[allow(dead_code)]
impl<T> RleNumSeq<T>
    where T: Add<Output=T> + Sub<Output=T> + Mul<Output=T> + Eq + Copy + Clone
{
    fn new(v: &Vec<T>, d: T) -> RleNumSeq<T> {
        let rle: RleVec<T> = v.iter().cloned().collect();
        RleNumSeq{
            rle: rle,
            denominator: d,
        }
    }

}


pub struct DeltaRleNumSeq<T> {
    root: T,
    rle: RleVec<T>,
}


#[allow(dead_code)]
impl<T> DeltaRleNumSeq<T>
    where T: Add<Output=T> + Sub<Output=T> + Mul<Output=T> + Eq + Copy + Clone
{
    fn new(v: &Vec<T>) -> DeltaRleNumSeq<T>
    {
        let root = v[0];
        let rle: RleVec<T> = v.iter().zip(v.iter().skip(1)).map(|(&x,&y)| y - x).collect();
        DeltaRleNumSeq{
            root: root,
            rle: rle,
        }
    }

    fn decode(&self) -> Vec<T> {
        let mut raw = vec![self.root];
        raw.extend(self.rle.iter().cloned().scan(self.root, |acc, x| {
            *acc = *acc + x;
            Some(*acc)
        }));
        raw
    }
}



#[cfg(test)]
mod tests {
    #[test]
    fn test_seq_id_2_rle_1() {
        let v: Vec<i16> = vec![1,2,3,4,5,6];
        //                       1 1 1 1 1
        //                       {1, 5}
        let trans = super::DeltaRleNumSeq::new(&v);
        assert_eq!(trans.rle.runs_len(), 1);
        assert_eq!(trans.rle.len(), 5);
        assert_eq!(trans.rle.iter().nth(3), Some(&1));
        let v_delta = trans.rle.iter().cloned().collect::<Vec<_>>();
        assert_eq!(v_delta, vec![1,1,1,1,1]);
        assert_eq!(trans.decode(), v);
    }
    #[test]
    fn test_seq_id_2_rle_2() {
        let v: Vec<i16> = vec![1,2,3,4,5,6,9,10,11];
        //                       1 1 1 1 1 3 1  1
        //                       {1, 5} {3, 1} {1, 2}
        let trans = super::DeltaRleNumSeq::new(&v);
        assert_eq!(trans.rle.runs_len(), 3);
        assert_eq!(trans.rle.len(), 8);
        assert_eq!(trans.rle.iter().nth(5), Some(&3));
        let v_delta = trans.rle.iter().cloned().collect::<Vec<_>>();
        assert_eq!(v_delta, vec![1,1,1,1,1,3,1,1]);
        assert_eq!(trans.decode(), v);
    }
    #[test]
    fn test_seq_id_2_rle_3() {
        let v: Vec<i16> = vec![1000,1000,0,0,0,881,882,1000];
        let trans = super::RleNumSeq::new(&v, 1000);
        assert_eq!(trans.rle.runs_len(), 5);
        assert_eq!(trans.rle.len(), 8);
        assert_eq!(trans.rle.iter().nth(5), Some(&881));
        let v_decode = trans.rle.iter().cloned().map(|x| x as f32 / trans.denominator as f32).collect::<Vec<_>>();
        assert_eq!(v_decode.len(), 8);
        println!("{:?}", v_decode);
    }
}