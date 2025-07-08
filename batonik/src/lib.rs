use std::future::Future;
use std::pin::Pin;

pub struct App {
    modules: Vec<Pin<Box<dyn Future<Output = String>>>>,
}


impl std::fmt::Debug for Module {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ptr = &*self.0 as *const _;
        f.debug_tuple("Module").field(&format_args!("{:p}", ptr)).finish()
    }
}

pub struct Module(Pin<Box<dyn Future<Output = String>>>);

pub trait IntoModule {
    fn into_module(self) -> Module;
}

impl IntoModule for String {
    fn into_module(self) -> Module {
        let f = async {
            return self;
        };

        Module(Box::pin(f))
    }
}

impl<F, Fut> IntoModule for F
where
    F: FnOnce() -> Fut,
    Fut: Future<Output = String> + Send + 'static,
{
    fn into_module(self) -> Module {
        let fut = self();
        Module(Box::pin(fut))
    }
}

impl<Fut> IntoModule for Fut
where
    Fut: Future<Output = String> + Send + 'static
{
    fn into_module(self) -> Module {
        todo!()
    }
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
