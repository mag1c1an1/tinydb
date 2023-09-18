use bitvec::prelude::BitVec;
pub trait Array: Send + Sync + Sized + 'static {
    type OwnedItem: Scalar<ArrayType = Self>;
    type Builder: ArrayBuilder<Array=Self>;
    type RefItem<'a>: Clone + std::fmt::Debug;
    fn get(&self, idx: usize) -> Option<Self::RefItem<'_>>;
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
    fn iter(&self) -> ArrayIterator<Self>;
}

pub trait ArrayBuilder {
    type Array: Array<Builder=Self>;
    fn with_capacity(capacity: usize) -> Self;
    fn push(&mut self, value: Option<<Self::Array as Array>::RefItem<'_>>);
    /// Finish build and return a new array
    fn finish(self) -> Self::Array;
}

trait PrimitiveType {}


pub struct PrimitiveArray<T: PrimitiveType> {
    data: Vec<T>,
    bitmap: BitVec,
}

impl<T: PrimitiveType + Copy> PrimitiveArray<T> {
    fn get(&self, idx: usize) -> Option<T> {
        if self.bitmap[idx] {
            Some(self.data[idx])
        } else {
            None
        }
    }

    fn len(&self) -> usize {
        self.data.len()
    }
}


pub struct StringArray {
    data: Vec<u8>,
    offsets: Vec<usize>,
    bitmap: BitVec,
}

pub struct StringArrayBuilder;

impl ArrayBuilder for StringArrayBuilder {
    type Array = StringArray;

    fn with_capacity(capacity: usize) -> Self {
        todo!()
    }

    fn push(&mut self, value: Option<<Self::Array as Array>::RefItem<'_>>) {
        todo!()
    }

    fn finish(self) -> Self::Array {
        todo!()
    }
}


impl Array for StringArray {
    type Builder = StringArrayBuilder;
    type RefItem<'a> = &'a str;

    fn get(&self, idx: usize) -> Option<&str> {
        if self.bitmap[idx] {
            let range = self.offsets[idx]..self.offsets[idx + 1];
            Some(unsafe {
                std::str::from_utf8_unchecked(&self.data[range])
            })
        } else {
            None
        }
    }

    fn len(&self) -> usize {
        todo!()
    }

    fn iter(&self) -> ArrayIterator<Self> {
        ArrayIterator::new(self)
    }
}

pub struct ArrayIterator<'a, A: Array> {
    array: &'a A,
    pos: usize,
}

impl<'a, A: Array> Iterator for ArrayIterator<'a, A> {
    type Item = Option<A::RefItem<'a>>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.pos >= self.array.len() { None } else {
            let item = self.array.get(self.pos);
            self.pos += 1;
            Some(item)
        }
    }
}

impl<'a, A: Array> ArrayIterator<'a, A> {
    pub fn new(array: &'a A) -> Self {
        Self {
            array,
            pos: 0,
        }
    }
}
}


fn eval_binary<I:Array,O:Array>(i1:I,i2:O) -> O {
  assert_eq!(i1.len(),i2.len(),"size mismatch");
    let mut builder = O::Builder::with_capacity(i1.len());
    for(i1,i2) in i1.iter().zip(i2.iter()) {

    }
    builder.finish()
}

fn main() {
    println!("Hello, world!");
}
