
fn main() {

}

///Supported Servo Types
enum ServoType {

}

///Main Servo Struct
struct Servo {
    pin: i32,

    min_angle: i32,
    max_angle: i32,

    servo_type: ServoType,


}

impl Servo {
    ///Initializes all environment variables necessary for the servo
    pub fn init() {

    }

    ///Sets the angle of the servo to an angle defined in radians
    pub fn set_angle_deg(angle: &f32) -> Result<(), std::error> {
        //Checks it is within bounds and passes to generic angle calc

        //Sets the calculated angle of the servo
    }

    ///Sets the angle of the servo to an angle defined in radians
    pub fn set_angle_rad(angle: &f32) -> Result<(), std::error> {
        //Converts to degrees, and passes to the degree angle function
    }

    ///Calculates the duty cycle and frequency required to achieve the necessary angle
    fn calculate_angle(angle: &f32) -> (i32, i32) {
        let duty_cycle = 0;
        let frequency = 0;


    }
}