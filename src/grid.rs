#[derive(Clone)]
pub struct FixedGrid<T> {
    data: Vec<T>,
    width: usize,
    height: usize,
}

impl<T> FixedGrid<T> where T: Clone + Copy {
    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn get(&self, x: usize, y: usize) -> T {
        *self.data.get(y * self.width + x).unwrap()
    }

    pub fn set(&mut self, x: usize, y: usize, v: T) {
        self.data[y * self.width + x] = v;
    }

    pub fn new(width: usize, height: usize, def: T) ->FixedGrid<T> {
        FixedGrid{
            data: vec![def; width * height],
            width, height,
        }
    }

    pub fn from(width: usize, height: usize, data: Vec<T>) ->FixedGrid<T> {
        assert_eq!(data.len(), (width * height));

        FixedGrid{
            data, width, height,
        }
    }
}