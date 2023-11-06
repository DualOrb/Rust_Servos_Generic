use std::f64::consts::PI;

fn main() {

}

///Supported Servo Types
enum ServoType {
    Miuzuki20kg,
    Custom
}

///Main Servo Struct
struct Servo {
    pin: i32,          //The GPIO pin the servo is connected to

    min_angle: f32,    //Minimum and Maximum angle the servo supports (degrees
    max_angle: f32,

    frequency: i32,    //The frequency in ms that the servo supports (repetition rate)

    min_pulse_width: f32,   //The min and max pulse widths that correspond with the angles inms
    max_pulse_width: f32,

    servo_type: ServoType,  //A type for a specific servo motor if supported. If not -> type = custom
}

impl Servo {
    ///Initializes raspberry pi related services
    pub fn init() -> Result<(), std::error> {

    }

    ///Sets the angle of the servo to an angle defined in radians
    pub fn set_angle_deg(&self, angle: &f32) {
        //Checks it is within bounds and passes to generic angle calc for servo
        if self.min_angle > *angle && angle > &self.max_angle {
            panic!("Angle provided is out of bounds");
        }

        let pulse_width = self.calculate_pulse_widths(angle);

        //Set the GPIO pin with the necessary information

    }

    ///Sets the angle of the servo to an angle defined in radians
    pub fn set_angle_rad(&self, angle: &f32) {
        //Converts to degrees, and passes to the degree angle function
        let degrees = angle * 180 / PI;

        return self.set_angle_deg(&degrees);
    }

    ///Calculates the pulse widths necessary to achieve the angle
    fn calculate_pulse_widths(&self, angle: &f32) -> (f32) {
        return ((self.max_pulse_width - self.min_pulse_width)
            / self.max_angle * angle)
            + self.min_pulse_width;
    }
}