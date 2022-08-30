use std::rc::Rc;

// The Target defines the domain-specfic interface used by the client code.
trait Target {
    fn get_request(&self) -> String {
        String::from("Target: The default target's behavior.")
    }
}

struct DefaultTarget;
impl Target for DefaultTarget {}

// The Adaptee contains some useful behavior, but its interface is
// incompatible with the existing client code. The Adaptee needs some
// adaptation before the client code can use it.
struct Adaptee {
    req_str: String,
}

impl Adaptee {
    fn new(s: String) -> Self {
        Self { req_str: s }
    }
    fn specfic_request(&self) -> String {
        format!("specfic request: {}", self.req_str)
    }
}

// The Adapter makes the Adaptee's interace compatible with the Target's
// interace.
struct Adapter {
    adaptee: Rc<Adaptee>,
}

impl Adapter {
    fn new(a: Rc<Adaptee>) -> Adapter {
        Adapter { adaptee: a }
    }

}

impl Target for Adapter {
    fn get_request(&self) -> String {
        format!("Adapter: {}", self.adaptee.specfic_request())
    }
}

// The client code supports all classes that follow the Target trait.
struct Client;
impl Client {
    fn client_code<T: Target>(target: &T) {
        println!("{}", target.get_request());
    }
}

fn main() {
    println!("Client: I can work just fine with the Target objects:");
    Client::client_code(&DefaultTarget {});
    let adaptee = Rc::new(Adaptee::new("hello world".to_string()));
    println!("Client: The Adaptee class has a weird interface. See, I don't understand it:");
    println!("Adaptee: {}", adaptee.specfic_request());

    println!("Client: But I can work with it via the Adapter:");
    let adapter = Adapter::new(adaptee);
    Client::client_code(&adapter);
}
