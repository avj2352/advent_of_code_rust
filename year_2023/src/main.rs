mod day_01;
mod day_02;
mod day_03;

use day_01::trebuchet;
use day_02::cube_conundrum;
use day_03::gondola_missing_parts;
fn main() {
    // ..day_01
    trebuchet::part_01();
    trebuchet::part_02();
    // ..day_02
    cube_conundrum::part_01();
    cube_conundrum::part_02();
    // ..day-03
    gondola_missing_parts::part_01();
    gondola_missing_parts::part_02();
}
