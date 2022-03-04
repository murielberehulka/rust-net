use rust_net::{Server, Socket};

struct Context {
    a: String
}

fn main() {
    let context = Context {
        a: "test".to_string()
    };
    let mut server = Server::new(Default::default(), context);
    server.add_get_route("test", |_, socket, _| {
        socket.send_200(b"Test");
    });
    server.add_post_route("test", |context, socket, _| {
        socket.send_200(context.a.as_bytes());
    });
    println!("Server running ...");
    server.run();
}