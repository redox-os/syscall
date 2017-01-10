use collections::{BTreeMap, Vec};
use collections::boxed::Box;

use ::Result;

pub struct EventQueue<R> {
    /// The file to read events from
    file: usize,
    /// A map of registered file descriptors to their handler callbacks
    callbacks: BTreeMap<usize, Box<FnMut(usize) -> Result<Option<R>>>>
}

impl<R> EventQueue<R> {
    /// Create a new event queue
    pub fn new() -> Result<EventQueue<R>> {
        Ok(EventQueue {
            file: ::open("event:", ::O_RDONLY)?,
            callbacks: BTreeMap::new()
        })
    }

    /// Add a file to the event queue, calling a callback when an event occurs
    ///
    /// The callback is given a mutable reference to the file and the event data
    /// (typically the length of data available for read)
    ///
    /// The callback returns Ok(None) if it wishes to continue the event loop,
    /// or Ok(Some(R)) to break the event loop and return the value.
    /// Err can be used to allow the callback to return an I/O error, and break the
    /// event loop
    pub fn add<F: FnMut(usize) -> Result<Option<R>> + 'static>(&mut self, fd: usize, callback: F) -> Result<()> {
        ::fevent(fd, ::EVENT_READ)?;

        self.callbacks.insert(fd, Box::new(callback));

        Ok(())
    }

    /// Remove a file from the event queue, returning its callback if found
    pub fn remove(&mut self, fd: usize) -> Result<Option<Box<FnMut(usize) -> Result<Option<R>>>>> {
        if let Some(callback) = self.callbacks.remove(&fd) {
            ::fevent(fd, 0)?;

            Ok(Some(callback))
        } else {
            Ok(None)
        }
    }

    /// Send an event to a descriptor callback
    pub fn trigger(&mut self, fd: usize, count: usize) -> Result<Option<R>> {
        if let Some(callback) = self.callbacks.get_mut(&fd) {
            callback(count)
        } else {
            Ok(None)
        }
    }

    /// Send an event to all descriptor callbacks, useful for cleaning out buffers after init
    pub fn trigger_all(&mut self, count: usize) -> Result<Vec<R>> {
        let mut rets = Vec::new();
        for (_fd, callback) in self.callbacks.iter_mut() {
            if let Some(ret) = callback(count)? {
                rets.push(ret);
            }
        }
        Ok(rets)
    }

    /// Process the event queue until a callback returns Some(R)
    pub fn run(&mut self) -> Result<R> {
        loop {
            let mut event = ::Event::default();
            if ::read(self.file, &mut event)? > 0 {
                if let Some(ret) = self.trigger(event.id, event.data)? {
                    return Ok(ret);
                }
            }
        }
    }
}
