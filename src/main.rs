mod vox_writer;

use simple_anvil::chunk::Chunk;
pub(crate) use simple_anvil::{region::Region};
pub(crate) use vox_writer::{vox_writer::VoxWriter};

use std::{collections::HashMap, time::SystemTime};

#[allow(non_snake_case)]
fn main() {
    //let mut printHelper = Vec::new();
    let blocksToSkip = Vec::from([
        "minecraft:grass".to_string(),
        "minecraft:tall_grass".to_string(),
        "minecraft:poppy".to_string(),
        "minecraft:dandelion".to_string(),
        "minecraft:oxeye_daisy".to_string(),
        "minecraft:powered_rail".to_string(),
        "minecraft:rail".to_string(),
        "minecraft:cornflower".to_string(),
        "minecraft:torch".to_string(),
        "minecraft:wall_torch".to_string(),
        "minecraft:yellow_carpet".to_string(),
        "minecraft:gray_carpet".to_string(),
        "minecraft:light_gray_carpet".to_string(),
        "minecraft:red_gray_carpet".to_string(),
    ]);

    let colorDict = HashMap::from([
        ("minecraft:stone".to_string(), 77),
        ("minecraft:cobblestone".to_string(), 87),
        ("minecraft:dirt".to_string(), 33),
        ("minecraft:grass_block".to_string(), 9),
        ("minecraft:oak_leaves".to_string(), 11),
        ("minecraft:coal_ore".to_string(), 52),
        ("minecraft:iron_ore".to_string(), 56),
        ("minecraft:water".to_string(), 5),
        ("minecraft:redstone_ore".to_string(), 8),
        ("minecraft:stone_bricks".to_string(), 81),
        ("minecraft:stone_brick_wall".to_string(), 82),
        ("minecraft:tuff".to_string(), 129),
        ("minecraft:deepslate".to_string(), 81),
        ("minecraft:glass".to_string(), 1),
    ]);

	let totalTime = SystemTime::now();
    let size = 1;

    
    let region: Region = Region::from_file(String::from("region/r.0.0.mca"));

    for voxX in 0..4 {
        for voxY in 0..4 {
            let mut vox = VoxWriter::create(128,128,48);
            setupColor(&mut vox);

            let voxXNum = voxX * 8;
            let voxYNum = voxY * 8;
            for chunkX in voxXNum..voxXNum+8 {
                for chunkY in voxYNum..voxYNum+8 {

                    let chunkTimer = SystemTime::now();

                    let chunkWrapper = region.get_chunk(31 - chunkX as u32, chunkY as u32);
                    if chunkWrapper.is_none() { continue }
                    let chunk: Chunk = chunkWrapper.unwrap();
                    let chunkXOffset: i32 = chunkX * 16 * size;
                    let chunkYOffset: i32 = chunkY * 16 * size;

                    println!("new chunk {}:{}", chunkX, chunkY);

                    for x in 0..16 {
                        for z in 0..16 {
                            for y in 50..128 {
                                let block = chunk.get_block(15 - x, y, z);
                                let blockname = block.name();

                                if blocksToSkip.contains(&blockname) { continue }

                                //if blockname == "minecraft:air" { continue };
                
                                //println!("{}" , blockname);
                                if blockname == "minecraft:air" { continue }

                                let cube_color: i32;

                                match colorDict.get(&blockname) {
                                    Some(review) => {
                                        cube_color = *review;
                                    }
                                    None => {
                                        /*
                                        if !printHelper.contains(&blockname) {
                                            println!("{}", blockname);
                                            printHelper.push(blockname);
                                        }
                                        */

                                        cube_color = 96;
                                    }
                                }

                                //println!("color: {}", cube_color);
                                //println!("color: {}", blockname);
                                //println!("{} {} {}", x,y,z);
                                //println!("{}", cube_color);
                
                                let newX = x * size;
                                let newY = y * size;
                                let newZ = z * size;

                                for xx in 0..size {
                                    for yy in 0..size {
                                        for zz in 0..size {
                                            vox.add_voxel(newX + xx + chunkXOffset, newZ + zz + chunkYOffset,  newY + yy, cube_color);
                                        }
                                    }
                                }
                                
                            }
                        }
                    }
                    
                    println!("chunk done, took: {}ms", chunkTimer.elapsed().unwrap().as_millis());
                }
            }

            let name = format!("voxes/chunk-{}-{}.vox", voxX, voxY);
            vox.save_to_file(name.to_string()).unwrap();
        }
    }

    println!("everything done took: {}s", totalTime.elapsed().unwrap().as_secs());
}

#[allow(non_snake_case)]
fn setupColor(vox: &mut VoxWriter) {
    //vox.clear_colors();

    vox.add_color(126, 126, 126, 2, 0);
    vox.add_color(113, 89, 83, 2, 1);
    vox.add_color(113, 111, 71, 2, 2);
    vox.add_color(75, 126, 78, 2, 3);
    vox.add_color(68, 119, 115, 2, 4);
    vox.add_color(88, 91, 126, 2, 5);
    vox.add_color(101, 79, 122, 2, 6);
    vox.add_color(126, 64, 112, 2, 7);

    vox.add_color(105, 146, 80, 255, 8);
    vox.add_color(92, 127, 70, 255, 9);
    vox.add_color(80, 108, 60, 255, 10);
    vox.add_color(68, 92, 49, 255, 11);
    vox.add_color(56, 77, 37, 255, 12);
    vox.add_color(43, 62, 26, 255, 13);
    vox.add_color(31, 47, 15, 255, 14);
    vox.add_color(19, 32, 4, 255, 15);
    vox.add_color(99, 146, 103, 255, 16);
    vox.add_color(84, 127, 86, 255, 17);
    vox.add_color(70, 107, 69, 255, 18);
    vox.add_color(57, 93, 57, 255, 19);
    vox.add_color(44, 78, 45, 255, 20);
    vox.add_color(31, 64, 33, 255, 21);
    vox.add_color(18, 49, 21, 255, 22);
    vox.add_color(6, 35, 9, 255, 23);
    vox.add_color(102, 90, 84, 255, 24);
    vox.add_color(92, 80, 74, 255, 25);
    vox.add_color(82, 70, 63, 255, 26);
    vox.add_color(73, 61, 54, 255, 27);
    vox.add_color(63, 53, 47, 255, 28);
    vox.add_color(54, 45, 40, 255, 29);
    vox.add_color(44, 37, 32, 255, 30);
    vox.add_color(35, 29, 26, 255, 31);
    vox.add_color(131, 106, 89, 255, 32);
    vox.add_color(119, 96, 80, 255, 33);
    vox.add_color(107, 86, 72, 255, 34);
    vox.add_color(95, 76, 64, 255, 35);
    vox.add_color(83, 66, 55, 255, 36);
    vox.add_color(71, 56, 47, 255, 37);
    vox.add_color(59, 46, 39, 255, 38);
    vox.add_color(47, 37, 31, 255, 39);
    vox.add_color(125, 119, 101, 255, 40);
    vox.add_color(112, 106, 90, 255, 41);
    vox.add_color(99, 94, 79, 255, 42);
    vox.add_color(86, 81, 68, 255, 43);
    vox.add_color(73, 69, 57, 255, 44);
    vox.add_color(60, 56, 46, 255, 45);
    vox.add_color(47, 44, 35, 255, 46);
    vox.add_color(35, 32, 24, 255, 47);
    vox.add_color(125, 125, 125, 255, 48);
    vox.add_color(113, 113, 113, 255, 49);
    vox.add_color(100, 100, 100, 255, 50);
    vox.add_color(87, 87, 87, 255, 51);
    vox.add_color(75, 75, 75, 255, 52);
    vox.add_color(62, 62, 62, 255, 53);
    vox.add_color(49, 49, 49, 255, 54);
    vox.add_color(37, 37, 37, 255, 55);
    vox.add_color(186, 161, 130, 255, 56);
    vox.add_color(158, 134, 106, 255, 57);
    vox.add_color(131, 107, 82, 255, 58);
    vox.add_color(104, 81, 59, 255, 59);
    vox.add_color(88, 68, 48, 255, 60);
    vox.add_color(72, 55, 38, 255, 61);
    vox.add_color(56, 42, 27, 255, 62);
    vox.add_color(40, 29, 17, 255, 63);
    vox.add_color(186, 147, 94, 255, 64);
    vox.add_color(160, 122, 74, 255, 65);
    vox.add_color(134, 97, 55, 255, 66);
    vox.add_color(107, 72, 36, 255, 67);
    vox.add_color(90, 60, 29, 255, 68);
    vox.add_color(73, 49, 23, 255, 69);
    vox.add_color(57, 38, 16, 255, 70);
    vox.add_color(40, 27, 10, 255, 71);
    vox.add_color(172, 172, 172, 255, 72);
    vox.add_color(154, 154, 154, 255, 73);
    vox.add_color(136, 136, 136, 255, 74);
    vox.add_color(119, 119, 119, 255, 75);
    vox.add_color(101, 101, 101, 255, 76);
    vox.add_color(84, 84, 84, 255, 77);
    vox.add_color(66, 66, 66, 255, 78);
    vox.add_color(48, 48, 48, 255, 79);
    vox.add_color(172, 172, 172, 255, 80);
    vox.add_color(154, 154, 154, 255, 81);
    vox.add_color(136, 136, 136, 255, 82);
    vox.add_color(119, 119, 119, 255, 83);
    vox.add_color(101, 101, 101, 255, 84);
    vox.add_color(84, 84, 84, 255, 85);
    vox.add_color(66, 66, 66, 255, 86);
    vox.add_color(49, 49, 49, 255, 87);
    vox.add_color(179, 115, 78, 255, 88);
    vox.add_color(138, 88, 59, 255, 89);
    vox.add_color(98, 62, 41, 255, 90);
    vox.add_color(58, 36, 23, 255, 91);
    vox.add_color(16, 8, 3, 255, 92);
    vox.add_color(54, 48, 45, 255, 93);
    vox.add_color(91, 89, 87, 255, 94);
    vox.add_color(129, 129, 129, 255, 95);
    vox.add_color(179, 84, 32, 255, 96);
    vox.add_color(155, 73, 25, 255, 97);
    vox.add_color(132, 63, 19, 255, 98);
    vox.add_color(109, 53, 12, 255, 99);
    vox.add_color(96, 49, 15, 255, 100);
    vox.add_color(83, 46, 17, 255, 101);
    vox.add_color(70, 42, 18, 255, 102);
    vox.add_color(57, 39, 20, 255, 103);
    vox.add_color(186, 157, 153, 255, 104);
    vox.add_color(171, 144, 140, 255, 105);
    vox.add_color(156, 132, 128, 255, 106);
    vox.add_color(141, 119, 116, 255, 107);
    vox.add_color(127, 107, 104, 255, 108);
    vox.add_color(112, 94, 92, 255, 109);
    vox.add_color(97, 82, 80, 255, 110);
    vox.add_color(83, 70, 68, 255, 111);
    vox.add_color(179, 164, 116, 255, 112);
    vox.add_color(165, 151, 107, 255, 113);
    vox.add_color(151, 138, 98, 255, 114);
    vox.add_color(137, 125, 89, 255, 115);
    vox.add_color(124, 112, 80, 255, 116);
    vox.add_color(110, 99, 71, 255, 117);
    vox.add_color(96, 86, 62, 255, 118);
    vox.add_color(83, 73, 53, 255, 119);
    vox.add_color(172, 172, 172, 255, 120);
    vox.add_color(154, 154, 154, 255, 121);
    vox.add_color(136, 136, 136, 255, 122);
    vox.add_color(118, 118, 118, 255, 123);
    vox.add_color(100, 100, 100, 255, 124);
    vox.add_color(82, 82, 82, 255, 125);
    vox.add_color(64, 64, 64, 255, 126);
    vox.add_color(47, 47, 47, 255, 127);
    vox.add_color(172, 172, 172, 255, 128);
    vox.add_color(154, 154, 154, 255, 129);
    vox.add_color(136, 136, 136, 255, 130);
    vox.add_color(118, 118, 118, 255, 131);
    vox.add_color(100, 100, 100, 255, 132);
    vox.add_color(82, 82, 82, 255, 133);
    vox.add_color(64, 64, 64, 255, 134);
    vox.add_color(47, 47, 47, 255, 135);
    vox.add_color(172, 172, 172, 255, 136);
    vox.add_color(154, 154, 154, 255, 137);
    vox.add_color(136, 136, 136, 255, 138);
    vox.add_color(118, 118, 118, 255, 139);
    vox.add_color(100, 100, 100, 255, 140);
    vox.add_color(82, 82, 82, 255, 141);
    vox.add_color(64, 64, 64, 255, 142);
    vox.add_color(46, 46, 46, 255, 143);
    vox.add_color(172, 172, 172, 255, 144);
    vox.add_color(154, 154, 154, 255, 145);
    vox.add_color(136, 136, 136, 255, 146);
    vox.add_color(118, 118, 118, 255, 147);
    vox.add_color(100, 100, 100, 255, 148);
    vox.add_color(82, 82, 82, 255, 149);
    vox.add_color(64, 64, 64, 255, 150);
    vox.add_color(46, 46, 46, 255, 151);
    vox.add_color(98, 137, 255, 255, 152);
    vox.add_color(87, 121, 228, 255, 153);
    vox.add_color(77, 106, 202, 255, 154);
    vox.add_color(67, 91, 176, 255, 155);
    vox.add_color(57, 75, 149, 255, 156);
    vox.add_color(47, 60, 123, 255, 157);
    vox.add_color(37, 45, 97, 255, 158);
    vox.add_color(27, 30, 71, 255, 159);
    vox.add_color(255, 99, 107, 255, 160);
    vox.add_color(228, 88, 95, 255, 161);
    vox.add_color(202, 78, 84, 255, 162);
    vox.add_color(176, 68, 73, 255, 163);
    vox.add_color(149, 57, 62, 255, 164);
    vox.add_color(123, 47, 51, 255, 165);
    vox.add_color(97, 37, 40, 255, 166);
    vox.add_color(71, 27, 29, 255, 167);
    vox.add_color(0, 0, 0, 255, 168);
    vox.add_color(0, 0, 0, 255, 169);
    vox.add_color(0, 0, 0, 255, 170);
    vox.add_color(0, 0, 0, 255, 171);
    vox.add_color(0, 0, 0, 255, 172);
    vox.add_color(0, 0, 0, 255, 173);
    vox.add_color(0, 0, 0, 255, 174);
    vox.add_color(0, 0, 0, 255, 175);
    vox.add_color(0, 0, 0, 255, 176);
    vox.add_color(0, 0, 0, 255, 177);
    vox.add_color(0, 0, 0, 255, 178);
    vox.add_color(0, 0, 0, 255, 179);
    vox.add_color(0, 0, 0, 255, 180);
    vox.add_color(0, 0, 0, 255, 181);
    vox.add_color(0, 0, 0, 255, 182);
    vox.add_color(0, 0, 0, 255, 183);
    vox.add_color(0, 0, 0, 255, 184);
    vox.add_color(0, 0, 0, 255, 185);
    vox.add_color(0, 0, 0, 255, 186);
    vox.add_color(0, 0, 0, 255, 187);
    vox.add_color(0, 0, 0, 255, 188);
    vox.add_color(0, 0, 0, 255, 189);
    vox.add_color(0, 0, 0, 255, 190);
    vox.add_color(0, 0, 0, 255, 191);
    vox.add_color(0, 0, 0, 255, 192);
    vox.add_color(0, 0, 0, 255, 193);
    vox.add_color(0, 0, 0, 255, 194);
    vox.add_color(0, 0, 0, 255, 195);
    vox.add_color(0, 0, 0, 255, 196);
    vox.add_color(0, 0, 0, 255, 197);
    vox.add_color(0, 0, 0, 255, 198);
    vox.add_color(0, 0, 0, 255, 199);
    vox.add_color(0, 0, 0, 255, 200);
    vox.add_color(0, 0, 0, 255, 201);
    vox.add_color(0, 0, 0, 255, 202);
    vox.add_color(0, 0, 0, 255, 203);
    vox.add_color(0, 0, 0, 255, 204);
    vox.add_color(0, 0, 0, 255, 205);
    vox.add_color(0, 0, 0, 255, 206);
    vox.add_color(0, 0, 0, 255, 207);
    vox.add_color(0, 0, 0, 255, 208);
    vox.add_color(0, 0, 0, 255, 209);
    vox.add_color(0, 0, 0, 255, 210);
    vox.add_color(0, 0, 0, 255, 211);
    vox.add_color(0, 0, 0, 255, 212);
    vox.add_color(0, 0, 0, 255, 213);
    vox.add_color(0, 0, 0, 255, 214);
    vox.add_color(0, 0, 0, 255, 215);
    vox.add_color(0, 0, 0, 255, 216);
    vox.add_color(0, 0, 0, 255, 217);
    vox.add_color(0, 0, 0, 255, 218);
    vox.add_color(0, 0, 0, 255, 219);
    vox.add_color(0, 0, 0, 255, 220);
    vox.add_color(0, 0, 0, 255, 221);
    vox.add_color(0, 0, 0, 255, 222);
    vox.add_color(0, 0, 0, 255, 223);
    vox.add_color(0, 0, 0, 255, 224);
    vox.add_color(0, 0, 0, 255, 225);
    vox.add_color(0, 0, 0, 255, 226);
    vox.add_color(0, 0, 0, 255, 227);
    vox.add_color(0, 0, 0, 255, 228);
    vox.add_color(0, 0, 0, 255, 229);
    vox.add_color(0, 0, 0, 255, 230);
    vox.add_color(0, 0, 0, 255, 231);
    vox.add_color(0, 0, 0, 255, 232);
    vox.add_color(0, 0, 0, 255, 233);
    vox.add_color(0, 0, 0, 255, 234);
    vox.add_color(0, 0, 0, 255, 235);
    vox.add_color(0, 0, 0, 255, 236);
    vox.add_color(0, 0, 0, 255, 237);
    vox.add_color(0, 0, 0, 255, 238);
    vox.add_color(0, 0, 0, 255, 239);
    vox.add_color(0, 0, 0, 255, 240);
    vox.add_color(0, 0, 0, 255, 241);
    vox.add_color(0, 0, 0, 255, 242);
    vox.add_color(0, 0, 0, 255, 243);
    vox.add_color(0, 0, 0, 255, 244);
    vox.add_color(0, 0, 0, 255, 245);
    vox.add_color(0, 0, 0, 255, 246);
    vox.add_color(0, 0, 0, 255, 247);
    vox.add_color(0, 0, 0, 255, 248);
    vox.add_color(0, 0, 0, 255, 249);
    vox.add_color(0, 0, 0, 255, 250);
    vox.add_color(0, 0, 0, 255, 251);
    vox.add_color(0, 0, 0, 255, 252);
    vox.add_color(0, 0, 0, 255, 253);
    vox.add_color(255, 0, 0, 255, 254);
    vox.add_color(0, 0, 0, 0, 255);
}
