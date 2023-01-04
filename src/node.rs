

#[derive(Clone)]
pub struct Node {
    //inner: Rc<RefCell<NodeInner>>,
    //pub timer: Timer,
}

impl Node {
    pub fn new(addr: SocketAddr) -> Node {

    }

    pub fn serve () {
        //let srv = socket.incoming().for_each(move |(tcpstream, addr)| {
    }

    //fn process_msg(inner: Rc<RefCell<NodeInner>>, msg: Msg, tx: Tx, handle: Handle) -> Result<(), io::Error> {

    //TODO
    pub fn handle_message() {

    }

    // pub fn broadcast(&self, m: String) {
    //     self.inner.borrow().broadcast(m)
    // }

}

// struct NodeInner {
//     pub id: Uuid,
//     pub addr: SocketAddr,
//     pub peers: HashMap<Uuid, (Tx, SocketAddr)>,
//     rng: Rc<RefCell<ThreadRng>>,
// }