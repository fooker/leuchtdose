
s = 6;
d = 12 * s / PI;
h = 66;

translate([0, 0, 2])
difference() {
    cylinder(h=h, d=d, $fa=360/s);
    
    translate([0, 0, -1])
    cylinder(h=h+2, d=d-4, $fa=360/s);
}

for (i = [0:s]) {
    rotate([0, 0, i * 360 / s])
    translate([d/2-0.5, 0, 0])
    cylinder(h=h+4, d=2, $fn=32);
}

difference() {
    union() {
        for (i = [0:s]) {
            rotate([0, 0, i * 360 / s])
            translate([0, 0, 2])
            cube([d-1, 2, 4], center=true);
        }
        cylinder(h=4, d=8, $fn=32);
    }
    translate([0, 0, -1])
    cylinder(h=6, d=4, $fn=32);
}
