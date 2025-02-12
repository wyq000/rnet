use pyo3::prelude::*;
use pyo3_stub_gen::derive::gen_stub_pyclass;

/// A IP address.
#[gen_stub_pyclass]
#[pyclass(eq)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct IpAddr(std::net::IpAddr);

impl From<std::net::IpAddr> for IpAddr {
    fn from(ip: std::net::IpAddr) -> Self {
        IpAddr(ip)
    }
}
