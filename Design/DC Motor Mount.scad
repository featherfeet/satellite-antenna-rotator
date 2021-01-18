include <DC Motor.scad>;

MOTOR_MOUNT_WIDTH = 35;
MOTOR_MOUNT_LENGTH = MOTOR_GEARBOX_LENGTH + MOTOR_BODY_LENGTH;
TOP_EXTRUSION_LENGTH = 50 - 0.5;
BASE_MOUNTING_SCREW_HOLE_DIAMETER = 4.5;

module rightTriangle(base, height, thickness) {
    linear_extrude(height = thickness) {
        polygon(points = [[0, 0], [0, height], [base, 0]]);
    }
}

module DC_Motor_Mount() {
    // Baseplate.
    difference() {
        translate([-2.5, -(MOTOR_MOUNT_WIDTH + 15) / 2, MOTOR_GEARBOX_DIAMETER / 2]) {
            union() {
                cube([MOTOR_MOUNT_LENGTH + 2.5, MOTOR_MOUNT_WIDTH + 15, 3]);
                translate([7.5, -10, 0]) {
                    cube([10, 70, 3]);
                }
                translate([38, -10, 0]) {
                    cube([10, 70, 3]);
                }
            }
        }
        // Screw holes for attaching to base.
        translate([10, -30, 10]) {
            cylinder(h = 10, d = BASE_MOUNTING_SCREW_HOLE_DIAMETER);
        }
        translate([40, -30, 10]) {
            cylinder(h = 10, d = BASE_MOUNTING_SCREW_HOLE_DIAMETER);
        }
        translate([10, 30, 10]) {
            cylinder(h = 10, d = BASE_MOUNTING_SCREW_HOLE_DIAMETER);
        }
        translate([40, 30, 10]) {
            cylinder(h = 10, d = BASE_MOUNTING_SCREW_HOLE_DIAMETER);
        }
    }
    // Extrusions on top of the baseplate that lock the mount in place against the wooden base.
    translate([6, -TOP_EXTRUSION_LENGTH / 2, MOTOR_GEARBOX_DIAMETER / 2 + 3]) {
        cube([10, TOP_EXTRUSION_LENGTH, 3]);
    }
    translate([24, -20 / 2, MOTOR_GEARBOX_DIAMETER / 2 + 3]) {
        cube([10, 20, 3]);
    }
    // Faceplate.
    translate([-MOTOR_FLANGE_LENGTH, -MOTOR_MOUNT_WIDTH / 2, -9.5]) {
        difference() {
            cube([MOTOR_FLANGE_LENGTH, MOTOR_MOUNT_WIDTH, MOTOR_GEARBOX_DIAMETER]);
            translate([-1, MOTOR_MOUNT_WIDTH / 2, 9.3]) {
                rotate([0, 90, 0]) {
                    cylinder(d = MOTOR_FLANGE_DIAMETER + 1, h = 10);
                    translate([0, 8.5, 0]) {
                        cylinder(d = MOUNTING_SCREW_SIZE + 0.75, h = 10);
                    }
                    translate([0, -8.5, 0]) {
                        cylinder(d = MOUNTING_SCREW_SIZE + 0.75, h = 10);
                    }
                }
            }
        }
    }
    // Faceplate supports.
    rotate([90, 90, 0]) {
        translate([-15.5, 0, -17.5]) {
            rightTriangle(MOTOR_GEARBOX_DIAMETER, MOTOR_MOUNT_LENGTH, 3);
        }
        translate([-15.5, 0, 14.5]) {
            rightTriangle(MOTOR_GEARBOX_DIAMETER, MOTOR_MOUNT_LENGTH, 3);
        }
    }
}

DC_Motor_Mount();
translate([0, 0, -0.2]) {
    rotate([90, 0, 90]) {
        //DC_Motor();
    }
}