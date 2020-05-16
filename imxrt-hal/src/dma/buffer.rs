//! DMA memory buffers

use super::Element;

use as_slice::{AsMutSlice, AsSlice};
use core::cell::UnsafeCell;
use core::ptr;
use core::sync::atomic::{AtomicBool, Ordering};

/// A dedicated DMA memory buffer for transfers and receives
///
/// `Buffer`s may be statically allocated. They are "owned" by a
/// DMA memory adapter. The ownership is enforced at runtime.
/// `Buffer`s should store arrays of `u8`, `u16`, or `u32` elements.
///
/// ```
/// use imxrt_hal::dma;
/// static UART2_DMA_RX: dma::Buffer<[u8; 256]> = dma::Buffer::new([0; 256]);
/// ```
#[repr(C)] // Need guaranteed layout for checking memory aligmnent, required by circular buffer
pub struct Buffer<B> {
    /// A mutable array that will be used by both the hardware DMA channel
    /// and the user.
    memory: UnsafeCell<B>,
    /// `true` if this buffer has been taken, else `false`
    taken: AtomicBool,
}

// Its safe to allocate `Buffer` as an immutable static. The two `Buffer` adapters
// will guarantee exclusive ownership of the mutable memory by setting the `taken`
// flag. This memory will be shared with the DMA hardware.
unsafe impl<B> Sync for Buffer<B> {}

impl<B> Buffer<B> {
    /// Create a buffer that wraps the provided memory
    ///
    /// Should be used to allocate a `static` buffer.
    ///
    /// ```
    /// use imxrt_hal::dma;
    /// static UART2_DMA_RX: dma::Buffer<[u8; 256]> = dma::Buffer::new([0; 256]);
    /// ```
    pub const fn new(memory: B) -> Self {
        Buffer {
            memory: UnsafeCell::new(memory),
            taken: AtomicBool::new(false),
        }
    }
}

/// A linear DMA buffer
///
/// The memory is represented as a slice of bytes. Use [`as_elements()`](struct.Linear.html#method.as_elements)
/// to read from the buffer, or [`as_mut_elements()`](struct.Linear.html#method.as_mut_elements) to read and
/// write from the buffer.
///
/// ```
/// use imxrt_hal::dma;
///
/// static DMA1_BUFFER: dma::Buffer<[u8; 256]> = dma::Buffer::new([0; 256]);
///
/// let linear = dma::Linear::new(&DMA1_BUFFER).unwrap();
/// // DMA1_BUFFER is owned by linear. If we try to use it again,
/// // it returns None.
/// assert!(dma::Linear::new(&DMA1_BUFFER).is_none());
/// ```
pub struct Linear<E> {
    /// Pointer to array
    ///
    /// Required to have static lifetime when using the safe interfaces
    ptr: *mut E,
    /// Length of the static memory buffer
    len: usize,
}

impl<E> Linear<E>
where
    E: Element,
{
    /// Create a new `Linear` DMA buffer that takes ownership of the memory described
    /// by `buffer`
    ///
    /// If the constructor has exclusive ownership of `buffer`, returns `Some(Linear)`.
    /// Returns `None` if the `buffer` is already owned.
    pub fn new<B>(buffer: &'static Buffer<B>) -> Option<Self>
    where
        B: AsMutSlice<Element = E>,
    {
        let taken = buffer.taken.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            unsafe { Some(Self::new_unchecked(buffer)) }
        }
    }

    /// Create a `Linear` DMA buffer without checking the ownership of the
    /// supplied `buffer`
    ///
    /// # Safety
    ///
    /// Using this method may result in two, mutable references to static memory,
    /// which may cause memory unsafey. Caller must ensure that the provided `buffer`
    /// is no longer owned.
    pub unsafe fn new_unchecked<B>(buffer: &'static Buffer<B>) -> Self
    where
        B: AsMutSlice<Element = E>,
    {
        let memory = &mut *buffer.memory.get();
        Self::from_raw(memory)
    }

    /// Create a `Linear` DMA buffer without any concern to ownership or memory
    /// lifetime
    ///
    /// # Safety
    ///
    /// The caller must guarantee that the lifetime of `raw` is greater than the
    /// lifetime of all DMA transfers that use the memory. The caller must guarantee
    /// that there are no other mutable references to this memory.
    pub unsafe fn from_raw<B>(mut raw: B) -> Self
    where
        B: AsMutSlice<Element = E>,
    {
        let ptr = raw.as_mut_slice().as_mut_ptr();
        let len = raw.as_mut_slice().len();
        Linear { ptr, len }
    }

    /// Returns a slice to the elements in the linear buffer
    pub fn as_elements(&self) -> &[E] {
        unsafe { core::slice::from_raw_parts(self.ptr, self.len) }
    }

    /// Returns a mutable slice to the elements in the linear buffer
    pub fn as_mut_elements(&mut self) -> &mut [E] {
        unsafe { core::slice::from_raw_parts_mut(self.ptr, self.len) }
    }
}

impl<E: Element> AsRef<[E]> for Linear<E> {
    fn as_ref(&self) -> &[E] {
        self.as_elements()
    }
}

impl<E: Element> AsMut<[E]> for Linear<E> {
    fn as_mut(&mut self) -> &mut [E] {
        self.as_mut_elements()
    }
}

impl<E: Element> AsSlice for Linear<E> {
    type Element = E;
    fn as_slice(&self) -> &[E] {
        self.as_elements()
    }
}

impl<E: Element> AsMutSlice for Linear<E> {
    fn as_mut_slice(&mut self) -> &mut [E] {
        self.as_mut_elements()
    }
}

/// A circular DMA queue
///
/// ```
/// use imxrt_hal::dma;
///
/// // A newtype to enforce the required alignment
/// #[repr(align(512))]
/// struct Align512(dma::Buffer<[u16; 512]>);
///
/// static BUFFER: Align512 = Align512(dma::Buffer::new([0; 512]));
///
/// let mut circular = dma::Circular::new(&BUFFER.0).unwrap();
/// // BUFFER is taken and cannot be used again
/// assert_eq!(dma::Circular::new(&BUFFER.0).unwrap_err(), dma::CircularError::BufferTaken);
///
/// // The maximum number of elements is one less than the size of the backing
/// // memory.
/// assert_eq!(circular.capacity(), 511);
///
/// circular.push(1);
/// circular.push(2);
/// circular.push(3);
/// assert_eq!(circular.len(), 3);
///
/// assert_eq!(circular.pop(), Some(1));
/// assert_eq!(circular.pop(), Some(2));
/// assert!(!circular.is_empty());
///
/// assert_eq!(circular.pop(), Some(3));
/// assert_eq!(circular.pop(), None);
/// assert!(circular.is_empty());
/// ```
///
/// If the underlying buffer size is *not* a power of two, we cannot create a
/// circular DMA queue:
///
/// ```
/// use imxrt_hal::dma;
///
/// #[repr(align(32))]
/// struct Align32(dma::Buffer<[u16; 30]>);
/// static BUFFER: Align32 = Align32(dma::Buffer::new([0; 30]));
///
/// let err = dma::Circular::new(&BUFFER.0).expect_err("30 is not a power of two");
/// assert_eq!(err, dma::CircularError::NotPowerOfTwo);
/// ```
#[derive(Debug)]
pub struct Circular<E> {
    /// Pointer to memory buffer
    ///
    /// Lifetime is static when using the safe interface
    ptr: *mut E,
    /// Total capacity of the buffer (length of the array)
    ///
    /// Will be a power of two
    cap: usize,
    /// Read position
    read: usize,
    /// Write position
    write: usize,
    /// Tracking for the number of elements in a DMA transfer
    ///
    /// Used by the `Source` and `Destination` implemenetations
    transfer: usize,
}

/// Possible errors when creating a circular buffer
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum CircularError {
    /// The size of the memory is not a power of two
    NotPowerOfTwo,
    /// The buffer is taken, likely used in another DMA buffer
    BufferTaken,
    /// The alignment of the buffer must be a multiple of the buffer's
    /// size
    IncorrectAlignment,
}

impl<E: Element> Circular<E> {
    /// Creates a new circular DMA buffer using the memory supplied by `buffer`
    ///
    /// If the `buffer` is already taken, returns `Err(CircularError::BufferTaken)`.
    /// If the size of the buffer is *not* a power of two, returns `Err(CircularError::NotPowerOfTwo)`.
    pub fn new<B>(buffer: &'static Buffer<B>) -> Result<Self, CircularError>
    where
        B: AsMutSlice<Element = E>,
    {
        let taken = buffer.taken.swap(true, Ordering::SeqCst);
        if taken {
            Err(CircularError::BufferTaken)
        } else {
            unsafe { Self::new_unchecked(buffer) }.or_else(|err| {
                buffer.taken.store(false, Ordering::SeqCst);
                Err(err)
            })
        }
    }

    /// Creates a new circular DMA buffer using the memory supplied by `buffer`, but do not
    /// check for buffer ownership.
    ///
    /// If the size of the buffer is *not* a power of two, returns `Err(CircularError::NotPowerOfTwo)`.
    ///
    /// # Safety
    ///
    /// Caller must ensure that the `buffer` is not in use anywhere else. Otherwise, there will be
    /// more than one owner of mutable memory.
    pub unsafe fn new_unchecked<B>(buffer: &'static Buffer<B>) -> Result<Self, CircularError>
    where
        B: AsMutSlice<Element = E>,
    {
        Self::from_raw(&mut *buffer.memory.get())
    }

    /// Creates a new circular DMA buffer from arbitrary memory
    ///
    /// If the size of the buffer is *not* a power of two, returns `Err(CircularError::NotPowerOfTwo)`.
    ///
    /// # Safety
    ///
    /// Caller must ensure that the lifetime of `raw` is greater than all the DMA transfers that use
    /// the memory. Caller must ensure that there are no other mutable references to this memory.
    pub unsafe fn from_raw<B>(mut raw: B) -> Result<Self, CircularError>
    where
        B: AsMutSlice<Element = E>,
    {
        let cap = raw.as_mut_slice().len();
        let ptr = raw.as_mut_slice().as_mut_ptr();
        if !cap.is_power_of_two() {
            Err(CircularError::NotPowerOfTwo)
        } else if (ptr as usize) % cap != 0 {
            Err(CircularError::IncorrectAlignment)
        } else {
            Ok(Circular {
                ptr,
                cap,
                read: 0,
                write: 0,
                transfer: 0,
            })
        }
    }

    /// Returns the number of readable elements in the queue
    pub fn len(&self) -> usize {
        self.write.wrapping_sub(self.read) & (self.cap - 1)
    }

    /// Returns `true` if the queue has no readable elements
    pub fn is_empty(&self) -> bool {
        self.write == self.read
    }

    /// Returns the number of elements the circular queue can hold
    pub fn capacity(&self) -> usize {
        self.cap - 1
    }

    /// Pushes an element into the circular queue
    ///
    /// Returns `true` if the element was enqueued, or `false`
    /// if there wasn't enough space
    pub fn push(&mut self, element: E) -> bool {
        if self.len() == (self.cap - 1) {
            false
        } else {
            unsafe {
                ptr::write(self.ptr.add(self.write), element);
            }
            self.write = (self.write + 1) & (self.cap - 1);
            true
        }
    }

    /// Peeks at the next element in the queue
    ///
    /// Returns `None` if there are no elements to read.
    pub fn peek(&self) -> Option<E> {
        if self.is_empty() {
            None
        } else {
            Some(unsafe { ptr::read(self.ptr.add(self.read)) })
        }
    }

    /// Remove the next element from the queue
    pub fn pop(&mut self) -> Option<E> {
        self.peek().and_then(|elem| {
            self.read = (self.read + 1) & (self.cap - 1);
            Some(elem)
        })
    }

    /// Returns an iterator that can drain the readable contents from
    /// the circular queue
    ///
    /// Each iteration calls `pop()`, returning the next readable element
    /// in the queue, until the read portion is exhausted.
    pub fn drain(&mut self) -> Drain<E> {
        Drain(self)
    }

    /// Returns the pointer to the start of the readable queue memory
    fn read_ptr(&self) -> *const E {
        unsafe { self.ptr.add(self.read) }
    }

    /// Returns the pointer to the start of the writeable queue memory
    fn write_ptr(&mut self) -> *mut E {
        unsafe { self.ptr.add(self.write) }
    }

    /// Mark `size` elements as read
    ///
    /// Equivalent to calling `pop()` `size` times, and dropping
    /// the elements.
    fn mark_read(&mut self, size: usize) {
        self.read = (self.read + size) & (self.cap - 1);
    }

    /// Mark `size` elements as written
    ///
    /// Equivalent to calling `push()` `size` times.
    fn mark_written(&mut self, size: usize) {
        self.write = (self.write + size) & (self.cap - 1)
    }
}

/// An iterator that will drain the contents from
/// a circular DMA buffer.
///
/// See the [`drain()`] method for details.
pub struct Drain<'a, E>(&'a mut Circular<E>);

impl<'a, E: Element> Iterator for Drain<'a, E> {
    type Item = E;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.0.len(), Some(self.0.len()))
    }
}

impl<'a, E: Element> ExactSizeIterator for Drain<'a, E> {}

pub trait Source<E: Element>: private::Sealed {
    fn set_source(&mut self, channel: &mut super::Channel, size: usize);
    fn complete_source(&mut self);
    fn source_capacity(&self) -> usize;
}

pub trait Destination<E: Element>: private::Sealed {
    fn set_destination(&mut self, channel: &mut super::Channel, size: usize);
    fn complete_destination(&mut self);
    fn destination_capacity(&self) -> usize;
}

mod private {
    pub trait Sealed {}

    use super::{Circular, Linear};
    impl<E> Sealed for Linear<E> {}
    impl<E> Sealed for Circular<E> {}
}

impl<E: Element> Source<E> for Linear<E> {
    fn set_source(&mut self, channel: &mut super::Channel, size: usize) {
        unsafe {
            channel.set_source_buffer(&self.as_elements()[..size]);
        }
    }
    fn complete_source(&mut self) {}
    fn source_capacity(&self) -> usize {
        self.len
    }
}

impl<E: Element> Destination<E> for Linear<E> {
    fn set_destination(&mut self, channel: &mut super::Channel, size: usize) {
        unsafe {
            channel.set_destination_buffer(&mut self.as_mut_elements()[..size]);
        }
    }
    fn complete_destination(&mut self) {}
    fn destination_capacity(&self) -> usize {
        self.len
    }
}

impl<E: Element> Source<E> for Circular<E> {
    fn set_source(&mut self, channel: &mut super::Channel, size: usize) {
        unsafe { channel.set_source_circular(self.read_ptr(), self.cap) };
        self.mark_read(size);
    }
    fn complete_source(&mut self) {}
    fn source_capacity(&self) -> usize {
        self.capacity()
    }
}

impl<E: Element> Destination<E> for Circular<E> {
    fn set_destination(&mut self, channel: &mut super::Channel, size: usize) {
        unsafe { channel.set_destination_circular(self.write_ptr(), self.cap) }
        self.transfer = size;
    }
    fn complete_destination(&mut self) {
        self.mark_written(self.transfer);
    }
    fn destination_capacity(&self) -> usize {
        self.capacity()
    }
}
