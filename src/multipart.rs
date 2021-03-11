use std::io::Write;

pub struct MultiPart<'a> {
    boundary: u32,
    values: Vec<(&'a str, &'a [u8])>,
}

impl<'a> MultiPart<'a> {
    pub fn new() -> Self {
        Self {
            boundary: rand::random(),
            values: Vec::new(),
        }
    }

    pub fn content_type(&self) -> String {
        format!("multipart/form-data; boundary={}", self.boundary,)
    }

    pub fn add<T>(&mut self, name: &'a str, content: &'a T)
    where
        T: AsRef<[u8]> + ?Sized,
    {
        self.values.push((name, content.as_ref()));
    }

    pub fn build(&self) -> Vec<u8> {
        let mut value = Vec::new();
        for (name, content) in &self.values {
            write!(
                &mut value,
                "--{}\r\nContent-Disposition: form-data; name=\"{}\"\r\n\r\n",
                self.boundary, name,
            )
            .unwrap();
            value.extend(content.iter());
            value.extend(b"\r\n");
        }

        write!(&mut value, "--{}--\r\n", self.boundary).unwrap();

        value
    }
}
