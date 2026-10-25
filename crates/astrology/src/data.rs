//! City constants: the 10 city panchang slugs observed on the target site.

/// One supported city page.
#[derive(Debug, Clone, Copy)]
pub struct City {
    pub slug: &'static str,
    pub name: &'static str,
    pub state: &'static str,
    pub lat: f64,
    pub lon: f64,
    pub tz_hours: f64,
}

/// Cities exactly as observed (homepage footer, sitemap); Bangalore uses
/// the slug the site uses (`bangalore`) while the display name is Bengaluru.
pub const CITIES: [City; 10] = [
    City { slug: "mumbai", name: "Mumbai", state: "Maharashtra", lat: 19.0760, lon: 72.8777, tz_hours: 5.5 },
    City { slug: "delhi", name: "New Delhi", state: "Delhi", lat: 28.6139, lon: 77.2090, tz_hours: 5.5 },
    City { slug: "pune", name: "Pune", state: "Maharashtra", lat: 18.5204, lon: 73.8567, tz_hours: 5.5 },
    City { slug: "ahmedabad", name: "Ahmedabad", state: "Gujarat", lat: 23.0225, lon: 72.5714, tz_hours: 5.5 },
    City { slug: "jaipur", name: "Jaipur", state: "Rajasthan", lat: 26.9124, lon: 75.7873, tz_hours: 5.5 },
    City { slug: "bangalore", name: "Bengaluru", state: "Karnataka", lat: 12.9716, lon: 77.5946, tz_hours: 5.5 },
    City { slug: "kolkata", name: "Kolkata", state: "West Bengal", lat: 22.5726, lon: 88.3639, tz_hours: 5.5 },
    City { slug: "noida", name: "Noida", state: "Uttar Pradesh", lat: 28.5355, lon: 77.3910, tz_hours: 5.5 },
    City { slug: "chennai", name: "Chennai", state: "Tamil Nadu", lat: 13.0827, lon: 80.2707, tz_hours: 5.5 },
    City { slug: "hyderabad", name: "Hyderabad", state: "Telangana", lat: 17.3850, lon: 78.4867, tz_hours: 5.5 },
];

pub fn city_by_slug(slug: &str) -> Option<&'static City> {
    CITIES.iter().find(|c| c.slug == slug)
}
