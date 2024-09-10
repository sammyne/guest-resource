mod bindings {
    wit_bindgen::generate!();

    use crate::App;

    export!(App);
}

use crate::bindings::exports::helloworld::example::greeter_api::GuestGreeter;
use bindings::exports::helloworld::example::api::Guest;

struct App;

struct Greeter;


impl Guest for App {
    type Greeter = Greeter;
    
    fn hi(who:String) {
        todo!()
    }
    
    fn hello_world(g:Greeter) {
    }
}

impl GuestGreeter for Greeter {
    fn new() -> Self {
        Greeter
    }

    fn greet(&self,who:String) {
        println!("hello {who}");
    }
}