//! rtps-udp provides the UDP/IP platform specfic implementation of the RTPS
//! specification.
//!
//! See Section 9 of the specification for more details.

use std::net::{IpAddr, Ipv4Addr, SocketAddr};

struct PortNumberParams {
    domain_id_gain: u16,
    participant_id_gain: u16,
    port_base_number: u16,
    additional_offset_d0: u16,
    additional_offset_d1: u16,
    additional_offset_d2: u16,
    additional_offset_d3: u16,
}

impl PortNumberParams {
    const fn spdp_well_known_multicast_port(&self, domain_id: u16) -> u16 {
        self.port_base_number + self.domain_id_gain * domain_id + self.additional_offset_d0
    }

    const fn spdp_well_known_unicast_port(&self, domain_id: u16, participant_id: u16) -> u16 {
        self.port_base_number
            + self.domain_id_gain * domain_id
            + self.additional_offset_d1
            + self.participant_id_gain * participant_id
    }

    const fn user_defined_multicast_port(&self, domain_id: u16) -> u16 {
        self.port_base_number + self.domain_id_gain * domain_id + self.additional_offset_d2
    }

    const fn user_defined_unicast_port(&self, domain_id: u16, participant_id: u16) -> u16 {
        self.port_base_number
            + self.domain_id_gain * domain_id
            + self.additional_offset_d3
            + self.participant_id_gain * participant_id
    }
}

impl Default for PortNumberParams {
    fn default() -> Self {
        Self {
            domain_id_gain: 250,
            participant_id_gain: 2,
            port_base_number: 7400,
            additional_offset_d0: 0,
            additional_offset_d1: 10,
            additional_offset_d2: 1,
            additional_offset_d3: 11,
        }
    }
}

fn default_multicast_locator(domain_id: u16, params: Option<PortNumberParams>) -> SocketAddr {
    let params = params.unwrap_or_default();
    let ip = IpAddr::V4(Ipv4Addr::new(239, 255, 0, 1));
    let port = params.spdp_well_known_multicast_port(domain_id);
    SocketAddr::new(ip, port)
}
