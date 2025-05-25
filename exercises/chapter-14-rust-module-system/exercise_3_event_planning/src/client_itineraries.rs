use std::collections::HashMap;
use crate::models::Itinerary;

pub type ClientItineraries = HashMap<String, Itinerary>; 

pub fn add_itinerary(itineraries: &mut ClientItineraries, client: &str, itinerary: Itinerary) {
  itineraries.insert(client.to_string(), itinerary);
}