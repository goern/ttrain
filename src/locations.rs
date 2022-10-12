#[derive(Debug)]
pub struct City {
    pub name: String,
    pub lat: f64,
    pub lon: f64,
}

impl City {
    pub fn distance(&self, other: &City) -> f64 {
        let lat1 = self.lat.to_radians();
        let lon1 = self.lon.to_radians();
        let lat2 = other.lat.to_radians();
        let lon2 = other.lon.to_radians();

        let dlat = lat2 - lat1;
        let dlon = lon2 - lon1;

        let a = (dlat / 2.0).sin().powi(2) + lat1.cos() * lat2.cos() * (dlon / 2.0).sin().powi(2);
        let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());

        6371.0 * c
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distance() {
        let cologne = City {
            name: "Cologne".to_string(),
            lat: 50.938361,
            lon: 6.959974,
        };
        let munich = City {
            name: "Munich".to_string(),
            lat: 48.1351253,
            lon: 11.5819806,
        };

        let d = cologne.distance(&munich);
        assert_eq!(d, 456.3478636153144);
    }
}
