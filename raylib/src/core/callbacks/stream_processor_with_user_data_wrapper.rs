use lazy_static::lazy_static;
use paste::paste;
use raylib_sys::{AttachAudioStreamProcessor, AudioStream};
use std::sync::Mutex;

type RawAudioCallbackWithUserData = extern "C" fn(
    user_data: *mut ::std::os::raw::c_void,
    data_ptr: *mut ::std::os::raw::c_void,
    frames: u32,
) -> ();

pub struct AudioCallbackWithUserData {
    user_data: *mut ::std::os::raw::c_void,
    callback: Option<RawAudioCallbackWithUserData>,
}

unsafe impl Send for AudioCallbackWithUserData {} //??

impl AudioCallbackWithUserData {
    pub fn new(
        user_data: *mut ::std::os::raw::c_void,
        raw_callback: RawAudioCallbackWithUserData,
    ) -> Self {
        AudioCallbackWithUserData {
            user_data: user_data,
            callback: Some(raw_callback),
        }
    }
}

impl Default for AudioCallbackWithUserData {
    fn default() -> Self {
        AudioCallbackWithUserData {
            user_data: std::ptr::null_mut(),
            callback: None,
        }
    }
}

macro_rules! generate_functions {
  ( $( $n:literal ),* ) => {
      paste! {
          lazy_static! {
              $(
                  static ref [< CLOSURE_ $n >]: Mutex<AudioCallbackWithUserData> = Mutex::new(AudioCallbackWithUserData::default());
              )*
          }

          // Function to set the closure
          fn set_closure(audio_callback: AudioCallbackWithUserData) -> usize {
              $(
                  {
                      let mut guard = [< CLOSURE_ $n >].lock().unwrap();
                      if (*guard).callback == None {
                        *guard = audio_callback;
                        return $n;
                      }
                  }
              )*
              panic!("index out of bounds");
          }

          // Function to set the closure
          fn clear_closure(index: usize) {
              $(
                  if index == $n {
                      let mut guard = [< CLOSURE_ $n >].lock().unwrap();
                      if (*guard).callback == None {
                          panic!(
                              "No callbacks registered under this number ({}).",
                              index
                          );
                      }
                      *guard = AudioCallbackWithUserData::default();
                  }
              )*
              panic!("index out of bounds");
          }

          $(
            #[no_mangle]
            pub extern "C" fn [< callback_ $n >](data_ptr: *mut ::std::os::raw::c_void, frames: u32) -> () {
              println!("raw callback $n");
              let guard = [< CLOSURE_ $n >].lock().unwrap();
              let audio_callback = &(*guard);
              if let Some(callback) = audio_callback.callback {
                (callback)(audio_callback.user_data, data_ptr, frames);
              } else {
                  panic!("unexpected: no callback $n set")
              }
            }
          )*

          // Function to get the callback
          fn get_callback(index: usize) -> extern "C" fn(data_ptr: *mut ::std::os::raw::c_void, frames: u32) {
            $(
                if index == $n {
                    return [< callback_ $n >];
                }
            )*
            panic!("index out of bounds");
          }
        }
  }
}

const N: usize = 20;
generate_functions!(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19);
lazy_static! {
    static ref CURRENT_IDX: Mutex<usize> = Mutex::new(0);
}

pub fn attach_audio_stream_processor_with_user_data(
    //    user_data: *mut ::std::os::raw::c_void,
    stream: AudioStream,
    callback: AudioCallbackWithUserData,
) -> usize {
    let idx = set_closure(callback);
    unsafe {
        AttachAudioStreamProcessor(stream, Some(get_callback(idx)));
    }
    idx
}

pub fn detach_audio_stream_processor_with_user_data(index: usize) {
    clear_closure(index);
}
