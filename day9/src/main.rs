use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn disk_unmap(disk_map: Vec<char>) -> Vec<String> {
    const RADIX: u32 = 10;
    let mut disk_unmapped: Vec<String> = Vec::new();

    let mut i: usize = 0;
    let mut fileid: u32 = 0;

    while i < disk_map.len() {
        // i is file
        let filesize: u32 = disk_map[i].to_digit(RADIX).unwrap();
        for _j in 0..filesize {
            disk_unmapped.push(fileid.to_string());
        }
        fileid += 1;
        i += 1;
        if i >= disk_map.len() {
            break;
        }
        // i is freespace
        let freespace: u32 = disk_map[i].to_digit(RADIX).unwrap();
        for _j in 0..freespace {
            disk_unmapped.push(".".to_string());
        }
        i += 1;
    }
    return disk_unmapped;
}

fn compact_disk(disk_unmap: Vec<String>) -> Vec<String> {
    let mut compact_disk: Vec<String> = disk_unmap.clone();
    let mut i = 0;
    let mut j = compact_disk.len() - 1;
    while compact_disk[i] != ".".to_string() {
        i += 1;
    }
    while compact_disk[j] == ".".to_string() {
        j -= 1;
    }
    while i < j {
        let file = compact_disk[j].clone();
        compact_disk[j] = ".".to_string();
        compact_disk[i] = file;
        while compact_disk[i] != ".".to_string() {
            i += 1;
        }
        while compact_disk[j] == ".".to_string() {
            j -= 1;
        }
    }

    return compact_disk;
}

fn compact_disk_2(disk_unmap: Vec<String>) -> Vec<String> {
    let mut compact_disk: Vec<String> = disk_unmap.clone();
    let mut j = compact_disk.len() - 1;
    let mut stop_j = 0;
    let mut moving_block: Vec<String> = Vec::new();
    // find block by block, then for each block, try to find a free spot (only once !)
    while compact_disk[j] == ".".to_string() {
        j -= 1;
    }
    let mut file_id: String = compact_disk[j].clone();
    let mut k = j;
    while compact_disk[j] == file_id {
        j -= 1;
        moving_block.push(file_id.clone());
    }
    while j > 0 {
        // find a spot for moving_block
        let mut i = 0;
        while i < j {
            let mut len = 0;
            while compact_disk[i] != ".".to_string(){
                i += 1;
                if i >= compact_disk.len() {
                    break;
                }
            }
            while compact_disk[i + len] == ".".to_string() && (i + len) < compact_disk.len(){
                len += 1;
                if i + len >= compact_disk.len() {
                    break;
                }
            }
            if len >= moving_block.len()  && i < j {
                for x in 0..moving_block.len() {
                    compact_disk[x + i] = moving_block[x].clone();
                }
                for x in j..k {
                    compact_disk[x + 1] = ".".to_string();
                }
                break;
            }
            i += len;
        }
        while compact_disk[j] == ".".to_string()  && j > 0{
            j -= 1;
        }
        file_id = compact_disk[j].clone();
        k = j;
        moving_block = Vec::new();
        while compact_disk[j] == file_id  && j > 0 {
            j -= 1;
            moving_block.push(file_id.clone());
        }
    }
    return compact_disk;
}

fn checksum(compact_disk: Vec<String>) -> i64 {
    let mut checksum: i64 = 0;
    for i in 0..compact_disk.len() {
        if compact_disk[i] == ".".to_string() {
            continue;
        }
        checksum = checksum + (i as i64 * compact_disk[i].parse::<i64>().unwrap());
    }
    return checksum;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let mut disk_map: Vec<char> = Vec::new();
    let mut sum: i64 = 0;
    let mut sum2: i64 = 0;
    // parse using filename
    if let Ok(lines) = read_lines(filename) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines.flatten() {
            disk_map = line.chars().collect();
        }
    }
    let disk: Vec<String> = compact_disk(disk_unmap(disk_map.clone()));
    let disk_2: Vec<String> = compact_disk_2(disk_unmap(disk_map.clone()));

    sum = checksum(disk);
    sum2 = checksum(disk_2);
    println!("Sum is :{}", sum);
    println!("Sum2 is :{}", sum2);
}

// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn pause() {
    dbg!("Pausing! Press enter to continue...");

    let mut buffer = String::new();

    std::io::stdin()
        .read_line(&mut buffer)
        .expect("Failed to read line");
}
