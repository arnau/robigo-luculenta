
// Robigo Luculenta -- Proof of concept spectral path tracer in Rust
// Copyright (C) 2014 Ruud van Asseldonk
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program. If not, see <http://www.gnu.org/licenses/>.

use std::io::{BufferedWriter, IoResult};
use std::io::net::ip::SocketAddr;
use std::io::net::tcp::TcpStream;
use vector3::Vector3;

struct Sender {
    /// The IP address and port of the master instance.
    master_addr: SocketAddr
}

impl Sender {
    /// Creates a new sender that sends render results to the specified master.
    pub fn new(master_addr: SocketAddr) -> Sender {
        Sender {
            master_addr: master_addr
        }
    }

    /// Attempts to connect to the master instance, and sends the buffer.
    pub fn send(&self, tristimuli: &[Vector3]) -> IoResult<()> {
        // There is this nice SocketAddr type, but then TcpStream::connect takes
        // a string and integer, too bad :(
        let host = format!("{}", self.master_addr.ip);

        let tcp_stream = try!(TcpStream::connect(host[], self.master_addr.port));
        // Buffer writes, we would not want to issue a syscall for every pixel.
        let mut writer = BufferedWriter::new(tcp_stream);
        for tri in tristimuli.iter() {
            try!(writer.write_le_f32(tri.x));
            try!(writer.write_le_f32(tri.y));
            try!(writer.write_le_f32(tri.z));
        }
        Ok(())
    }
}
