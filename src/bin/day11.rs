use std::{cell::{RefCell, RefMut}, collections::HashMap, fs::File, io::Read, mem::swap, rc::{Rc, Weak}};

#[derive(Debug)]
struct Device {
    // Weak so that they do not cause memory leak
    connections: Vec<Weak<RefCell<Device>>>,
    ways_to_reach: usize
}

impl Device {
    fn new() -> Self {
        Device { connections: Vec::new(), ways_to_reach: 0 }
    }
}

fn not_dumb_solution(content: &String) -> Result<usize, Box<dyn std::error::Error>> {
    let mut devices: HashMap<&str, Rc<RefCell<Device>>> = HashMap::new();
    for line in content.lines() {
        let device_to_modify_label = line.split_once(':').ok_or("Each line must have a colon")?.0;
        let device_to_modify: Rc<RefCell<Device>>;
        match devices.get(&device_to_modify_label) {
            Some(x) => {
                device_to_modify = x.clone();
            }
            None => {
                let new_device = Device::new();
                let new_device_ref = Rc::new(RefCell::new(new_device));
                devices.insert(device_to_modify_label, new_device_ref.clone());
                device_to_modify = new_device_ref;
            }
        }
        for device_which_connects_label in line.split_once(':').unwrap().1.trim().split(' ') {
            let device_which_connects: Rc<RefCell<Device>>;
            match devices.get(&device_which_connects_label) {
                Some(x) => {
                    device_which_connects = x.clone();
                }
                None => {
                    let new_device = Device::new();
                    let new_device_ref = Rc::new(RefCell::new(new_device));
                    devices.insert(device_which_connects_label, new_device_ref.clone());
                    device_which_connects = new_device_ref;
                }
            }
            device_to_modify.borrow_mut().connections.push(Rc::downgrade(&device_which_connects));
        }
    }
    let mut current_devices = vec![devices.get("you").unwrap().clone()];
    let mut next_devices: Vec<_> = Vec::new();
    while current_devices.len() != 0 {
        for device in current_devices.iter() {
            for connecting_device_weak in &device.borrow().connections {
                let connecting_device = connecting_device_weak.upgrade().unwrap();
                let mut connecting_device_mut = connecting_device.borrow_mut();
                connecting_device_mut.ways_to_reach += 1;
                next_devices.push(connecting_device.clone());
            }
        }
        swap(&mut current_devices, &mut next_devices);
        next_devices.clear();
    }
    Ok(devices.get("out").unwrap().borrow().ways_to_reach)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut file_contents: Vec<u8> = Vec::new();
    let mut file = File::open("day11.txt")?;
    file.read_to_end(&mut file_contents)?;
    let content = String::from_utf8(file_contents)?;
    println!("{}", not_dumb_solution(&content)?);
    Ok(())
}
