use std::io;
use std::path::Path;
use std::{fs::File, io::Write};

use anyhow::{Ok, Result};

use crate::vec3::Color;

pub struct Image {
    file: File,
    width: usize,
    height: usize,
}

impl Image {
    pub fn open<P: AsRef<Path>>(path: P, width: usize, height: usize) -> Result<Self> {
        let mut file = File::options().create(true).write(true).open(path)?;
        file.write_fmt(format_args!("P3\n{} {}\n255\n", width, height))?;

        Ok(Image {
            file,
            width,
            height,
        })
    }

    pub fn write_color(&mut self, c: Color, sample: usize) -> Result<()> {
        let scale = 1.0 / (sample as f64);
        let r = (c.x() * scale).sqrt().max(0.0).min(0.999) * 256.0;
        let g = (c.y() * scale).sqrt().max(0.0).min(0.999) * 256.0;
        let b = (c.z() * scale).sqrt().max(0.0).min(0.999) * 256.0;

        self.file
            .write_fmt(format_args!("{} {} {}\n", r as u8, g as u8, b as u8))
            .map_err(|e| e.into())
    }

    pub fn write_color_with<F>(&mut self, sample: usize, mut f: F) -> Result<()>
    where
        F: FnMut(usize, usize) -> Color,
    {
        let mut stdout = io::stdout().lock();
        for j in (0..self.height).rev() {
            stdout.write_fmt(format_args!("\rwritting line {}/{} ...", j, self.height))?;
            stdout.flush()?;
            for i in 0..self.width {
                self.write_color(f(i, j), sample)?;
            }
        }

        stdout.write_fmt(format_args!("\rdone!\n"))?;
        stdout.flush()?;

        self.file.flush().map_err(|e| e.into())
    }
}
