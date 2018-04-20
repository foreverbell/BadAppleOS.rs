#[lang = "eh_personality"]
#[no_mangle]
#[allow(private_no_mangle_fns)]
pub extern "C" fn eh_personality() {}

#[lang = "panic_fmt"]
#[no_mangle]
#[allow(private_no_mangle_fns)]
pub extern "C" fn panic_fmt() -> ! {
  loop {}
}
