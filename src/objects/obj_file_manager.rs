use crate::objects::polygon::Polygon;
use crate::objects::vertex::Vertex;
use num::Float;
use std::str::FromStr;
use std::{fmt, fs};

pub struct ObjFileManager<T> {
    pub objects: Vec<(T, T)>,
    pub polygons: Vec<Vec<usize>>,
}

impl<T> ObjFileManager<T>
where
    T: Float + Copy + FromStr<Err: fmt::Debug> + fmt::Debug,
{
    fn read_file(path: &str) -> (Vec<(T, T)>, Vec<Vec<usize>>) {
        let file_content = fs::read_to_string(path).expect("File did not exists, or other error");
        let lines = file_content.split("\n").collect::<Vec<&str>>();

        let mut vertices = vec![];
        let mut polygons = vec![];

        for line in lines {
            let splitted = line.split_whitespace().collect::<Vec<&str>>();

            if splitted.is_empty() || splitted[0] == "#" {
                continue;
            }

            if splitted[0] == "v" {
                if splitted.len() > 3 {
                    panic!("ObjFileManager does not support more than 2 coordinates at the moment");
                }
                let tuple = (
                    splitted[1].parse::<T>().unwrap(),
                    splitted[2].parse::<T>().unwrap(),
                );
                vertices.push(tuple);
            }

            if splitted[0] == "f" {
                let mut points = vec![];
                for p in splitted[1..].iter() {
                    points.push(p.parse::<usize>().unwrap());
                }

                polygons.push(points);
            }
        }

        (vertices, polygons)
    }

    pub fn new(path: &str) -> ObjFileManager<T> {
        let (objects, mut polygons) = Self::read_file(path);
        if polygons.is_empty() {
            polygons.push((1..=objects.len()).collect::<Vec<usize>>())
        }
        Self { objects, polygons }
    }

    pub fn len_polygons(&self) -> usize {
        self.polygons.len()
    }

    pub fn get_polygon(&self, idx: usize) -> Polygon<T> {
        assert!(idx < self.len_polygons());

        let mut points = vec![];

        for p in &self.polygons[idx] {
            let tuple = &self.objects[*p - 1];
            points.push(Vertex {
                position: [tuple.0, tuple.1, T::zero()],
                color: [1.0, 0.0, 0.0],
            });
        }

        Polygon::new(points)
    }

    pub fn get_custom_polygon<I: AsRef<[usize]>>(&self, indizes: I) -> Polygon<T> {
        let mut points = vec![];

        for p in indizes.as_ref() {
            let tuple = &self.objects[*p - 1];
            points.push(Vertex {
                position: [tuple.0, tuple.1, T::zero()],
                color: [1.0, 0.0, 0.0],
            });
        }

        Polygon::new(points)
    }
}
