use std::ffi::CString;
use std::{io, slice};
use std::io::Write;

mod ffi {
  #![allow(non_upper_case_globals)]
  #![allow(non_camel_case_types)]
  #![allow(non_snake_case)]

  include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

pub struct Decoder {
  decoder: *mut ffi::Decoder
}

// TODO(Assasans): Not sure...
unsafe impl Send for Decoder {}
unsafe impl Sync for Decoder {}

impl Decoder {
  pub fn new() -> Self {
    Self {
      decoder: unsafe { ffi::decoder_alloc() }
    }
  }

  pub fn open_input(&self, path: &str) {
    let path = CString::new(path).unwrap();
    unsafe {
      ffi::decoder_open_input(self.decoder, path.as_ptr());
    }
  }

  pub fn init_filters(&self, filters_descr: &str) {
    let filters_descr = CString::new(filters_descr).unwrap();
    unsafe {
      ffi::decoder_init_filters(self.decoder, filters_descr.as_ptr());
    }
  }

  pub fn read_frame(&self) -> Vec<f32> {
    let mut data_ptr = std::ptr::null_mut();
    let mut data_length = 0;
    unsafe {
      ffi::decoder_read_frame(self.decoder, &mut data_ptr, &mut data_length);
    }

    let data_slice = unsafe { slice::from_raw_parts(data_ptr, data_length as usize) };
    data_slice.to_vec()
  }
}

impl Drop for Decoder {
  fn drop(&mut self) {
    unsafe { ffi::decoder_free(self.decoder) };
  }
}

#[test]
fn run() {
  let decoder = Decoder::new();
  decoder.open_input("https://rr2---sn-qo5-2vgs.googlevideo.com/videoplayback?expire=1689993017&ei=2eq6ZJO0JO2Xv_IP4MWsCA&ip=176.93.44.73&id=o-AOw2Eiob0WYOjTrgY8UJjXCg2rp9Tm5p74GwWlBT3aQM&itag=251&source=youtube&requiressl=yes&mh=sS&mm=31%2C29&mn=sn-qo5-2vgs%2Csn-ixh7rn76&ms=au%2Crdu&mv=m&mvi=2&pl=21&gcr=fi&initcwndbps=1338750&spc=Ul2Sq4B7S9MLUmFmVQbQP0lju-bjCgs&vprv=1&svpuc=1&mime=audio%2Fwebm&gir=yes&clen=5357975&dur=284.781&lmt=1566010568166984&mt=1689971128&fvip=3&keepalive=yes&fexp=24007246%2C24363392&c=ANDROID&txp=2311222&sparams=expire%2Cei%2Cip%2Cid%2Citag%2Csource%2Crequiressl%2Cgcr%2Cspc%2Cvprv%2Csvpuc%2Cmime%2Cgir%2Cclen%2Cdur%2Clmt&sig=AOq0QJ8wRQIgPQimcgkZ30ERgbuK1nFz_tQaM4QLKyRJ-HBFqN6KiIQCIQDs2gG8U1ZW1u7wDRGtvdGZTfLs-KshYk8SPyGBwUnc5Q%3D%3D&lsparams=mh%2Cmm%2Cmn%2Cms%2Cmv%2Cmvi%2Cpl%2Cinitcwndbps&lsig=AG3C_xAwRAIgbDw3ole897m6dy9Nl1QW-eijNK1RnvVHXr84Fn6gyGICIB9W3jOeL4l3GJfttoZmFRaOaJeCvjL7-0OL5138n66I");
  // decoder.open_input("file:///run/media/assasans/D2C29497C2948201/Documents/REAPER Media/Катерина 2/3.mp3");
  decoder.init_filters("lv2=p=http\\\\://calf.sourceforge.net/plugins/BassEnhancer:c=amount=3,alimiter=limit=0.891251");
  // decoder.init_filters("anull");
  let stdout = io::stdout();
  let mut handle = stdout.lock();
  loop {
    let frame = decoder.read_frame();
    eprintln!("Frame {} samples", frame.len());

    for sample in frame {
      let bytes = sample.to_le_bytes();
      handle.write_all(&bytes).unwrap();
    }
    handle.flush().unwrap();
  }
}
