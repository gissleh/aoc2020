#[derive(Clone)]
pub struct FixedGrid<T> {
    data: Vec<T>,
    width: usize,
    height: usize,
}

impl<T> FixedGrid<T>
where
    T: Clone + Copy,
{
    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn get(&self, x: usize, y: usize) -> T {
        *self.data.get(y * self.width + x).unwrap()
    }

    pub fn get_safe(&self, x: usize, y: usize) -> Option<T> {
        if x > self.width {
            None
        } else if y > self.height {
            None
        } else {
            self.data.get(y * self.width + x).map(|v| *v)
        }
    }

    pub fn data(&self) -> &[T] {
        return &self.data;
    }

    pub fn set(&mut self, x: usize, y: usize, v: T) {
        self.data[y * self.width + x] = v;
    }

    pub fn set_slice(&mut self, x: usize, y: usize, src: &[T]) {
        let index = y * self.width + x;
        self.data[index..index + src.len()].copy_from_slice(src);
    }

    pub fn new(width: usize, height: usize, def: T) -> FixedGrid<T> {
        FixedGrid {
            data: vec![def; width * height],
            width,
            height,
        }
    }

    pub fn from(width: usize, height: usize, data: Vec<T>) -> FixedGrid<T> {
        assert_eq!(data.len(), (width * height));

        FixedGrid {
            data,
            width,
            height,
        }
    }
}

impl<T> FixedGrid<T>
where
    T: Eq,
{
    pub fn count(&self, v: T) -> usize {
        let mut count = 0;
        for v2 in self.data.iter() {
            if v == *v2 {
                count += 1;
            }
        }

        count
    }
}
