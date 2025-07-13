pub mod modules;
use clap::{Arg, Command};
pub use colored;

use std::future::Future;
use std::path::PathBuf;
use std::pin::Pin;

use tokio::task::JoinSet;

type RawModule = Pin<Box<dyn Future<Output = String> + Send + 'static>>;

pub struct Batonik {
    modules: Vec<RawModule>,
    final_space: bool,
}

type ModuleAdder<M> = fn(&mut Batonik, M) -> &mut Batonik;

pub trait AutoFutureModule {
    type This;
    fn get_add(&self) -> ModuleAdder<Self::This>;
}

pub trait AutoStringModule {
    type This;
    fn get_add(&self) -> ModuleAdder<Self::This>;
}

pub trait AutoModuleModule {
    type This;
    fn get_add(&self) -> ModuleAdder<Self::This>;
}

pub trait Module {
    fn run(self) -> impl Future<Output = String> + Send + 'static;
}

impl Batonik {
    pub fn new() -> Self {
        return Self {
            modules: vec![],
            final_space: true,
        };
    }

    pub fn final_space(&mut self, yes: bool) {
        self.final_space = yes;
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

    fn add_module(&mut self, m: impl Module) -> &mut Batonik {
        self.modules.push(Box::pin(m.run()));
        self
    }

    pub fn render(self) -> String {
        let rt = tokio::runtime::Runtime::new().unwrap();

        let mut res = String::new();
        rt.block_on(async {
            let mut set = JoinSet::new();

            for (n, module) in self.modules.into_iter().enumerate() {
                set.spawn(async move {
                    let res = module.await;
                    return (n, res);
                });
            }

            let mut results = set.join_all().await;
            results.sort_by(|&(left_idx, _), (right_idx, _)| left_idx.cmp(right_idx));

            for (idx, module_res) in results {
                eprintln!("{idx:?} -> {module_res}");
                if module_res.is_empty() {
                    continue;
                } else {
                    if idx != 0 {
                        res.push(' ');
                    }
                    res.push_str(&module_res);
                }
            }

            if self.final_space {
                res.push(' ');
            }
        });

        return res;
    }

    pub fn run(self) {
        let matches = Command::new("batonik")
            .subcommand_required(true)
            .arg_required_else_help(true)
            .subcommand(
                Command::new("init")
                    .about("Prints the shell function used to execute batonik")
                    .arg(
                        Arg::new("shell")
                            .help("The shell to generate the function for")
                            .required(true)
                            .value_parser(["bash", "fish", "zsh"]),
                    )
                    .arg(
                        Arg::new("print-full-init")
                            .long("print-full-init")
                            .help("Print the full shell init script")
                            .action(clap::ArgAction::SetTrue),
                    ),
            )
            .subcommand(Command::new("prompt").about("Prints the full starship prompt"))
            .get_matches();

        match matches.subcommand() {
            Some(("init", sub_matches)) => {
                let shell = sub_matches.get_one::<String>("shell").unwrap();
                let self_cmd = PathBuf::from(std::env::args().next().unwrap())
                    .canonicalize()
                    .expect("failed to canonicalize argv0")
                    .to_string_lossy()
                    .to_string();

                match shell.as_str() {
                    "fish" => {
                        if *sub_matches
                            .get_one::<bool>("print-full-init")
                            .unwrap_or(&false)
                        {
                            print!(
                                "function fish_prompt
    {self_cmd} prompt
end"
                            );
                        } else {
                            print!("source ({self_cmd} init fish --print-full-init | psub)");
                        }
                    }
                    "bash" => {
                        todo!();
                    }
                    "zsh" => {
                        todo!();
                    }
                    _ => unreachable!(),
                };
            }
            Some(("prompt", _)) => {
                let res = self.render();
                print!("{res}");
            }
            _ => unreachable!(),
        }
    }
}

impl<Fut: Future<Output = String> + Send + 'static> AutoFutureModule for Fut {
    type This = Fut;
    fn get_add(&self) -> ModuleAdder<Fut> {
        Batonik::add_fut
    }
}

impl<S: ToString> AutoStringModule for &S {
    type This = S;
    fn get_add(&self) -> ModuleAdder<S> {
        Batonik::add_str
    }
}

impl<M: Module> AutoModuleModule for M {
    type This = M;

    fn get_add(&self) -> ModuleAdder<Self::This> {
        Batonik::add_module
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
