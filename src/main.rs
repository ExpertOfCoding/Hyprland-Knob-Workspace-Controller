use std::path::PathBuf;

use evdev::{
    AttributeSet, Device, EventSummary, EventType, InputEvent, KeyCode, enumerate,
    uinput::VirtualDevice,
};
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut paths: Vec<PathBuf> = Vec::new();
    let mut keys = AttributeSet::<KeyCode>::new();
    keys.insert(KeyCode::KEY_LEFTMETA);
    keys.insert(KeyCode::KEY_1);
    keys.insert(KeyCode::KEY_2);
    keys.insert(KeyCode::KEY_3);
    keys.insert(KeyCode::KEY_4);
    let mut vdevice = VirtualDevice::builder()?
        .name("ElPsyKongrooKeyboard")
        .with_keys(&keys)?
        .build()?;

    for (path, device) in enumerate() {
        let name = device.name().unwrap_or("");
        if name == "uinput" || name == "" {
            continue;
        }
        //if let Some(keys) = device.supported_keys(){
        //    if keys.contains(KeyCode::KEY_VOLUMEDOWN) && keys.contains(KeyCode::KEY_VOLUMEUP){
        //    println!("{:?}",path);
        //
        //    }
        //}
        if name.to_lowercase().contains("keyboard")
            && device.supported_keys().map_or(false, |f| {
                f.contains(KeyCode::KEY_VOLUMEUP)
                    && f.contains(KeyCode::KEY_VOLUMEDOWN)
                    && f.contains(KeyCode::KEY_MUTE)
            })
        {
            paths.push(path);
        }
    }
    let mut devices: Vec<Device> = Vec::new();
    for path in paths {
        let device = Device::open(path.as_path())?;
        device.set_nonblocking(true)?;
        devices.push(device);
    }
    let keycodes = [
        KeyCode::KEY_1,
        KeyCode::KEY_2,
        KeyCode::KEY_3,
        KeyCode::KEY_4,
    ];
    let mut current = 0;
    loop {
        for device in devices.iter_mut() {
            if let Ok(events) = device.fetch_events() {
                for event in events {
                    let mut found = true;
                    match event.destructure() {
                        EventSummary::Key(_, KeyCode::KEY_VOLUMEUP, 1) => {
                            current = (current + 1) % 4
                        }
                        EventSummary::Key(_, KeyCode::KEY_VOLUMEDOWN, 1) => {
                            if current > 0 {
                                current -= 1
                            } else {
                                current = 3
                            }
                        }
                        _ => found = false,
                    }
                    if found {
                        let keydown_super: InputEvent =
                            InputEvent::new_now(EventType::KEY.0, KeyCode::KEY_LEFTMETA.code(), 1);
                        let keyup_super: InputEvent =
                            InputEvent::new_now(EventType::KEY.0, KeyCode::KEY_LEFTMETA.code(), 0);
                        vdevice.emit(&[
                            keydown_super,
                            InputEvent::new_now(EventType::KEY.0, keycodes[current].code(), 1),
                            //InputEvent::new_now(EventType::SYNCHRONIZATION.0, 0, 0), // Kernel'a event paketini tamamla mesajı
                        ])?;
                        vdevice.emit(&[
                            keyup_super,
                            InputEvent::new_now(EventType::KEY.0, keycodes[current].code(), 0),
                            //InputEvent::new_now(EventType::SYNCHRONIZATION.0, 0, 0), // Kernel'a event paketini tamamla mesajı
                        ])?;
                    }
                }
            }
        }
    }
}


// because we set non blocking we need to use Ok to ignore errors such as resource temporarily unavailable
/*
            for event in device.fetch_events().unwrap() {
                match event.destructure() {
                    EventSummary::Key(ev, key_type, 1) => {
                        println!("event : {:?} ,key_type : {:?}", ev, key_type);
                        if key_type == KeyCode::KEY_VOLUMEUP {
                            let keydown_super: InputEvent = InputEvent::new_now(
                                EventType::KEY.0,
                                KeyCode::KEY_LEFTMETA.code(),
                                1,
                            );
                            let keyup_super: InputEvent = InputEvent::new_now(
                                EventType::KEY.0,
                                KeyCode::KEY_LEFTMETA.code(),
                                0,
                            );
                            current += 1;
                            current = current % 4;
                            vdevice.emit(&[
                                keydown_super,
                                InputEvent::new_now(EventType::KEY.0, keycodes[current].code(), 1),
                            ])?;
                            vdevice.emit(&[
                                keyup_super,
                                InputEvent::new_now(EventType::KEY.0, keycodes[current].code(), 0),
                            ])?;
                        }
                        if key_type == KeyCode::KEY_VOLUMEDOWN {
                            let keydown_super: InputEvent = InputEvent::new_now(
                                EventType::KEY.0,
                                KeyCode::KEY_LEFTMETA.code(),
                                1,
                            );
                            let keyup_super: InputEvent = InputEvent::new_now(
                                EventType::KEY.0,
                                KeyCode::KEY_LEFTMETA.code(),
                                0,
                            );
                            if current > 0 {
                                current -= 1;
                            } else {
                                current = 3
                            }

                            vdevice.emit(&[keydown_super]).unwrap();
                            vdevice
                                .emit(&[InputEvent::new_now(
                                    EventType::KEY.0,
                                    keycodes[current].code(),
                                    1,
                                )])
                                .unwrap();
                            vdevice
                                .emit(&[InputEvent::new_now(
                                    EventType::KEY.0,
                                    keycodes[current].code(),
                                    0,
                                )])
                                .unwrap();
                            vdevice.emit(&[keyup_super]).unwrap();
                        }
                    }
                    _ => continue,
                }
            }
*/