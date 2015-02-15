#![feature(env)]
#![feature(core)]
#![feature(collections)]

extern crate "sass-rs" as sass_rs;
use sass_rs::sass_context::SassFileContext;
use sass_rs::sass_function::*;
use sass_rs::sass_value::*;

fn foo(_: * const SassValueRaw) -> * mut SassValueRaw {
  SassValue::raw_from_str("foo-ed")
}

fn compile(filename:&str) {
  let mut file_context = SassFileContext::new(filename);
  let fns = vec![SassFunctionCallback::from_sig_fn(String::from_str("foo()"),(foo as SassFunction))];
  file_context.sass_context.sass_options.set_sass_functions(fns);
  let out = file_context.compile();
  match out {
    Ok(css) => println!("------- css  ------\n{}\n--------", css),
    Err(err) => println!("{}", err)
  };
}

pub fn main() {
  let mut args = std::env::args();
  let _ = args.next();
  let file = args.next().expect("Please pass in a file name");
  println!("Compiling sass file: `{}`.", file);
  compile(file.as_slice());
}