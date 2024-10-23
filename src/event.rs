use virt::error::Error;

// wrapper for callbacks
unsafe extern "C" fn event_callback<T: Event>(
    watch: libc::c_int,
    fd: libc::c_int,
    events: libc::c_int,
    opaque: *mut libc::c_void,
) {
    let events = events as virt_sys::virEventHandleType;
    let console = &mut *(opaque as *mut T);
    if let Some(callback) = &mut console.get_callback() {
        callback(watch, fd, events, opaque);
    }
}

unsafe extern "C" fn event_free(_opaque: *mut libc::c_void) {}

pub type EventHandleCallback =
    Box<dyn FnMut(libc::c_int, libc::c_int, virt_sys::virEventHandleType, *mut libc::c_void)>;

pub trait Event {
    fn event_add_handle<
        F: 'static + FnMut(libc::c_int, libc::c_int, virt_sys::virEventHandleType, *mut libc::c_void),
        T: Event,
    >(
        &mut self,
        fd: libc::c_int,
        events: virt_sys::virEventHandleType,
        cb: F,
    ) -> Result<libc::c_int, Error> {
        let ptr = self as *mut _ as *mut _;
        let ret = unsafe {
            virt_sys::virEventAddHandle(
                fd,
                events as libc::c_int,
                Some(event_callback::<T>),
                ptr,
                Some(event_free),
            )
        };
        if ret == -1 {
            return Err(Error::last_error());
        }
        self.set_callback(Box::new(cb));
        Ok(ret)
    }

    fn event_remove_handle(watch: libc::c_int) -> Result<(), Error> {
        let ret = unsafe { virt_sys::virEventRemoveHandle(watch) };
        if ret == -1 {
            return Err(Error::last_error());
        }
        Ok(())
    }

    fn event_update_handle(watch: libc::c_int, events: libc::c_int) {
        unsafe { virt_sys::virEventUpdateHandle(watch, events) };
    }

    fn set_callback(&mut self, cb: EventHandleCallback);
    fn get_callback(&mut self) -> &mut Option<EventHandleCallback>;
}

pub fn event_register_default_impl() -> Result<(), Error> {
    let ret = unsafe { virt_sys::virEventRegisterDefaultImpl() };
    if ret == -1 {
        return Err(Error::last_error());
    }
    Ok(())
}

pub fn event_run_default_impl() -> Result<(), Error> {
    let ret = unsafe { virt_sys::virEventRunDefaultImpl() };
    if ret == -1 {
        return Err(Error::last_error());
    }
    Ok(())
}
