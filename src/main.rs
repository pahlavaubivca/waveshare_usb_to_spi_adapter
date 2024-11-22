// #[link(name = "ch347")]
// extern "C" {
//     fn CH347OpenDevice() ->
//     fn add_numbers(a: i32, b: i32) -> i32;
// }

mod bindings {
    // Include the generated bindings
    // include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
    include!(concat!("../lib", "/bindings.rs"));
}

use std::ffi::{c_char, CString};
use std::os::raw::c_void;
use std::{result, thread};
use std::time::Duration;
use bindings::*;

pub struct SPIConfig {
    // ("iMode", c_ubyte),
    // ("iClock", c_ubyte),
    // ("iByteOrder", c_ubyte),
    // ("iSpiWriteReadInterval", c_ushort),
    // ("iSpiOutDefaultData",c_ubyte),
    // ("iChipSelect", c_ulong),
    // ("CS1Polarity",c_ubyte),
    // ("CS2Polarity", c_ubyte),
    // ("iIsAutoDeativeCS", c_ushort),
    // ("iActiveDelay", c_ushort),
    // ("iDelayDeactive", c_ulong),

    iMode: u8, // 0-3:SPI Mode0/1/2/3
    iClock: u8, //0=60MHz, 1=30MHz, 2=15MHz, 3=7.5MHz, 4=3.75MHz, 5=1.875MHz, 6=937.5KHz,7=468.75KHz
    iByteOrder: u8, // 0=(LSB), 1=(MSB)
    iSpiWriteReadInterval: u16,
    iSpiOutDefaultData: u8, //The SPI outputs data by default when it reads data
    iChipSelect: u64, //CS control, bit 7 is 0, the CS control is ignored, bit 7 is 1, the parameter is valid: bit 1, bit 0 is 00/01, respectively select CS1/CS2 pin as a low level active CS
    CS1Polarity: u8,
    CS2Polarity: u8,
    iIsAutoDeativeCS: u16,
    iActiveDelay: u16,
    iDelayDeactive: u64,
}

fn main()
{
    let device_path = b"/dev/ch34x_pis6"; // Replace with the actual device path
    // let utf8_bytes: &[u8] = device_path.as_bytes();
    println!("Device path: {:?}", device_path);
    // Convert the Rust string to a C-compatible string


    let c_device_path = CString::new(device_path).expect("CString::new failed");
    println!("C Device path: {:?}", c_device_path);
    unsafe {
        let fd = CH347OpenDevice(c_device_path.as_ptr() as *const c_char);
        if fd >= 0 {
            let mut CH347_SPI = mSpiCfgS {
                iMode: 1,
                iClock: 2,
                iByteOrder: 1,
                iSpiWriteReadInterval: 0,
                iSpiOutDefaultData: 0xff,
                iChipSelect: 0x00,
                CS1Polarity: 0,
                CS2Polarity: 0,
                iIsAutoDeativeCS: 0,
                iActiveDelay: 0,
                iDelayDeactive: 0,
            };
            if !CH347SPI_Init(fd, &mut CH347_SPI) {
                println!("Failed to init SPI");
                CH347SPI_Init(fd, &mut CH347_SPI);
                if !CH347SPI_Init(fd, &mut CH347_SPI)
                {
                    println!("Failed to init SPI");
                    panic!("Failed to init SPI");
                } else {
                    println!("SPI Init success");
                }
            } else {
                println!("SPI Init success");
            }
            println!("Device opened successfully, fd = {}", fd);
            loop {
                let mut data_to_send: [u8; 4] = [b'a', b'b', b'c', b'd'];
                let io_buffer = data_to_send.as_mut_ptr() as *mut c_void;
                let result = CH347SPI_Write(
                    fd,
                    true,
                    0x00,
                    4,
                    4,
                    io_buffer,
                );
                println!("Result: {}", result);
                thread::sleep(Duration::from_millis(1000));
            }
            let mut data_to_send: [u8; 1] = [1];
            let io_buffer = data_to_send.as_mut_ptr() as *mut c_void;
            let result = CH347SPI_WriteRead(fd, false, 0x80, 1, io_buffer);
            println!("Result: {}", result);
            // You can now use the file descriptor `fd` with other functions
        } else {
            println!("Failed to open device, error code = {}", fd);
            // let error_code = CH347GetLastError(); // Replace with actual error function if available
            // println!("Error code: {}", error_code);
            // 
            // // Map the error code to a human-readable message if possible
            // match error_code {
            //     // Replace with actual error codes and messages
            //     1 => println!("Error: Device not found"),
            //     2 => println!("Error: Access denied"),
            //     // Add other error codes as needed
            //     _ => println!("Error: Unknown error"),
            // }
        }
    }
    // let some: bindings::CH347 = unsafe { std::mem::zeroed() };
    println!("Hello, world!");
}
