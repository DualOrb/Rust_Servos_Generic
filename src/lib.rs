use crate::servo::Servo;
mod servo;

fn main() {

}

//Tests
#[cfg(test)]
mod tests {
    use crate::servo::{Servo, ServoType};

    #[test]
    fn test_initialize() {
        let servo = Servo{
            pin: 18,
            min_angle: 0.0,
            max_angle: 180.0,
            frequency: 50,
            min_pulse_width: 2.0,
            max_pulse_width: 12.0,

            servo_type: ServoType::Custom,
        };

        assert_eq!(servo.pin, 18);
        assert_eq!(servo.min_angle, 0.0);
        assert_eq!(servo.max_angle, 180.0);
        assert_eq!(servo.frequency, 50);
        assert_eq!(servo.min_pulse_width, 2.0);
        assert_eq!(servo.max_pulse_width, 12.0);
        assert_eq!(servo.servo_type, ServoType::Custom);

    }

    #[test]
    fn test_calc_pulse_width() {

    }

    #[test]
    fn test_set_angle_deg() {

    }

    #[test]
    fn test_set_angle_rad() {

    }
}





