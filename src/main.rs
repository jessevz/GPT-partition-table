use simple_bytes::{Bytes, BytesArray, BytesRead, BytesWrite};
use std::{fs::File, io::Write};
// use crc::{Crc, Algorithm} 

static FILE_NAME: &str = "GPT.bin";
static LBA_SIZE: u32 = 512;
static ESP_SIZE: u32 = 1024*1024*33; //33MiB
static DATA_SIZE: u32 = 1024*1024*33; //33MiB

pub struct GPT {
    protective_MBR: ProtectiveMBR,
    gpt_header: GPT_header,
}

impl GPT {
    fn new(image_size_lbas: u32) -> Self {
        Self { 
            protective_MBR: ProtectiveMBR::new(image_size_lbas),
            gpt_header: GPT_header::new(image_size_lbas)  
        }
    }

    fn to_bytes(&self) -> [u8; 1024] {
        let mut bytes = BytesArray::from([0u8; 1024]);
        BytesWrite::write(&mut bytes, self.protective_MBR.to_bytes());
        BytesWrite::write(&mut bytes, self.gpt_header.to_bytes());

        return bytes.into_array();
    }

    fn write_to_file(&self) {
        let mut f = File::create(FILE_NAME).expect("Cant create file!");
        let _ = f.write_all(&self.to_bytes());
    }
}

pub struct ProtectiveMBR {
    bootcode: [u8; 440],
    disk_signature: [u8; 4],
    unknown: [u8; 2],
    partitions: [Partition_record; 4],
    signature: [u8; 2],
}

// impl Default for ProtectiveMBR {
//     fn default() -> Self {
//         //see table 5.3 in UEFI spec
//         Self{
//             bootcode: [0x00; 440],
//             disk_signature: [0x00; 4],
//             unknown: [0x00, 2],
//             partitions: [
//                 Partition_record::new_protective_partition_record(),
//                 Partition_record::zero_partition_record(),
//                 Partition_record::zero_partition_record(),
//                 Partition_record::zero_partition_record(),
//                 ],
//             signature: [0x55, 0xAA]
//         }
//     }
// }

impl ProtectiveMBR {
    fn new(image_size_lbas: u32) -> Self {
        Self{
            bootcode: [0x00; 440],
            disk_signature: [0x00; 4],
            unknown: [0x00, 2],
            partitions: [
                Partition_record::new_protective_partition_record(image_size_lbas),
                Partition_record::zero_partition_record(),
                Partition_record::zero_partition_record(),
                Partition_record::zero_partition_record(),
                ],
            signature: [0x55, 0xAA]
        }

    }

    fn to_bytes(&self) -> [u8; 512] {
        let mut bytes = BytesArray::from([0u8; 512]);

        BytesWrite::write(&mut bytes, self.bootcode);
        BytesWrite::write(&mut bytes, self.disk_signature);
        BytesWrite::write(&mut bytes, self.unknown);
        for partition in self.partitions {
            BytesWrite::write(&mut bytes, partition.to_bytes())
        }
        BytesWrite::write(&mut bytes, self.signature);

        return bytes.into_array();
    }

    fn write_to_file(&self) {
        let mut f = File::create("protectiveMBR.bin").expect("Cant create file!");
        let _ = f.write_all(&self.to_bytes());
    }
    
}

pub struct GPT_header {
    //table 5.5 in UEFI spec
    signature: [u8; 8], //offset 0
    revision: [u8; 4], //offset 8
    headerSize: u32, //offset 12
    headerCRC32: u32, //offset 16
    reserved: u32, //offset 20
    myLBA: u64, //offset 24
    alternateLBA: u64, //offset 32
    first_usable_LBA: u64, //offset 40
    last_usable_LBA: u64, //offset 48
    diskGUID: [u8; 16], //offset 56
    partitionEntryLBA: u64,  //offset 72
    number_of_partition_entries: u32,  //offset 80
    size_of_partition_entry: u32, //offset 84
    partition_entry_array_CRC32: u32, //offset 88

}

impl GPT_header {
    fn calculate_crc(&self) {

    }

    fn check_is_valid(&self) {

    }

    fn new(size: u32) -> Self {
        Self { 
            signature: [0x45, 0x46, 0x49, 0x20, 0x50, 0x41, 0x52, 0x54], //ASCII string EFI PART
            revision: [0x00, 0x00, 0x01, 0x00], //version 1.0
            headerSize: 92, 
            headerCRC32: 0, //TODO
            reserved: 0, 
            myLBA: 1, //hard coded 
            alternateLBA: size as u64,  
            first_usable_LBA: 0, //TODO
            last_usable_LBA: 0,  //TODO
            diskGUID: [0xe5, 0x85, 0xe5, 0x17, 0x04, 0x0d, 0x24, 0x42, 0x97, 0x21, 0x3b, 0x49, 0x09, 0xbf, 0x33, 0x56], //hardcoded
            partitionEntryLBA: 2, 
            number_of_partition_entries: 0, //TODO 
            size_of_partition_entry: 128, 
            partition_entry_array_CRC32: 0 //TODO
        }
    }

    fn write_to_file(&self) {
        let mut f = File::create("GPT_header.bin").expect("Cant create file!");
        let _ = f.write_all(&self.to_bytes());
    }

    fn to_bytes(&self) -> [u8; 512] {
        let mut bytes = BytesArray::from([0u8; 512]);
        BytesWrite::write(&mut bytes, self.signature);
        BytesWrite::write(&mut bytes, self.revision);
        BytesWrite::write_le_u32(&mut bytes, self.headerSize);
        BytesWrite::write_le_u32(&mut bytes, self.headerCRC32);
        BytesWrite::write_le_u32(&mut bytes, self.reserved);
        BytesWrite::write_le_u64(&mut bytes, self.myLBA);
        BytesWrite::write_le_u64(&mut bytes, self.alternateLBA);
        BytesWrite::write_le_u64(&mut bytes, self.first_usable_LBA);
        BytesWrite::write_le_u64(&mut bytes, self.last_usable_LBA);
        BytesWrite::write(&mut bytes, self.diskGUID);
        BytesWrite::write_le_u64(&mut bytes, self.partitionEntryLBA);
        BytesWrite::write_le_u32(&mut bytes, self.number_of_partition_entries);
        BytesWrite::write_le_u32(&mut bytes, self.size_of_partition_entry);
        BytesWrite::write_le_u32(&mut bytes, self.partition_entry_array_CRC32);

        return  bytes.into_array();
    }
}

pub struct partition_entry {
    //table 5.3.3 in UEFI spec
    partition_type_GUID: [u8; 16],
    unique_partition_GUID: [u8; 16],
    starting_lba: u64,
    ending_lba: u64,
    attributes: [u8; 8],
    partition_name: [u8; 72],
}

// impl partition_entry {
//     fn new() -> Self{
//         Self { partition_type_GUID: (), unique_partition_GUID: (), starting_lba: (), ending_lba: (), attributes: (), partition_name: () }
//     }
// }

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Partition_record {
    //See table 5.4 in UEFI spec
    bootindicator: u8,
    startingCHS: [u8; 3],
    OStype: u8,
    endingCHS: [u8; 3],
    startingLBA: [u8; 4],
    size_in_LBA: u32,
}

impl Partition_record {
    fn new_protective_partition_record(size: u32) -> Self {
        //todo set disksize
        Self {
            bootindicator: 0x00,
            startingCHS: [0x00, 0x02, 0x00],
            OStype: 0xEE,
            endingCHS: [0xff, 0xff, 0xff],
            startingLBA: [0x01, 0x00, 0x00, 0x00],
            size_in_LBA: size - 1  
        }
    }

    fn zero_partition_record() -> Self {
        Self {
            bootindicator: 0x00,
            startingCHS: [0x00, 0x00, 0x00],
            OStype: 0x00,
            endingCHS: [0x00, 0x00, 0x00],
            startingLBA: [0x00, 0x00, 0x00, 0x00],
            size_in_LBA: 0
        }
    }

    fn to_bytes(&self) -> [u8; 16] {
        let mut bytes = BytesArray::from([0u8; 16]);

        bytes.write_u8(self.bootindicator);
        BytesWrite::write(&mut bytes, self.startingCHS);
        bytes.write_u8(self.OStype);
        BytesWrite::write(&mut bytes, self.endingCHS);
        BytesWrite::write(&mut bytes, self.startingLBA);
        BytesWrite::write_le_u32(&mut bytes, self.size_in_LBA);

        return bytes.into_array();

    }
}

//convert bytes to LBA's
fn bytes_to_lbas(bytes: u32) -> u32{
    let leftover = if bytes % 512 == 0 {0} else {1};
    return bytes / LBA_SIZE + leftover;
}

fn main() {
    let image_size = ESP_SIZE + DATA_SIZE + (1024*1024); //add MiB of padding
    let image_size_lbas = bytes_to_lbas(image_size);

    // let protectiveMBR = ProtectiveMBR::new(image_size_lbas);
    // protectiveMBR.write_to_file();

    // let gpt_header = GPT_header::new(image_size_lbas);
    // gpt_header.write_to_file();

    let gpt = GPT::new(image_size_lbas);
    gpt.write_to_file();

}
