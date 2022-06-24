$fn = 300;

MOTOR_BODY_LENGTH = 30.8;
MOTOR_BODY_DIAMETER = 25;
MOUNTING_SCREW_SIZE = 3;
MOTOR_GEARBOX_LENGTH = 23;
MOTOR_GEARBOX_DIAMETER = MOTOR_BODY_DIAMETER;
MOTOR_FLANGE_LENGTH = 2.5;
MOTOR_FLANGE_DIAMETER = 7;
MOTOR_SHAFT_LENGTH = 10;
MOTOR_SHAFT_DIAMETER = 4;

module DC_Motor() {
    // Gearbox.
    color([0.8, 0.8, 0.8, 1.0]) {
        difference() {
            // Gearbox casing.
            cylinder(h = MOTOR_GEARBOX_LENGTH, d = MOTOR_GEARBOX_DIAMETER);
            // Mounting screw holes.
            /*
            translate([8.5, 0, -20]) {
                #cylinder(h = 50, d = MOUNTING_SCREW_SIZE);
            }
            translate([-8.5, 0, -20]) {
                #cylinder(h = 50, d = MOUNTING_SCREW_SIZE);
            }
            */
        }
    }
    // Body.
    color([0.5, 0.5, 0.5, 1.0]) {
        translate([0, 0, 23]) {
            cylinder(h = MOTOR_BODY_LENGTH, d = MOTOR_BODY_DIAMETER);
        }
    }
    // Flange (golden thing around shaft).
    color([1.0, 0.87, 0.0, 1.0]) {
         translate([0, 0, -MOTOR_FLANGE_LENGTH]) {
             cylinder(h = MOTOR_FLANGE_LENGTH, d = MOTOR_FLANGE_DIAMETER);
         }
    }
    // Shaft.
    color([0.5, 0.5, 0.5, 1.0]) {
        translate([0, 0, -MOTOR_FLANGE_LENGTH - MOTOR_SHAFT_LENGTH]) {
            difference() {
                cylinder(h = MOTOR_SHAFT_LENGTH, d = MOTOR_SHAFT_DIAMETER);
                translate([1.5, -5, -0.5]) {
                    cube([0.5, 10, MOTOR_SHAFT_LENGTH + 1]);
                }
            }
        }
    }
}

module DC_Motor_Mounting_Screw_Holes() {
    translate([8.5, 0, -20]) {
        cylinder(h = 50, d = MOUNTING_SCREW_SIZE + 0.25);
    }
    translate([-8.5, 0, -20]) {
        cylinder(h = 50, d = MOUNTING_SCREW_SIZE + 0.25);
    }
    translate([0, 0, -20]) {
        cylinder(h = 50, d = MOTOR_FLANGE_DIAMETER + 0.25);
    }
}

module DC_Motor_Mounting_Screw_Holes_2D() {
    translate([8.5, 0, -20]) {
        circle(d = MOUNTING_SCREW_SIZE + 0.25);
    }
    translate([-8.5, 0, -20]) {
        circle(d = MOUNTING_SCREW_SIZE + 0.25);
    }
    translate([0, 0, -20]) {
        circle(d = MOTOR_FLANGE_DIAMETER + 0.25);
    }
}

//DC_Motor();
