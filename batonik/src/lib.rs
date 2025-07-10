use std::future::Future;
use std::pin::Pin;

use tokio::task::JoinSet;

type Module = Pin<Box<dyn Future<Output = String> + Send + 'static>>;

pub struct Batonik {
    modules: Vec<Module>,
}

type ModuleAdder<Mod> = fn(&mut Batonik, Mod) -> &mut Batonik;

pub trait FutureModule {
    type This;
    fn get_add(&self) -> ModuleAdder<Self::This>;
}

pub trait StringModule {
    type This;
    fn get_add(&self) -> ModuleAdder<Self::This>;
}

impl Batonik {
    pub fn new() -> Self {
        return Self { modules: vec![] };
    }

    fn add_fut(&mut self, fut: impl Future<Output = String> + Send + 'static) -> &mut Batonik {
        self.modules.push(Box::pin(fut));
        self
    }

    fn add_str(&mut self, s: impl ToString) -> &mut Batonik {
        let s = s.to_string();
        self.modules.push(Box::pin(async move { s }));
        self
    }

    pub fn run(self) {
        let rt = tokio::runtime::Runtime::new().unwrap();

        rt.block_on(async {
            let mut set = JoinSet::new();
            let mut n = 0;

            for m in self.modules {
                set.spawn(async move {
                    let res = m.await;
                    return (n, res);
                });

                n = n + 1;
            }

            let results = set.join_all().await;
            println!("{results:?}");
        })
    }
}

impl<Fut: Future<Output = String> + Send + 'static> FutureModule for Fut {
    type This = Fut;
    fn get_add(&self) -> ModuleAdder<Fut> {
        Batonik::add_fut
    }
}

impl<S: ToString> StringModule for &S {
    type This = S;
    fn get_add(&self) -> ModuleAdder<S> {
        Batonik::add_str
    }
}

#[macro_export]
macro_rules! batonik {
    ($($module:expr),* $(,)?) => {{
        let mut app = $crate::Batonik::new();
        $crate::batonik!(&mut app => [
            $($module),*
        ]);
        app
    }};

    ($app:expr => [$($module:expr),* $(,)?]) => {{
        use $crate::*;
        let mut app = $app;
        $(
        let module = $module;
        let mut app = (&module).get_add()(app, module);
        )*
        app
    }};
}
