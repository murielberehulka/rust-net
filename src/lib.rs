use mio::{Events, Interest, Poll};
pub use mio::net::{TcpListener, TcpStream};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::collections::HashMap;

pub mod util;
mod socket;
#[macro_user]
mod response;
mod settings;
mod static_files;
pub use response::*;
pub use socket::*;
pub use settings::*;
pub use static_files::*;

pub type RouteFunction<T> = fn(&mut T, &mut TcpStream, Vec<u8>);
struct Routes<T> {
    get: HashMap<Vec<u8>, RouteFunction<T>>,
    post: HashMap<Vec<u8>, RouteFunction<T>>
}

pub struct Server<T> {
    address: SocketAddr,
    clients: Sockets,
    static_files: Option<StaticFiles>,
    routes: Routes<T>,
    context: T
}

impl<T> Server<T> {
    pub fn new(settings: Settings, context: T) -> Self {
        Self {
            address: SocketAddr::new(
                IpAddr::V4(Ipv4Addr::new(settings.address[0], settings.address[1], settings.address[2], settings.address[3])), 
                settings.port
            ),
            clients: Sockets::new(),
            static_files: match settings.static_files {
                Some(static_files_settings) => 
                    Some(StaticFiles::new(static_files_settings.root_path, static_files_settings.enable_cache)),
                None => None
            },
            routes: Routes {
                get: HashMap::new(),
                post: HashMap::new()
            },
            context
        }
    }
    pub fn add_get_route<S: AsRef<str>>(&mut self, path: S, func: RouteFunction<T>) {
        self.routes.get.insert(Vec::from(path.as_ref().as_bytes()), func);
    }
    pub fn add_post_route<S: AsRef<str>>(&mut self, path: S, func: RouteFunction<T>) {
        self.routes.post.insert(Vec::from(path.as_ref().as_bytes()), func);
    }
    pub fn run(self) {
        let mut poll = Poll::new().unwrap();
        let mut listener = TcpListener::bind(self.address).unwrap();
        poll.registry().register(&mut listener, LISTENER_EVENT_TOKEN, Interest::READABLE).unwrap();

        let mut clients = self.clients;
        let static_files = self.static_files;
        let mut routes = self.routes;
        let mut context = self.context;

        let mut events = Events::with_capacity(1024);
        loop {
            poll.poll(&mut events, None).unwrap();
            for event in events.iter() {
                Self::new_event(&event, &poll, &mut listener, &mut clients, &static_files, &mut routes, &mut context);
            }
        }
    }
    fn new_event(
        event: &mio::event::Event,
        poll: &mio::Poll,
        listener: &mut TcpListener,
        sockets: &mut Sockets,
        static_files: &Option<StaticFiles>,
        routes: &mut Routes<T>,
        context: &mut T
    ) {
        match event.token().0 {
            0 => {
                match listener.accept() {
                    Ok((socket, _)) => {
                        sockets.insert(poll, socket);
                        poll.registry().reregister(listener, LISTENER_EVENT_TOKEN, Interest::READABLE).unwrap();
                    },
                    Err(e) => panic!("{}",e)
                }
            },
            token => {
                if let Some(socket) = sockets.get_mut(token) {
                    match socket.read_stream() {
                        Some(data) => {
                            if data.len() > 0 {
                                match data[0] {
                                    // If the request start with G that means it's a GET request
                                    util::U8_G => {
                                        let mut path = util::read_data_until_space(&data, 5);
                                        if path.len() == 0 {
                                            path = util::INDEX;
                                        }
                                        match static_files {
                                            //if static files is enabled
                                            Some(static_files) => match static_files.cache {
                                                //if static files cache is enabled
                                                Some(ref cache) => match cache.get(path) {
                                                    Some(file) =>
                                                        socket.send_file(file),
                                                    //if file dont exists send program get route
                                                    None => match routes.get.get_mut(path) {
                                                        Some(func) => func(context, socket, data),
                                                        //if route dont exists send 404 error
                                                        None => socket.send_404()
                                                    }
                                                },
                                                //if static files cache is disabled, read file
                                                None => match read_file(static_files.root_path, path) {
                                                    Some(file) => socket.send_file(&file),
                                                    //if file dont exists send program get route
                                                    None => match routes.get.get_mut(path) {
                                                        Some(func) => func(context, socket, data),
                                                        //if route dont exists send 404 error
                                                        None => socket.send_404()
                                                    }
                                                }
                                            },
                                            //if static files is disabled, send route
                                            None => match routes.get.get_mut(path) {
                                                Some(func) => func(context, socket, data),
                                                None => socket.send_404()
                                            }
                                        }
                                    },
                                    // If the request start with P that means it's a POST request
                                    util::U8_P => {
                                        match routes.post.get_mut(util::read_data_until_space(&data, 6)) {
                                            Some(func) => func(context, socket, data),
                                            None => socket.send_404()
                                        }
                                    },
                                    // If the request start with O that means it's a OPTIONS request (treated like a post request)
                                    util::U8_O => {
                                        match routes.post.get_mut(util::read_data_until_space(&data, 9)) {
                                            Some(func) => func(context, socket, data),
                                            None => socket.send_404()
                                        }
                                    },
                                    _ => {}
                                }
                            }
                        },
                        None => {}
                    }
                }
                sockets.remove(&poll, token);
            }
        }
    }
}
