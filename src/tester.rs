use std::result::Result;

struct MyStruct {
    values: Result<Values, ()>,
}

struct Values {
    value1: String,
    value2: u32,
}

impl MyStruct {
    pub fn new() -> MyStruct {
        // In practice, this can fail, and the result gets stored.
        MyStruct {
            values: Ok(Values {
                value1: "TESTER".to_string(),
                value2: 56,
            }),
        }
    }

    fn with_values<'a, T, F>(&'a self, f: F) -> Result<T, ()>
    where
        F: FnOnce(&'a Values) -> T,
    {
        if let Ok(values) = self.values.as_ref() {
            Ok(f(values))
        } else {
            Err(())
        }
    }

    pub fn value1(&self) -> Result<&str, ()> {
        self.with_values(|v| v.value1.as_str())
    }

    pub fn value2(&self) -> Result<u32, ()> {
        self.with_values(|v| v.value2)
    }
}
