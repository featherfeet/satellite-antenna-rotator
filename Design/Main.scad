// units: mm

$fn = 200;

// Diameter of the metal axles used for the main altitude and azimuth gears.
AXLES_DIAMETER = 10.5; // mm

// Diameter of the holes used to seat the radial bearings for the gears.
RADIAL_BEARINGS_DIAMETER = 29; // mm

// Number of teeth on the main (larger) azimuth gear.
MAIN_AZIMUTH_GEAR_TEETH = 32; // teeth

// Number of teeth on the driving (smaller) azimuth gear.
DRIVING_AZIMUTH_GEAR_TEETH = 7; // teeth

// Angle to rotate the driving azimuth gear by to make it "mesh" with the main azimuth gear.
DRIVING_AZIMUTH_GEAR_ANGLE = 13; // degrees

// Angle to rotate the driving altitude gear by to make it "mesh" with the main altitude gear.
DRIVING_ALTITUDE_GEAR_ANGLE = 13; // degrees

// Thickness of the azimuth gears.
AZIMUTH_GEARS_THICKNESS = 10; // mm

// Tooth size of the azimuth gears.
AZIMUTH_GEARS_MM_PER_TOOTH = 18.45; // mm/tooth

// Number of teeth on the main (larger) altitude gear.
MAIN_ALTITUDE_GEAR_TEETH = 32; // teeth

// Thickness of the altitude gears.
ALTITUDE_GEARS_THICKNESS = 10; // mm

// Tooth size of the altitude gears.
ALTITUDE_GEARS_MM_PER_TOOTH = 18.45; // mm/tooth

// Number of teeth on the driving (smaller) altitude gear.
DRIVING_ALTITUDE_GEAR_TEETH = 7; // teeth

// Height (measured from the bottom face of the azimuth gear) to the axle of the altitude gear.
ALTITUDE_GEAR_AXLE_HEIGHT = 100 + 15 + 50 + 20;

use <pd-gears.scad>
use <DC Motor.scad>
use <triangles.scad>

module Hex_Coupler() {
    color([1, 0, 0])
        difference() {
            union() {
                cylinder(d = 13.868, h = 5, $fn = 6);
                translate([0, 0, 5])
                    cylinder(d = 11.41, h = 18 - 5);
            }
            cylinder(d = 4, h = 40, center = true);
        }
}

// A hole appropriately shaped and sized to hold a partially-recessed M3 nut and bolt, with a washer under the head of the nut.
module Hole_For_M3_Nut_And_Bolt() {
    NUT_HEIGHT = 2;
    BOLT_DIAMETER = 4;
    NUT_DIAMETER = 6.282 + 0.3;
    HEAD_WITH_WASHER_THICKNESS = 2;
    WASHER_DIAMETER = 10;
    translate([-NUT_HEIGHT - (ALTITUDE_GEARS_THICKNESS - NUT_HEIGHT - HEAD_WITH_WASHER_THICKNESS) / 2, 0]) {
        rotate([0, 90]) {
            translate([0, 0, -0.1])
                cylinder(d = NUT_DIAMETER, h = NUT_HEIGHT + 0.1, $fn = 6);
            cylinder(d = BOLT_DIAMETER, h = ALTITUDE_GEARS_THICKNESS);
            translate([0, 0, ALTITUDE_GEARS_THICKNESS - HEAD_WITH_WASHER_THICKNESS])
                cylinder(d = WASHER_DIAMETER, h = HEAD_WITH_WASHER_THICKNESS + 0.1);
        }
    }
}

// Main azimuth gear.
module Main_Azimuth_Gear() {
    color([0.67, 0.84, 0.9])
        difference() {
            gear(thickness = AZIMUTH_GEARS_THICKNESS, number_of_teeth = MAIN_AZIMUTH_GEAR_TEETH, mm_per_tooth = AZIMUTH_GEARS_MM_PER_TOOTH, hole_diameter = 25);
            for (angle = [0 : 360 / 3 : 360]) {
                rotate([0, 0, angle])
                    translate([35 / 2, 0, 5])
                        cylinder(d = 3.8, h = 20, center = true);
            }
        }
}

// Driving azimuth gear.
module Driving_Azimuth_Gear() {
    color([0, 1, 0])
        translate([0, 0, 10])
            rotate([180, 0, -DRIVING_AZIMUTH_GEAR_ANGLE]) {
                difference() {
                    gear(thickness = AZIMUTH_GEARS_THICKNESS, number_of_teeth = DRIVING_AZIMUTH_GEAR_TEETH, mm_per_tooth = AZIMUTH_GEARS_MM_PER_TOOTH, hole_diameter = 0);
                    translate([0, 0, -12])
                        cylinder(d = 15, h = 20);
                    cylinder(d = 4.5, h = 20);
                }
                translate([0, 0, AZIMUTH_GEARS_THICKNESS/2 + 5]) {
                    difference() {
                        cylinder(d = 18.5, h = 7.2);
                        cylinder(d = 14.6, h = 20, $fn = 6, center = true);
                    }
                }
            }
}

// Supports for the main altitude gear.
module Altitude_Gear_Support() {
    color([0.67, 0.84, 0.9])
        difference() {
            union() {
                // Vertical supports for altitude gear's axle.
                translate([35, 0, ALTITUDE_GEAR_AXLE_HEIGHT / 2]) {
                    cube([5, 50, ALTITUDE_GEAR_AXLE_HEIGHT], true);
                }
                // Round top parts of vertical supports for altitude gear's axle.
                translate([35, 0, ALTITUDE_GEAR_AXLE_HEIGHT]) {
                    rotate([0, 90]) {
                        cylinder(d = 50, h = 5, center = true);
                    }
                }
                // Support ribs for vertical supports.
                translate([37.5, 25]) {
                    rotate([90, 0, 0]) {
                        triangle(ALTITUDE_GEAR_AXLE_HEIGHT, 40, 5);
                    }
                }
                translate([37.5, -20]) {
                    rotate([90, 0, 0]) {
                        triangle(ALTITUDE_GEAR_AXLE_HEIGHT, 40, 5);
                    }
                }
            }
            // Hole for radial bearing.
            translate([-36, 0, ALTITUDE_GEAR_AXLE_HEIGHT]) {
                rotate([0, 90]) {
                        cylinder(d = RADIAL_BEARINGS_DIAMETER, h = 80);
                }
            }
        }
}

// Main altitude gear.
module Main_Altitude_Gear_Assembly() {
    altitude_gear_outer_radius = outer_radius(mm_per_tooth = ALTITUDE_GEARS_MM_PER_TOOTH, number_of_teeth = MAIN_ALTITUDE_GEAR_TEETH, clearance = 0.0);

    color([1, 1, 0]) {
        translate([-ALTITUDE_GEARS_THICKNESS/2 - 2.25, 0, ALTITUDE_GEAR_AXLE_HEIGHT]) {
            // Make the main altitude gear with a hole cut out of the top for the PVC pipe and holes for nuts and bolts.
            difference() {
                // Gear body.
                rotate([0, 90]) {
                    gear(thickness = ALTITUDE_GEARS_THICKNESS, number_of_teeth = MAIN_ALTITUDE_GEAR_TEETH, mm_per_tooth = ALTITUDE_GEARS_MM_PER_TOOTH, hole_diameter = AXLES_DIAMETER);
                }
                // Hole for the PVC pipe.
                translate([ALTITUDE_GEARS_THICKNESS/2, 0, altitude_gear_outer_radius - 45])
                    cylinder(d = 35, h = 50);
                // Holes for nuts and bolts to hold the gear together, plus triangular holes to make the gear use less plastic.
                for (angle = [90:360 / 6:360 + 90]) {
                    rotate([angle, 0, 0]) {
                        translate([ALTITUDE_GEARS_THICKNESS/2, 0, 70])
                            Hole_For_M3_Nut_And_Bolt();
                        translate([5, 0, 50])
                            rotate([0, 90, 0])
                                triangle(60, 30, 20, center = true);
                    }
                }
            }
            // Add cone-shaped pieces to keep the gear on the axle.
            difference() {
                union() {
                    translate([-25.2, 0])
                        rotate([0, 90])
                            cylinder(h = 25.2, d1 = 16.2, d2 = 35);
                    translate([10, 0])
                        rotate([0, 90])
                            cylinder(h = 29.7, d1 = 35, d2 = 16.2);
                }
                rotate([0, 90])
                    cylinder(h = 100, d = AXLES_DIAMETER, center = true);
            }
            // Add a collar to go around the PVC pipe, plus a half-sphere underneath it to brace it against the gear.
            translate([ALTITUDE_GEARS_THICKNESS/2, 0, altitude_gear_outer_radius - 45]) {
                // Collar for PVC pipe.
                difference() {
                    // Outside of collar.
                    cylinder(d = 35, h = 40);
                    // Hole in collar.
                    translate([0, 0, 4])
                        cylinder(d = 28, h = 45);
                }
                // Half-sphere to brace collar against gear.
                difference() {
                    sphere(d = 35);
                    translate([0, 0, 21])
                        cube([40, 40, 40], center = true);
                }
            }
        }
    }
}

// Driving altitude gear.
module Driving_Altitude_Gear() {
    color([0, 1, 0])
        translate([-ALTITUDE_GEARS_THICKNESS/2 - 2.25, 0])
            rotate([0, 90, 0])
                    rotate([0, 0, DRIVING_ALTITUDE_GEAR_ANGLE]) {
                        difference() {
                            gear(thickness = ALTITUDE_GEARS_THICKNESS, number_of_teeth = DRIVING_ALTITUDE_GEAR_TEETH, mm_per_tooth = ALTITUDE_GEARS_MM_PER_TOOTH, hole_diameter = 0);
                            translate([0, 0, -12])
                                cylinder(d = 15, h = 20);
                            cylinder(d = 4.5, h = 20);
                        }
                        translate([0, 0, ALTITUDE_GEARS_THICKNESS/2 + 5]) {
                            difference() {
                                cylinder(d = 18.5, h = 7.2);
                                cylinder(d = 14.6, h = 20, $fn = 6, center = true);
                            }
                        }
                    }
}

// Main azimuth gear and the supports for the altitude gear.
module Main_Azimuth_Gear_Assembly() {
    Main_Azimuth_Gear();

    difference() {
        Altitude_Gear_Support();
        translate([0, 0, ALTITUDE_GEAR_AXLE_HEIGHT - (pitch_radius_main_altitude_gear + pitch_radius_driving_altitude_gear + 1)])
            translate([26, 0])
                rotate([0, 90])
                    DC_Motor();
    }
    rotate([0, 0, 180])
        Altitude_Gear_Support();

    // Altitude motor mounting.
    color([0.67, 0.84, 0.9])
        translate([0, 0, ALTITUDE_GEAR_AXLE_HEIGHT - (pitch_radius_main_altitude_gear + pitch_radius_driving_altitude_gear + 1)])
            translate([28, 0])
                difference() {
                    cube([9, 50, 40], center = true);
                    translate([2, 0])
                        cube([9, 40, 30], center = true);
                    rotate([0, 90])
                        DC_Motor_Mounting_Screw_Holes();
                }
}

pitch_radius_main_azimuth_gear = pitch_radius(mm_per_tooth = AZIMUTH_GEARS_MM_PER_TOOTH, number_of_teeth = MAIN_AZIMUTH_GEAR_TEETH);
pitch_radius_driving_azimuth_gear = pitch_radius(mm_per_tooth = AZIMUTH_GEARS_MM_PER_TOOTH, number_of_teeth = DRIVING_AZIMUTH_GEAR_TEETH);
azimuth_driving_gear_offset = pitch_radius_main_azimuth_gear + pitch_radius_driving_azimuth_gear + 1;

// Driving azimuth gear.
module Driving_Azimuth_Gear_Assembly() {
    translate([azimuth_driving_gear_offset, 0])
        Driving_Azimuth_Gear();
}

// Driving azimuth motor.
module Driving_Azimuth_Motor_Assembly() {
    translate([azimuth_driving_gear_offset, 0, -21]) {
        rotate([180]) {
            DC_Motor();
            translate([0, 0, -21])
                rotate([0, 0, DRIVING_AZIMUTH_GEAR_ANGLE])
                    Hex_Coupler();
        }
    }
}

pitch_radius_main_altitude_gear = pitch_radius(mm_per_tooth = ALTITUDE_GEARS_MM_PER_TOOTH, number_of_teeth = MAIN_ALTITUDE_GEAR_TEETH);
pitch_radius_driving_altitude_gear = pitch_radius(mm_per_tooth = ALTITUDE_GEARS_MM_PER_TOOTH, number_of_teeth = DRIVING_ALTITUDE_GEAR_TEETH);
// Motor assembly for driving the altitude gear.
module Driving_Altitude_Motor_Assembly() {
    translate([0, 0, ALTITUDE_GEAR_AXLE_HEIGHT - (pitch_radius_main_altitude_gear + pitch_radius_driving_altitude_gear + 1)]) {
        // Altitude motor and hex coupler.
        translate([26, 0]) {
            rotate([0, 90]) {
                DC_Motor();
                translate([0, 0, -21])
                    rotate([0, 0, DRIVING_ALTITUDE_GEAR_ANGLE])
                        Hex_Coupler();
            }
        }
    }
}

module Driving_Altitude_Gear_Assembly() {
    translate([0, 0, ALTITUDE_GEAR_AXLE_HEIGHT - (pitch_radius_main_altitude_gear + pitch_radius_driving_altitude_gear + 1)])
        Driving_Altitude_Gear();
}

// Conical spacer that goes between the main azimuth gear and its bearing to keep the main azimuth gear at the right height.
/*
module Main_Azimuth_Gear_Spacer_Assembly() {
    spacer_height = 17.5;
    translate([0, 0, -spacer_height])
        difference() {
            cylinder(d1 = 16.2, d2 = 35, h = spacer_height);
            cylinder(d = AXLES_DIAMETER, h = 70, center = true);
        }
}
*/

// Baseplate.
module Baseplate_Assembly() {
    echo(str("azimuth_driving_gear_offset = ", azimuth_driving_gear_offset, " mm"));
    difference() {
        translate([0, 0, -21])
            cube([279.4, 279.4, 6.68], true);
        translate([azimuth_driving_gear_offset, 0, -22]) {
            rotate([180]) {
                DC_Motor_Mounting_Screw_Holes();
            }
        }
        translate([0, 0, -25])
            cylinder(d = RADIAL_BEARINGS_DIAMETER, h = 10);
    }
}

// Slip ring.
module Slip_Ring() {
    // SNM022A-12 Slip Ring from SenRing.
    translate([0, 0, -9]) {
        cylinder(d = 22, h = 26);
        translate([0, 0, 26])
            cylinder(d = 7.8, h = 9);
        difference() {
            translate([0, 0, 18.6])
                cylinder(d = 44.5, h = 2.4);
            for (angle = [0 : 360 / 3 : 360]) {
                rotate([0, 0, angle])
                    translate([35 / 2, 0, 15])
                        cylinder(d = 5.2, h = 10);
            }
        }
    }
}

// Main altitude gear assembly sliced into two pieces (with holes for bolts to attach them together) that can be printed flat.
module Altitude_Gear_Assembly_Printable_Piece_1() {
    difference() {
        translate([0, 0, -ALTITUDE_GEAR_AXLE_HEIGHT])
            Main_Altitude_Gear_Assembly();
        translate([50 / 2 - 2.25, 0])
            cube([50, 250, 300], center = true);
    }
}

module Altitude_Gear_Assembly_Printable_Piece_2() {
    difference() {
        translate([0, 0, -ALTITUDE_GEAR_AXLE_HEIGHT])
            Main_Altitude_Gear_Assembly();
        translate([-50 / 2 - 2.25, 0])
            cube([50, 250, 300], center = true);
    }
}

Main_Azimuth_Gear_Assembly();
/*
Driving_Azimuth_Gear_Assembly();
Driving_Azimuth_Motor_Assembly();

Main_Altitude_Gear_Assembly();
Driving_Altitude_Motor_Assembly();
Driving_Altitude_Gear_Assembly();

Baseplate_Assembly();

Slip_Ring();
*/

//Altitude_Gear_Assembly_Printable_Piece_1();
//Altitude_Gear_Assembly_Printable_Piece_2();
