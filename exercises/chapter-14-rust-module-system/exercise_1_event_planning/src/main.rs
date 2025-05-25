mod itinerary;
mod itinerary_item;
use itinerary::Itinerary;
use itinerary_item::ItineraryItem;

fn main() {
    let beach_trip = ItineraryItem {
        destination: String::from("Waikiki Beach"),
        departure_date: String::from("2025-07-04"),
        return_date: String::from("2025-07-11"),
    };
    let mountain_trip = ItineraryItem {
        destination: String::from("Maui Mountains"),
        departure_date: String::from("2025-07-11"),
        return_date: String::from("2025-07-18"),
    };
    let itinerary = Itinerary {
        items: vec![beach_trip, mountain_trip],
    };
    println!("Itinerary: {:#?}", itinerary);
}
