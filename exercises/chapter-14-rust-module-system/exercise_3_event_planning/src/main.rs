mod models;
mod client_itineraries;
use models::Itinerary;
use models::ItineraryItem;

fn main() {
    let mut itineraries = client_itineraries::ClientItineraries::new();

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

    client_itineraries::add_itinerary(&mut itineraries, "Frank Smith", itinerary);

    let liberty_trip = ItineraryItem {
        destination: String::from("Statue of Liberty"),
        departure_date: String::from("2025-08-10"),
        return_date: String::from("2025-08-10"),
    };
    let times_square_trip = ItineraryItem {
        destination: String::from("Times Square"),
        departure_date: String::from("2025-08-11"),
        return_date: String::from("2025-08-11"),
    };

    let itinerary = Itinerary {
        items: vec![liberty_trip, times_square_trip],
    };

    client_itineraries::add_itinerary(&mut itineraries, "Mary McGuire", itinerary);

    println!("Client Itineraries: {:#?}", itineraries);
}
