use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

use blake2::digest::generic_array::arr;
use ordered_float::OrderedFloat;

#[derive(Debug, Clone)]
struct CommunicationLink {
    from: usize,     // The satellite (node) initiating the contact (sender)
    to: usize,       // The satellite (node) receiving the contact (receiver)
    start_time: f64, // The earliest time this communication link is available
    end_time: f64,   // The latest time this contact is available
    latency: f64,    // The time delay for data transmission over this link
    bandwidth: f64,  // The capacity of the link (not used in shortest path search)
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct RouteNode {
    id: usize,                       // Current satellite (node) in the route
    arrival_time: OrderedFloat<f64>, // The earliest time data can arrive at this node
    path: Vec<usize>,                // The sequence of satellites visited so far in this route
}

/**
 * Flipping the comparison in order to turn max-heap to min-heap.
 */
impl Ord for RouteNode {
    fn cmp(&self, other: &Self) -> Ordering {
        other.arrival_time.cmp(&self.arrival_time)
    }
}

impl PartialOrd for RouteNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other)) // using cmp directly
    }
}

#[derive(Clone, Debug)]
enum CGRState {
    IDLE,
    DiscoverCommunicationLinks,
    SelectRoute,
    TransmitData,
    HoldData,
    Retransmit,
    Delivered,
    Failed,
}

struct CGR {
    communication_links: Vec<CommunicationLink>,
    // Map a Sat Id to a list of communication links it points to
    adjacency_list: HashMap<usize, Vec<CommunicationLink>>,
    // The state within the Constrained Geographic Routing Algorithm State Machine
    state: CGRState,
}

enum CGREvent {
    NewPacketArrived,
    CommunicationLinksAvailable,
    NoCommunicationLinksAvailable,
    RouteComputed,
    CommunicationLinkLost,
    CommunicationLinkRestored,
    Timeout,
    DataSent,
}

impl CGR {
    fn new(communication_links: Vec<CommunicationLink>) -> Self {
        let mut adjacency_list = HashMap::new();
        for communication_link in &communication_links {
            adjacency_list
                .entry(communication_link.from)
                .or_insert_with(Vec::new)
                .push(communication_link.clone());
        }
        Self {
            communication_links,
            adjacency_list,
            state: CGRState::IDLE,
        }
    }

    fn transition(&mut self, event: &CGREvent) {
        self.state = match (self.state.clone(), event) {
            (CGRState::IDLE, CGREvent::NewPacketArrived) => CGRState::DiscoverCommunicationLinks,

            (CGRState::DiscoverCommunicationLinks, CGREvent::CommunicationLinksAvailable) => {
                CGRState::SelectRoute
            }
            (CGRState::DiscoverCommunicationLinks, CGREvent::NoCommunicationLinksAvailable) => {
                CGRState::HoldData
            }

            (CGRState::SelectRoute, CGREvent::RouteComputed) => CGRState::TransmitData,
            // the route unavailable state **
            (CGRState::TransmitData, CGREvent::DataSent) => CGRState::Delivered,
            (CGRState::TransmitData, CGREvent::CommunicationLinkLost) => CGRState::HoldData,

            (CGRState::HoldData, CGREvent::Timeout) => CGRState::Failed,
            (CGRState::HoldData, CGREvent::CommunicationLinkRestored) => CGRState::Retransmit,

            (CGRState::Retransmit, CGREvent::DataSent) => CGRState::Delivered,

            _ => self.state.clone(), // for now the default state is to remain in current state
        }
    }

    fn find_best_route(
        &mut self,
        source_satellite: usize,
        destination_satellite: usize,
        arrival_time: f64,
    ) -> Vec<usize> {
        // define a Path type on the return
        let mut queue = BinaryHeap::new();
        let mut path_so_far = Vec::new().push(source_satellite);
        queue.push((
            source_satellite,
            OrderedFloat::from(arrival_time),
            path_so_far,
        ));
    }
}
