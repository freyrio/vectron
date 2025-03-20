use super::GpuError;

pub struct ErrorContext {
    pub function: &'static str,
    pub file: &'static str,
    pub line: u32,
}

impl ErrorContext {
    pub fn current() -> Self {
        Self {
            function: "",
            file: file!(),
            line: line!(),
        }
    }
    
    pub fn log(&self, error: &GpuError) {
        log::error!("{} at {}:{}: {}", 
                  self.function, self.file, self.line, error);
    }
}

#[macro_export]
macro_rules! gpu_try {
    ($expr:expr) => {
        match $expr {
            Ok(val) => val,
            Err(e) => {
                let ctx = $crate::common::ErrorContext::current();
                ctx.log(&e);
                return Err(e);
            }
        }
    };
    ($expr:expr, $context:expr) => {
        match $expr {
            Ok(val) => val,
            Err(e) => {
                log::error!("{} at {}:{}: {}", 
                          $context, file!(), line!(), e);
                return Err(e);
            }
        }
    };
}