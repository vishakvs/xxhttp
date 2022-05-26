//! Fast, minimalist web framework for Rust

/// Describes the xxhttp server
pub struct Server {
    ipv4addr: u32,
}

impl Server {
    /// Get bounded IP of Server instance
    pub fn getip(&self) -> u32 {
        self.ipv4addr
    }
}

/// Returns the instance of Server.
pub fn xxhttp_server() -> Server {
    let s = Server { ipv4addr: 1 };
    s
}

#[cfg(test)]
mod tests {
    //use super::*;
    #[test]
    fn it_works() {
        assert_eq!(1, 1);
    }
}
