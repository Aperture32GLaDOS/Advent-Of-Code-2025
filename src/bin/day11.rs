use std::{cell::{RefCell, RefMut}, collections::HashMap, fs::File, io::Read, mem::swap, rc::{Rc, Weak}};

#[derive(Debug)]
struct Device {
    // Weak so that they do not cause memory leak
    connections: Vec<Weak<RefCell<Device>>>,
    label: String,
}

impl Device {
    fn new(label: &str) -> Self {
        Device { connections: Vec::new(), label: label.to_string() }
    }

    fn dfs(&self, to: &str, cache: &mut HashMap<String, usize>) -> usize {
        if self.label == to {
            return 1;
        }
        else {
            let cache_result = cache.get(&self.label);
            match cache_result {
                Some(x) => {
                    return *x;
                }
                None => {
                    let dfs_result = self.connections.iter().map(|x| x.upgrade().unwrap().borrow().dfs(to, cache)).sum();
                    cache.insert(self.label.clone(), dfs_result);
                    return dfs_result;
                }
            }
        }
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
                let new_device = Device::new(device_to_modify_label);
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
                    let new_device = Device::new(device_which_connects_label);
                    let new_device_ref = Rc::new(RefCell::new(new_device));
                    devices.insert(device_which_connects_label, new_device_ref.clone());
                    device_which_connects = new_device_ref;
                }
            }
            device_to_modify.borrow_mut().connections.push(Rc::downgrade(&device_which_connects));
        }
    }
    let root = devices.get("svr").unwrap().borrow();
    let fft = devices.get("fft").unwrap().borrow();
    let dac = devices.get("dac").unwrap().borrow();
    let mut root_dfs_cache = HashMap::new();
    let mut fft_dfs_cache = HashMap::new();
    let mut dac_dfs_cache = HashMap::new();
    let possible_ways = root.dfs("fft", &mut root_dfs_cache) * fft.dfs("dac", &mut fft_dfs_cache) * dac.dfs("out", &mut dac_dfs_cache);
    Ok(possible_ways)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut file_contents: Vec<u8> = Vec::new();
    let mut file = File::open("day11.txt")?;
    file.read_to_end(&mut file_contents)?;
    let content = String::from_utf8(file_contents)?;
    println!("{}", not_dumb_solution(&content)?);
    Ok(())
}
