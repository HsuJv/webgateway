use super::protocol::*;
use super::*;

pub type ConnectCb = fn(&mut RdpInner);
pub type FailCb = fn(&mut RdpInner, &str);

pub trait RdpInitializer {
    fn hello(&mut self, rdp: &mut RdpInner);
    fn do_input(&mut self, rdp: &mut RdpInner);
}

#[derive(Debug)]
enum State {
    Init,
    X224,
    Nla,
    Core,
    Disconnected,
}

pub struct RdpInner {
    state: State,
    initializer: Option<Box<dyn RdpInitializer>>,
    // core: Box<dyn super::RdpInitializer>,
    pub reader: StreamReader,
    pub writer: StreamWriter,
    outs: Vec<RdpOutput>,
    require: usize,
    need_nla: bool,
}

impl RdpInner {
    pub fn new() -> Self {
        RdpInner {
            state: State::Init,
            initializer: None,
            // core: None,
            reader: StreamReader::new(Vec::with_capacity(10), 0),
            writer: StreamWriter::new(Vec::with_capacity(1024)),
            outs: Vec::with_capacity(10),
            require: 0,
            need_nla: false,
        }
    }

    pub fn init(&mut self) {
        if self.initializer.is_some() {
            panic!("inited");
        }
        self.move_next();
    }

    pub fn do_input(&mut self, buf: Vec<u8>) {
        if let State::Disconnected = self.state {
            return;
        };
        self.reader.append(buf);
        while self.reader.remain() >= self.require {
            match self.state {
                State::Disconnected => {
                    unimplemented!("TO DO, clean up")
                }
                State::Core => {
                    unimplemented!("TO DO")
                }
                _ => {
                    let mut handler = self.initializer.take().unwrap();

                    handler.do_input(self);
                    if self.initializer.is_none() {
                        self.initializer = Some(handler);
                    }

                    // trace!("left {}, require {}", self.reader.remain(), self.require);
                    if let State::Disconnected = self.state {
                        break;
                    }
                }
            }
        }
    }

    pub fn get_output(&mut self) -> Option<Vec<RdpOutput>> {
        if self.outs.is_empty() && self.writer.buf.is_empty() {
            None
        } else {
            let mut out = Vec::with_capacity(self.outs.len());
            // trace!("Get {} output", self.outs.len());
            for o in self.outs.drain(..) {
                out.push(o);
            }
            if !self.writer.buf.is_empty() {
                out.push(RdpOutput::WsBuf(self.writer.buf.clone()));
                self.writer.buf.clear();
            }
            Some(out)
        }
    }

    pub fn flush(&mut self) {
        if !self.writer.buf.is_empty() {
            self.outs.push(RdpOutput::WsBuf(self.writer.buf.clone()));
            self.writer.buf.clear();
        }
    }

    pub fn close(&mut self) {
        self.state = State::Disconnected;
    }
}

impl RdpInner {
    fn move_next(&mut self) {
        trace!("State move from {:?} to the next", self.state);
        match self.state {
            State::Init => {
                let mut x224 = x224::X224::new(Self::move_next, Self::disconnect_with_err);
                x224.hello(self);
                self.initializer = Some(Box::new(x224));
                self.state = State::X224;
            }
            State::X224 => {
                if self.need_nla {
                    self.start_tls();
                    let mut nla = nla::Nla::new(
                        Self::move_next,
                        Self::disconnect_with_err,
                        "sonicwall",
                        "sonicwall",
                        "",
                        "webrdp",
                    );
                    nla.hello(self);
                    self.initializer = Some(Box::new(nla));
                    self.state = State::Nla;
                } else {
                    self.state = State::Core;
                }
            }
            State::Nla => {
                self.flush();
                self.state = State::Core;
            }
            State::Core => unreachable!(),
            State::Disconnected => unreachable!(),
        }
    }

    fn disconnect_with_err(&mut self, err: &str) {
        error!("{:#?}", err);
        self.state = State::Disconnected;
        self.outs.push(RdpOutput::Err(err.to_string()));
    }

    pub fn need_wait(&mut self, len: usize) -> bool {
        self.reader.remain() < len
    }

    pub fn wait(&mut self, len: usize) {
        self.require = len;
    }

    pub fn nla(&mut self, need: bool) {
        self.need_nla = need;
    }

    fn start_tls(&mut self) {
        self.outs.push(RdpOutput::RequireSSL);
    }
}
