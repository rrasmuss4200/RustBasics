struct Starship {
    is_docking: bool,
    payload_mass_in_kg: u32,
    destination: String,
    launch_site: String,
}
// To IMPLEMENT methods
impl Starship {
    fn new(is_docking: bool, payload_mass_in_kg: u32, destination: String, launch_site: String) -> Starship {
        if payload_mass_in_kg > 200000 {
            // Not how are errors are typically handled
            panic!("Too heavy to launch.")
        } else {
            Starship {
                is_docking,
                payload_mass_in_kg,
                destination,
                launch_site,
            }
        }
    }
    fn is_grasshopper(&self) -> bool {
        if self.destination == self.launch_site {true}
        else {false}
    }
    fn calc_fees(&self, dollars_per_kg: u32, docking_fee: u32) -> u32 {
        let mut travel_fees: u32 = self.payload_mass_in_kg * dollars_per_kg;
        if self.is_docking {travel_fees += docking_fee} else {return travel_fees;}
        travel_fees
    }
}

fn main() {
    use ::*;
    let destination: String = String::from("Mars");
    let launch_site: String = String::from("Moon Base");
    let ship = Starship::new(true,50000,destination, launch_site); //create instance of starship
    assert!(!Starship::is_grasshopper(&ship));
    assert_eq!(ship.calc_fees(5,500),250500);

}

