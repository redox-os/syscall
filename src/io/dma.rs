use core::{mem, ptr};
use core::ops::{Deref, DerefMut};

use crate::Result;

struct PhysBox {
    address: usize,
    size: usize
}

impl PhysBox {
    fn new(size: usize) -> Result<PhysBox> {
        let address = unsafe { crate::physalloc(size)? };
        Ok(PhysBox {
            address: address,
            size: size
        })
    }
}

impl Drop for PhysBox {
    fn drop(&mut self) {
        let _ = unsafe { crate::physfree(self.address, self.size) };
    }
}

pub struct Dma<T: ?Sized> {
    phys: PhysBox,
    virt: *mut T
}

impl<T> Dma<T> {
    pub fn new(value: T) -> Result<Dma<T>> {
        let phys = PhysBox::new(mem::size_of::<T>())?;
        let virt = unsafe { crate::physmap(phys.address, phys.size, crate::PHYSMAP_WRITE)? } as *mut T;
        unsafe { ptr::write(virt, value); }
        Ok(Dma {
            phys: phys,
            virt: virt
        })
    }

    pub fn zeroed() -> Result<Dma<T>> {
        let phys = PhysBox::new(mem::size_of::<T>())?;
        let virt = unsafe { crate::physmap(phys.address, phys.size, crate::PHYSMAP_WRITE)? } as *mut T;
        unsafe { ptr::write_bytes(virt as *mut u8, 0, phys.size); }
        Ok(Dma {
            phys: phys,
            virt: virt
        })
    }
}
impl<T: ?Sized> Dma<T> {
    pub fn physical(&self) -> usize {
        self.phys.address
    }
}

impl<T> Dma<[T]> {
    /// Crates a new DMA buffer with a size only known at runtime.
    /// ## Safety
    /// * `T` must be properly aligned.
    /// * `T` must be valid as zeroed (i.e. no NonNull pointers).
    pub unsafe fn zeroed_unsized(count: usize) -> Result<Self> {
        let phys = PhysBox::new(mem::size_of::<T>() * count)?;
        let virt_ptr = crate::physmap(phys.address, phys.size, crate::PHYSMAP_WRITE)? as *mut T;
        ptr::write_bytes(virt_ptr, 0, count);

        let virt = core::slice::from_raw_parts_mut(virt_ptr, count);

        Ok(Dma {
            phys,
            virt,
        })
    }
}

impl<T: ?Sized> Deref for Dma<T> {
    type Target = T;
    fn deref(&self) -> &T {
        unsafe { &*self.virt }
    }
}

impl<T: ?Sized> DerefMut for Dma<T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.virt }
    }
}

impl<T: ?Sized> Drop for Dma<T> {
    fn drop(&mut self) {
        unsafe { ptr::drop_in_place(self.virt) }
        let _ = unsafe { crate::physunmap(self.virt as *mut u8 as usize) };
    }
}
