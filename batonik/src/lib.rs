use std::pin::Pin;

pub struct App {
    modules: Vec<Pin<Box<dyn Future<Output = String>>>>,
}

impl App {
    pub fn new() -> Self {
        return Self { modules: vec![] };
    }

    pub fn add<F>(mut self, future: F) -> Self
    where
        F: Future<Output = String> + Send + 'static,
    {
        let p = Box::pin(future);
        self.modules.push(p);
        // self.modules.push(Box::pin(x));
        return self;
    }

    pub fn run(self) {
        let rt = tokio::runtime::Runtime::new().unwrap();

        rt.block_on(async {

            for m in self.modules {
                let res = m.await;
                println!("{res}");
            }

        })
    }
}
