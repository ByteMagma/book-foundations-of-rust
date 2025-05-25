use super::itinerary_item::ItineraryItem;

#[derive(Debug, Clone)] 
pub struct Itinerary {
    pub items: Vec<ItineraryItem>,
}