use crate::{console_log, log};

use super::protocol::*;
use super::*;

#[derive(Debug)]
enum State {
    Init,
    X224,
    Nla,
    Disconnected,
}

pub struct RdpInner {
    state: State,
    engine: Option<Box<dyn super::Engine>>,
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
            engine: None,
            reader: StreamReader::new(Vec::with_capacity(10), 0),
            writer: StreamWriter::new(Vec::with_capacity(1024)),
            outs: Vec::with_capacity(10),
            require: 0,
            need_nla: false,
        }
    }

    pub fn init(&mut self) {
        if self.engine.is_some() {
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
            let mut handler = self.engine.take().unwrap();

            handler.do_input(self);
            if self.engine.is_none() {
                self.engine = Some(handler);
            }

            // console_log!("left {}, require {}", self.reader.remain(), self.require);
            if let State::Disconnected = self.state {
                break;
            }
        }
    }

    pub fn get_output(&mut self) -> Option<Vec<RdpOutput>> {
        if self.outs.is_empty() && self.writer.buf.is_empty() {
            None
        } else {
            let mut out = Vec::with_capacity(self.outs.len());
            // console_log!("Get {} output", self.outs.len());
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

    pub fn close(&mut self) {
        self.state = State::Disconnected;
    }
}

impl RdpInner {
    fn move_next(&mut self) {
        console_log!("State move from {:?} to the next", self.state);
        match self.state {
            State::Init => {
                let mut x224 = x224::X224::new(Self::move_next, Self::disconnect_with_err);
                x224.hello(self);
                self.engine = Some(Box::new(x224));
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
                    self.engine = Some(Box::new(nla));
                    self.state = State::Nla;
                } else {
                    unimplemented!("Only nla is supported now");
                }
            }
            State::Nla => {}
            State::Disconnected => {}
        }
    }

    fn disconnect_with_err(&mut self, err: &str) {
        console_log!("{:#?}", err);
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
