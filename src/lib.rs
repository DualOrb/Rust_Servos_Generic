use crate::servo::Servo;
mod servo;

fn main() {

}

//Tests
#[cfg(test)]
mod tests {
    use crate::servo::{Servo, ServoType};
    use rust_decimal::prelude::*;


    fn create_test_servo() -> Servo {
        let servo = Servo{
            pin: 18,
            min_angle: 0.0,
            max_angle: 180.0,
            frequency: 50,
            min_pulse_width: 2.0,
            max_pulse_width: 12.0,

            servo_type: ServoType::Custom,
        };
        return servo;
    }

    #[test]
    fn test_initialize() {
        let servo = create_test_servo();

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
        let servo = create_test_servo();

        let angle = 47 as f32;

        let calculated_value = Decimal::from_f64(servo.calculate_pulse_widths(&angle) as f64).unwrap().round_dp(2).to_f32().unwrap();

        assert_eq!(calculated_value, 4.61);
    }

    #[test]
    fn test_set_angle_deg() {
        //Min val

        //Middle val

        //Max Val

        //Error Bottom

        //Error top
    }

    #[test]
    fn test_set_angle_rad() {
        //Min val

        //Middle val

        //Max Val

        //Error Bottom

        //Error top
    }
}





